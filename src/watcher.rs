use anyhow::Result;
use chrono::Local;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs::metadata;
use std::path::{Path, PathBuf};
use std::sync::{mpsc::channel, Mutex};

lazy_static::lazy_static! {
    static ref FILE_SIZES: Mutex<HashMap<PathBuf, u64>> = Mutex::new(HashMap::new());
}

pub fn watch_folder(watch_path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;

    println!("Watching folder: {}", watch_path);
    println!("{:<8} {:<10} {}", "TIME", "EVENT", "FILE");
    println!("--------------------------------");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if let Err(e) = handle_event(event) {
                    eprintln!("event error: {:?}", e);
                }
            }
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(e) => eprintln!("channel error: {:?}", e),
        }
    }
}

fn handle_event(event: Event) -> Result<()> {
    let time = Local::now().format("%H:%M:%S").to_string();

    for path in event.paths {
        if path.is_dir() {
            continue;
        }

        let file = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        let event_type = classify_event(&event.kind, &path);

        if event_type == "ignore" {
            continue;
        }

        println!("{:<8} {:<10} {}", time, event_type.to_uppercase(), file);

        crate::state::record_event(&time, event_type, &file)?;
    }

    Ok(())
}

fn classify_event(kind: &EventKind, path: &Path) -> &'static str {
    match kind {
        EventKind::Create(_) => {
            if let Ok(meta) = metadata(path) {
                FILE_SIZES.lock().unwrap().insert(path.to_path_buf(), meta.len());
            }
            "create"
        }

        EventKind::Modify(_) => {
            let mut sizes = FILE_SIZES.lock().unwrap();

            let old = sizes.get(path).copied().unwrap_or(0);
            let new = metadata(path).map(|m| m.len()).unwrap_or(old);

            sizes.insert(path.to_path_buf(), new);

            if new > old {
                "append"
            } else {
                "overwrite"
            }
        }

        EventKind::Remove(_) => {
            FILE_SIZES.lock().unwrap().remove(path);
            "delete"
        }

        _ => "ignore",
    }
}
