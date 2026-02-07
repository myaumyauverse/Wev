use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Node {
    id: u64,
    time: String,
    event: String,
    file: String,
    parent: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct State {
    head: Option<u64>,
    nodes: Vec<Node>,
}

pub fn print_log() -> Result<()> {
    let data = fs::read_to_string(".wev/state.json")?;
    let state: State = serde_json::from_str(&data)?;

    let mut current = state.head;

    while let Some(id) = current {
        if let Some(node) = state.nodes.iter().find(|n| n.id == id) {
            println!("* {:<10} {}", node.event, node.file);
            current = node.parent;
        } else {
            break;
        }
    }

    Ok(())
}
