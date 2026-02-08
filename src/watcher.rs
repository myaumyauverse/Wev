use anyhow::Result;
use chrono::Local;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::{metadata, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{mpsc::channel, Mutex};

lazy_static::lazy_static! {
    // Track last known file sizes
    static ref FILE_SIZES: Mutex<HashMap<PathBuf, u64>> = Mutex::new(HashMap::new());
    // Track files just created to suppress first write noise
    static ref JUST_CREATED: Mutex<HashMap<PathBuf, bool>> = Mutex::new(HashMap::new());
}

/// Watch a folder and log semantic file events
pub fn watch_folder(watch_path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;

    println!("Watching folder: {}\n", watch_path);
    println!("{:<8} {:<10} {}", "TIME", "EVENT", "FILE");
    println!("--------------------------------");

    // Workspace root (where .wev lives)
    let workspace = std::env::current_dir()?;

    loop {
        match rx.recv() {
            Ok(Ok(event)) => handle_event(&workspace, event)?,
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(e) => eprintln!("channel error: {:?}", e),
        }
    }
}

fn handle_event(workspace: &PathBuf, event: Event) -> Result<()> {
    let time = Local::now().format("%H:%M:%S").to_string();

    for path in event.paths {
        if path.is_dir() {
            continue;
        }

        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>")
            .to_string();

        let event_type = classify_event(&event.kind, &path);
        if event_type == "ignore" {
            continue;
        }

        // UX-friendly CLI output
        println!("{:<8} {:<10} {}", time, event_type.to_uppercase(), file);

        // Append-only event log (StateMesh will read this later)
        log_event(workspace, &time, &event_type, &file)?;
    }

    Ok(())
}

fn classify_event(kind: &EventKind, path: &Path) -> &'static str {
    match kind {
        EventKind::Create(_) => {
            if let Ok(meta) = metadata(path) {
                FILE_SIZES
                    .lock()
                    .unwrap()
                    .insert(path.to_path_buf(), meta.len());

                JUST_CREATED
                    .lock()
                    .unwrap()
                    .insert(path.to_path_buf(), true);
            }
            "create"
        }

        EventKind::Modify(_) => {
            let mut sizes = FILE_SIZES.lock().unwrap();
            let mut created = JUST_CREATED.lock().unwrap();

            let old = sizes.get(path).copied().unwrap_or(0);
            let new = metadata(path).map(|m| m.len()).unwrap_or(old);

            sizes.insert(path.to_path_buf(), new);

            // Ignore first write after create
            if created.remove(path).is_some() {
                "ignore"
            } else if new > old {
                "append"
            } else {
                "overwrite"
            }
        }

        EventKind::Remove(_) => {
            FILE_SIZES.lock().unwrap().remove(path);
            JUST_CREATED.lock().unwrap().remove(path);
            "delete"
        }

        _ => "ignore",
    }
}

fn log_event(workspace: &PathBuf, time: &str, event: &str, file: &str) -> Result<()> {
    let log_path = workspace.join(".wev").join("events.log");

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let line = format!(
        "{{\"time\":\"{}\",\"event\":\"{}\",\"file\":\"{}\"}}\n",
        time, event, file
    );

    log.write_all(line.as_bytes())?;
    Ok(())
}
