### 🧭 Overview

**Man-TUI** is a **Rust-based terminal application** built using the **Ratatui** framework.  
It provides an interactive, high-performance **TUI (Text User Interface)** for browsing, searching, and reading Linux man pages efficiently — with a clean, modern interface similar to tools like `broot` or `tldr`.

---
### 🎯 Objectives

- Provide a **fast, offline, interactive viewer** for Linux man pages.    
- Enable **fuzzy search**, **keyboard navigation**, and **scrollable content**.
- Offer a **split-pane layout**: left for searching and selecting commands, right for viewing the selected man page.
- Maintain **zero external dependencies** (works on any Linux system with `man` installed)
---
### 🧱 Architecture

#### **Tech Stack**
- **Language:** Rust
- **UI Framework:** [`ratatui`](https://ratatui.rs/)
- **Terminal Backend:** `crossterm`
- **Search Support:** [`skim`](https://github.com/lotabout/skim) or internal fuzzy matcher (like `fuzzy_matcher`)
- **Error Handling:** `anyhow`
#### **App Layers**

1. **Input & Event Layer** – handles keyboard/mouse input and navigation.
2. **Search Layer** – manages list of available man pages, fuzzy search, and filtering.
3. **Viewer Layer** – renders formatted man-page content with scrolling and highlighting.
4. **System Layer** – invokes `man` command, processes output via `col -b` to clean formatting.
---
### 🧩 Core Components
#### **1. App State (`App`)**
Holds:
- `search_query: String` – user’s current search text
- `filtered_commands: Vec<String>` – filtered man-page names
- `selected_index: usize` – which command is selected in list
- `man_content: String` – current loaded man-page content
- `scroll_offset: usize` – vertical scroll position in viewer
- `focus: FocusState` – enum `{ Search, List, Viewer }`
#### **2. UI Layout**
```mathematica
┌──────────────────────────────────────────────┐
│ Search: [__________grep___________]          │
├──────────────────────────────────────────────┤
│ Left Pane: Command List      │ Right Pane: Man Viewer      │
│───────────────────────────────┼────────────────────────────│
│ grep                         │ G R E P(1)                 │
│ ls                            │ NAME                       │
│ awk                           │    grep - print lines...    │
│ sed                           │                            │
│ ...                           │ [Scrollable content]        │
│                               │                            │
├──────────────────────────────────────────────┤
│ [↑↓] Navigate  [Enter] Open  [Tab] Switch  [q] Quit        │
└──────────────────────────────────────────────┘
```
#### **3. Event Handling**

|Key|Action|
|---|---|
|`↑ / ↓`|Navigate list or scroll content|
|`Enter`|Load selected man page|
|`Tab`|Toggle focus between panes|
|`/`|Jump to search bar|
|`PgUp / PgDn`|Fast scroll viewer|
|`q`|Quit application|

---
### ⚙️ Functional Requirements

| ID  | Feature           | Description                                                                                 |
| --- | ----------------- | ------------------------------------------------------------------------------------------- |
| F1  | Command Discovery | Scan `/usr/share/man/man*/*.gz` to build list of available man pages (cached on first run). |
| F2  | Fuzzy Search      | Filter available commands as user types in the search bar.                                  |
| F3  | Page Loading      | Run `man <cmd>                                                                              |
| F4  | Scrollable Viewer | Use `ratatui::widgets::Paragraph` to display multi-line content with scroll offset.         |
| F5  | Split Layout      | Use `ratatui::layout::Layout` with two main vertical panes.                                 |
| F6  | Persistent State  | Optional — store last opened command and scroll offset in config file.                      |
| F7  | Graceful Exit     | Restore terminal state on exit (disable raw mode, leave alt screen).                        |

---
### ⚡ Non-Functional Requirements

|Type|Requirement|
|---|---|
|Performance|Open and render man pages in under 200ms|
|Compatibility|Works on Linux and macOS terminals|
|Memory|Under 50MB runtime|
|Stability|Handles missing or broken man pages gracefully|
|Usability|Intuitive keybindings and layout|

---
### 🧰 Implementation Notes
##### Fetching Man Pages
```rust
use std::process::Command;

fn fetch_man_page(cmd: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("man {} | col -b", cmd))
        .output();
    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        _ => format!("Man page for '{}' not found.", cmd),
    }
}
```
##### Listing Available Commands
```rust
use std::fs;

fn get_available_commands() -> Vec<String> {
    let mut cmds = Vec::new();
    if let Ok(entries) = fs::read_dir("/usr/share/man/man1") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(cmd) = name.split('.').next() {
                    cmds.push(cmd.to_string());
                }
            }
        }
    }
    cmds.sort();
    cmds
}
```

---

### 🖥️ UI Design Details
#### **Layout**
- Top bar → search input
- Middle → split view (list + viewer)
- Bottom → status/help bar
#### **Widgets**
- `Paragraph` for text and search input
- `List` for command list
- `Block` with titles for sections
- `Wrap { trim: false }` for text wrapping in viewer
#### **Color Theme**
- Background: black
- Search box: cyan border
- List selection: yellow highlight
- Viewer text: white
- Footer: dim Gray
---
### 🧠 App Flow (Pseudocode)
```rust
AppState {
  init: load all man page names
  focus = Search
  loop {
    draw layout
    handle input:
      if q => exit
      if Tab => switch focus
      if Enter => load man page for selected command
      if ↑↓ => navigate list or scroll viewer
      if text input => update search filter
  }
}
```

---
### 🧩 Future Enhancements
- 📚 Section selection (man 1, 2, 3, …)
- 🔍 In-page search (`/` then highlight matches)
- 🧠 Cached previews for faster reload
- 🌓 Dark/light theme toggle
- 💾 Persistent history and favorites
---
### 🧑‍💻 Suggested Crates

```toml
[dependencies]
ratatui = "0.28"
crossterm = "0.27"
anyhow = "1.0"
fuzzy-matcher = "0.3"
```
---
### 📦 Directory Structure

```bash
man-tui/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── app.rs        # Main state and event loop
    ├── ui.rs         # Ratatui layout and widgets
    ├── man.rs        # Functions to fetch man pages
    ├── search.rs     # Fuzzy matching
    └── config.rs     # Optional: persistence
```
---
### 🧩 Example Build Command
```bash
cargo build --release
./target/release/man-tui
```
---
### 🧠 Developer Notes
- Must gracefully exit on `q` — restore terminal mode with `LeaveAlternateScreen`.
- Should debounce search updates for performance (e.g., wait 150ms after typing).
- Support resizing (use `terminal.resize()` events).
- Keep logic modular — separate **UI rendering**, **state**, and **I/O**.