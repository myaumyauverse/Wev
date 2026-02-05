use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn init_workspace(root: &str) -> Result<()> {
    let wev_dir = Path::new(root).join(".wev");

    if wev_dir.exists() {
        println!("Wev already initialized in {}", root);
        return Ok(());
    }

    fs::create_dir(&wev_dir)?;
    fs::write(wev_dir.join("events.log"), "")?;

    println!("Initialized Wev in {}", root);
    Ok(())
}
