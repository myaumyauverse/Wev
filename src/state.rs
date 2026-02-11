use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u64,
    pub time: String,
    pub event: String,
    pub file: String,
    pub parent: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub head: Option<u64>,
    pub nodes: Vec<Node>,
}

const STATE_PATH: &str = ".wev/state.json";

/// Record a new event into StateMesh
pub fn record_event(time: &str, event: &str, file: &str) -> Result<()> {
    ensure_state_exists()?;

    let data = fs::read_to_string(STATE_PATH)?;
    let mut state: State = serde_json::from_str(&data)?;

    let new_id = state.nodes.len() as u64 + 1;

    let node = Node {
        id: new_id,
        time: time.to_string(),
        event: event.to_string(),
        file: file.to_string(),
        parent: state.head,
    };

    state.nodes.push(node);
    state.head = Some(new_id);

    let updated = serde_json::to_string_pretty(&state)?;
    fs::write(STATE_PATH, updated)?;

    Ok(())
}

/// Ensure .wev/state.json exists
fn ensure_state_exists() -> Result<()> {
    if !Path::new(".wev").exists() {
        fs::create_dir(".wev")?;
    }

    if !Path::new(STATE_PATH).exists() {
        let initial = State {
            head: None,
            nodes: Vec::new(),
        };

        let json = serde_json::to_string_pretty(&initial)?;
        fs::write(STATE_PATH, json)?;
    }

    Ok(())
}
