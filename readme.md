# wev

<div align="center">

### Write Event Versioning

**Event-driven version control for semantic file history**

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-experimental-yellow.svg)]()

*Lightweight • Experimental • Built in Rust*

[Installation](#installation) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Roadmap](#roadmap)

</div>

---

## 📖 What is wev?

`wev` is an experimental version control system that records **semantic file operations** instead of full file snapshots.

Where traditional systems store file states, `wev` stores **intent**:

| Operation | Description |
|-----------|-------------|
| `create` | File creation event |
| `append` | Content addition |
| `overwrite` | Content replacement |
| `delete` | File removal |

This enables a different way to think about history — one built from events, not diffs.

---

## 🤔 Why?

Modern version control systems are powerful, but snapshot-based.

`wev` explores a different model:

- ✅ Event-based versioning
- ✅ Append-only logs
- ✅ Graph-based state reconstruction
- ✅ Lightweight architecture
- ✅ Transparent storage

This project is both:

- 🔬 **A systems experiment** — exploring event sourcing in version control
- 🚀 **A foundation** — for a future event-driven VCS

---

## 🎯 Current Status

**Active development** — Not production-ready. Research-grade.

### ✨ Implemented Features

- [x] Workspace initialization
- [x] Recursive file watching
- [x] Semantic event classification
- [x] Append-only event logging
- [x] State engine (StateMesh)
- [x] Event history traversal
- [x] CLI interface (clap-based)

---

## 🚀 Installation

### Prerequisites

| Requirement | Version | Check Command |
|-------------|---------|---------------|
| Rust | stable | `rustc --version` |
| Cargo | latest | `cargo --version` |

If Rust is not installed:
```bash
curl https://sh.rustup.rs -sSf | sh
```

### Install from Source
```bash
# Clone the repository
git clone https://github.com/myaumyauverse/Wev.git
cd Wev

# Install using Cargo
cargo install --path .
```

### Verify Installation

Ensure Cargo binaries are in your `PATH`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Verify `wev` is installed:
```bash
wev --help
```

---

## 🏃 Quick Start

### 1. Initialize a Workspace
```bash
wev init <path>
```

**Example:**
```bash
wev init demo
```

**Creates:**
```
demo/
└── .wev/
    ├── events.log    # Append-only event stream
    └── state.json    # Graph representation
```

### 2. Watch a Folder
```bash
wev watch <path>
```

**Example:**
```bash
wev watch demo
```

**Example Output:**
```
TIME     EVENT      FILE
--------------------------------
12:40:15 CREATE     a.txt
12:40:33 APPEND     a.txt
12:40:40 OVERWRITE  a.txt
12:40:44 DELETE     a.txt
```

> `wev` classifies filesystem noise into meaningful semantic operations.

### 3. View History
```bash
wev log
```

**Example Output:**
```
* delete     a.txt
* overwrite  a.txt
* append     a.txt
* create     a.txt
```

> The state engine reconstructs history by traversing the event graph.

---

## 🏗️ Architecture

### Event Flow
```
Filesystem Watcher (notify) → Semantic Classification → Append-Only Event Log → StateMesh Engine → History Traversal
```

### Storage Model

| File | Purpose | Format |
|------|---------|--------|
| `.wev/events.log` | Append-only event stream | JSON |
| `.wev/state.json` | Graph representation of state | JSON |

**Benefits:**

- ✅ Deterministic reconstruction
- ✅ Auditable history
- ✅ Minimal storage overhead
- ✅ Potential branching via graph evolution

### Project Structure
```
src/
├── cli.rs        # CLI interface
├── init.rs       # Workspace initialization
├── watcher.rs    # Filesystem event engine
├── state.rs      # StateMesh graph engine
├── log.rs        # History traversal
└── main.rs       # Entry point
```

---

## 💭 Design Philosophy

### Git stores snapshots. wev stores intent.

Instead of asking:

> *"What did the file look like?"*

**wev asks:**

> *"What happened to the file?"*

This fundamental shift enables:

- Event sourcing patterns in version control
- Semantic understanding of file evolution
- Lightweight, intention-based history

---

## 🗺️ Roadmap

### 📅 Short Term

- [ ] Improve CLI UX
- [ ] Add colored output
- [ ] Improve error handling
- [ ] Add `status` command
- [ ] Add comprehensive tests

### 📅 Mid Term

- [ ] Branching support
- [ ] Checkout support
- [ ] State diffing
- [ ] Event compression

### 📅 Long Term

- [ ] Distributed event sync
- [ ] Remote support
- [ ] Visual graph viewer
- [ ] Event-driven CI integration

---

## 🤝 Contributing

Contributions are welcome!

If you're interested in exploring:

- 🔍 Event sourcing
- 📁 Filesystem systems
- 🎯 Lightweight VCS design
- 🦀 Rust systems programming

**Feel free to:**

1. Open an issue for discussion
2. Submit a pull request
3. Share your ideas and feedback

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Satvik Porwal**

*Experimental systems engineering project*

<div align="center">

---

**Built with 🦀 Rust**

*If you find this project interesting, consider giving it a ⭐*

</div>