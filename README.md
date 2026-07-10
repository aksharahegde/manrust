# man-tui

A fast, keyboard-first terminal UI for browsing Linux/macOS man pages.

`man-tui` is a Rust application built with [Ratatui](https://ratatui.rs/) and `crossterm`. It gives you a split-pane interface with fuzzy command search on the left and a scrollable man-page viewer on the right.

<p>
  <img src="assets/cover.png" alt="man-tui screenshot showing command search and man page viewer" width="900">
</p>

---

## Features

- **Interactive split view**: search/list pane + man viewer pane.
- **Fuzzy command search** powered by `fuzzy-matcher` (Skim algorithm).
- **Section selection** (man sections 1–9 when available on your system).
- **In-page search** with next/previous match navigation.
- **Command history + favorites** persisted to config.
- **Theme toggle** (dark/light).
- **Man-page content caching** during runtime for fast revisits.
- **Terminal-safe lifecycle** (raw mode + alternate screen are restored on exit).

---

## Installation

### PyPI (recommended)

```bash
pip install man-tui
```

Or with [uv](https://github.com/astral-sh/uv):

```bash
uv tool install man-tui
```

### Prerequisites

- `man` command available on your PATH
- `col` command (used to clean formatting from man output)

### Build from source (Rust)

```bash
cargo build --release
```

Binary output:

```bash
./target/release/man-tui
```

### Run in development

```bash
cargo run
```

---

## Usage

Launch:

```bash
man-tui
```

### Core flow

1. Type in the search box to filter available commands.
2. Move to list, pick a command, press Enter.
3. Read/scroll in the viewer.
4. Use `/` in viewer for in-page search.

---

## Keybindings

> Behavior is focus-aware (Search, List, Viewer, Section Select, In-page Search).

### Global-ish

- `q` — quit
- `l` — jump to command list (from search/viewer/in-page search)

### Search focus

- `Type` — update fuzzy query
- `Backspace` — delete character
- `Enter` / `Tab` / `Down` / `Esc` — move focus forward

### List focus

- `↑` / `↓` — move selection
- `Enter` — open selected man page
- `Tab` — cycle focus
- `/` — move focus to search input
- `s` — open section selector
- `t` — toggle theme

### Viewer focus

- `↑` / `↓` — line scroll
- `PgUp` / `PgDn` — page scroll
- `Tab` — cycle focus
- `/` — start in-page search
- `n` / `N` — next / previous in-page match (when search active)
- `f` — toggle favorite for current command
- `t` — toggle theme
- `s` — open section selector

### In-page search focus

- `Type` — update search query
- `Backspace` — delete character
- `n` / `N` — next / previous match
- `Enter` — return to viewer
- `Esc` — cancel in-page search and return to viewer

### Section select focus

- `↑` / `↓` — choose section
- `Enter` — confirm section and refresh command list
- `Esc` — cancel and return to list

---

## Configuration

The app stores data in a TOML config file:

- Linux/macOS typical path:
  - `$XDG_CONFIG_HOME/man-tui/config.toml`
  - or `~/.config/man-tui/config.toml`

Stored fields:

- `history` (recent opened commands, max 100)
- `favorites`
- `last_command`
- `last_section`
- `theme` (`"dark"` or `"light"`)

---

## How command discovery works

- The app inspects `/usr/share/man/man{section}` directories.
- It extracts `.gz` page basenames as command candidates.
- It deduplicates and sorts commands.
- Selected section filtering is applied when section mode is active.

When opening a page, the app executes:

```bash
man [section] <command> | col -b
```

The output is rendered in the right pane.

---

## Project structure

```text
.
├── Cargo.toml
├── spec.md
└── src
    ├── app.rs       # state machine, focus handling, search debounce, cache
    ├── colorize.rs  # lightweight man content heuristics for styling
    ├── config.rs    # config load/save + history/favorites
    ├── main.rs      # terminal setup + event loop + key routing
    ├── man.rs       # man section discovery + man command execution
    ├── search.rs    # fuzzy filtering
    └── ui.rs        # ratatui rendering and layout
```

---

## Notes and limitations

- Command discovery currently reads `/usr/share/man/...`; systems with custom man paths may need adaptation.
- Runtime cache is in-memory only (cleared when app exits).
- If `man`/`col` are unavailable, page loading will fail.

---

## Development checks

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

