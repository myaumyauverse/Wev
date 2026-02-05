use anyhow::Result;
use chrono::Local;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::path::Path;
use std::sync::mpsc::channel;

pub fn watch_folder(path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    println!("Watching folder: {}\n", path);
    println!("{:<8} {:<8} {}", "TIME", "EVENT", "FILE");
    println!("--------------------------------");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => handle_event(event),
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(e) => eprintln!("channel error: {:?}", e),
        }
    }
}

fn handle_event(event: Event) {
    let time = Local::now().format("%H:%M:%S");

    for path in event.paths {
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>");

        let kind = format!("{:?}", event.kind);

        println!("{:<8} {:<8} {}", time, kind, file);
    }
}
