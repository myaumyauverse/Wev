README.md

# wev
### Write Event Versioning

> Event‑driven version control for semantic file history.
> Lightweight. Experimental. Built in Rust.

---

## What is wev?

`wev` is an experimental version control system that records **semantic file operations** instead of full file snapshots.

Where traditional systems store file states, `wev` stores **intent**:

- create
- append
- overwrite
- delete

This enables a different way to think about history — one built from events, not diffs.

---

## Why?

Modern version control systems are powerful, but snapshot-based.

`wev` explores a different model:

- Event-based versioning
- Append-only logs
- Graph-based state reconstruction
- Lightweight architecture
- Transparent storage

This project is both:

- 🔬 A systems experiment  
- 🚀 A foundation for a future event-driven VCS  

---

## Current Status

Active development.

Core components implemented:

- Workspace initialization
- Recursive file watching
- Semantic event classification
- Append-only event logging
- State engine (StateMesh)
- Event history traversal
- CLI interface (clap-based)

Not production-ready. Research-grade.

---

## Installation

### Requirements

- Rust (stable)
- Cargo

Check:

```bash
rustc --version
cargo --version

If needed:

curl https://sh.rustup.rs -sSf | sh

Install from Source

git clone https://github.com/myaumyauverse/Wev.git
cd Wev
cargo install --path .

Ensure Cargo binaries are in your PATH:

export PATH="$HOME/.cargo/bin:$PATH"

Verify:

wev --help

Quick Start
1. Initialize a workspace

wev init <path>

Example:

wev init demo

Creates:

demo/
└── .wev/
    ├── events.log
    └── state.json

2. Watch a folder

wev watch <path>

Example:

wev watch demo

Example output:

TIME     EVENT      FILE
--------------------------------
12:40:15 CREATE     a.txt
12:40:33 APPEND     a.txt
12:40:40 OVERWRITE  a.txt
12:40:44 DELETE     a.txt

wev classifies filesystem noise into meaningful semantic operations.
3. View history

wev log

Example:

* delete     a.txt
* overwrite  a.txt
* append     a.txt
* create     a.txt

The state engine reconstructs history by traversing the event graph.
Architecture
Event Flow

    Filesystem watcher (notify)

    Semantic classification layer

    Append-only event log

    StateMesh engine (graph builder)

    History traversal from HEAD

Storage Model

    .wev/events.log → append-only JSON event stream

    .wev/state.json → graph representation of state

This design allows:

    Deterministic reconstruction

    Auditable history

    Minimal storage overhead

    Potential branching via graph evolution

Project Structure

src/
├── cli.rs        # CLI interface
├── init.rs       # Workspace initialization
├── watcher.rs    # Filesystem event engine
├── state.rs      # StateMesh graph engine
├── log.rs        # History traversal
└── main.rs       # Entry point

Design Philosophy

Git stores snapshots.

wev stores intent.

Instead of asking:

    What did the file look like?

wev asks:

    What happened to the file?

Roadmap

Short Term:

    Improve CLI UX

    Add colored output

    Improve error handling

    Add status command

    Add tests

Mid Term:

    Branching support

    Checkout support

    State diffing

    Event compression

Long Term:

    Distributed event sync

    Remote support

    Visual graph viewer

    Event-driven CI integration

Contributing

Contributions are welcome.

If you're exploring:

    Event sourcing

    Filesystem systems

    Lightweight VCS design

    Rust systems programming

Open an issue or submit a PR.
License

MIT (or update accordingly)
Author

Built by Satvik Porwal
Experimental systems engineering project.