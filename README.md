# manrust

Fast, keyboard-first man pages in your terminal.

`manrust` turns the classic `man` workflow into an interactive TUI: search commands with fuzzy matching, open documentation in a split-pane viewer, jump through sections, search inside pages, and keep useful commands close with history and favorites. Launch it with `man-tui`.

It is written in Rust with [Ratatui](https://ratatui.rs/) and `crossterm`, so it feels native in a terminal and exits cleanly back to your shell.

<p>
  <img src="assets/cover.png" alt="manrust screenshot showing command search and man page viewer" width="900">
</p>

## Why manrust?

Plain `man` is powerful, but it is not always quick to explore. `manrust` keeps you in the terminal while giving you a faster browsing loop: type a few characters, pick a command, read the page, search within it, and move on.

## Features

- Interactive split-pane TUI for command search and man-page reading
- Fuzzy command search powered by the Skim matching algorithm
- Man section filtering for sections 1 through 9 when present on your system
- In-page search with next and previous match navigation
- Favorites and recent command history saved between runs
- Dark and light theme toggle
- Runtime man-page caching for fast revisits
- Syntax-aware highlighting for section headers, options, and code-like blocks
- Terminal-safe lifecycle (raw mode + alternate screen are restored on exit)

## Installation

### From PyPI

```bash
pip install man-tui
```

Then launch the TUI:

```bash
man-tui
```

### With uv

```bash
uv tool install man-tui
man-tui
```

### From source

You can also build the Rust binary directly:

```bash
git clone https://github.com/aksharahegde/manrust.git
cd manrust
cargo build --release
./target/release/man-tui
```

For local development:

```bash
cargo run
```

### Requirements

`manrust` reads the man pages already installed on your system. Make sure these commands are available:

- `man`
- `col`

On Linux and macOS, they are usually provided by the base system or standard developer tools.

## Usage

Open the browser:

```bash
man-tui
```

Expected result:

```text
┌Search──────────────────────────────────────────────┐
│ Search: git                                        │
└────────────────────────────────────────────────────┘
┌Commands (Section 1)──────────┐┌Man Page────────────┐
│ git                          ││ GIT(1)             │
│ git-add                      ││ NAME               │
│ git-branch                   ││     git - the...   │
│ git-commit                   ││                    │
└──────────────────────────────┘└────────────────────┘
q quit | tab switch focus | / search | s section | t theme
```

Open a specific workflow:

1. Type part of a command name in the search box.
2. Press Enter, Tab, or Down to move into the command list.
3. Use Up/Down to choose a command.
4. Press Enter to open the man page.
5. Use PageUp/PageDown or Up/Down to scroll.
6. Press `/` in the viewer to search inside the current page.

Key behavior is focus-aware (Search, List, Viewer, Section Select, In-page Search). See the detailed bindings below.

## Keybindings

### Quick reference

| Key | Action |
| --- | --- |
| `q` | Quit |
| `Tab` | Switch focus between search, list, and viewer |
| `Enter` | Open the selected command or confirm the current action |
| `Up` / `Down` | Move through commands or scroll the viewer |
| `PageUp` / `PageDown` | Scroll the viewer by page |
| `/` | Focus command search from the list, or start in-page search from the viewer |
| `n` / `N` | Move to the next or previous in-page search match |
| `s` | Open the man section selector |
| `f` | Favorite the current command |
| `t` | Toggle dark/light theme |
| `l` | Jump back to the command list |

### Focus-aware detail

#### Global-ish

- `q` — quit
- `l` — jump to command list (from search/viewer/in-page search)

#### Search focus

- `Type` — update fuzzy query
- `Backspace` — delete character
- `Enter` / `Tab` / `Down` / `Esc` — move focus forward

#### List focus

- `↑` / `↓` — move selection
- `Enter` — open selected man page
- `Tab` — cycle focus
- `/` — move focus to search input
- `s` — open section selector
- `t` — toggle theme

#### Viewer focus

- `↑` / `↓` — line scroll
- `PgUp` / `PgDn` — page scroll
- `Tab` — cycle focus
- `/` — start in-page search
- `n` / `N` — next / previous in-page match (when search active)
- `f` — toggle favorite for current command
- `t` — toggle theme
- `s` — open section selector

#### In-page search focus

- `Type` — update search query
- `Backspace` — delete character
- `n` / `N` — next / previous match
- `Enter` — return to viewer
- `Esc` — cancel in-page search and return to viewer

#### Section select focus

- `↑` / `↓` — choose section
- `Enter` — confirm section and refresh command list
- `Esc` — cancel and return to list

## Configuration

`manrust` stores lightweight app state as TOML. Depending on your platform and environment, the base config directory follows the system default returned by Rust's `dirs` crate.

Typical paths:

- `$XDG_CONFIG_HOME/man-tui/config.toml`
- or `~/.config/man-tui/config.toml`

Stored fields:

- `history` (recent opened commands, max 100)
- `favorites`
- `last_command`
- `last_section`
- `theme` (`"dark"` or `"light"`)

## How it works

`manrust` discovers installed man pages from section directories such as:

```text
/usr/share/man/man1
/usr/share/man/man2
...
/usr/share/man/man9
```

It extracts `.gz` page basenames as command candidates, deduplicates and sorts them, and applies section filtering when section mode is active.

When you open a page, it runs the system man command and strips terminal formatting for clean rendering:

```bash
man [section] <command> | col -b
```

The output is rendered in the right pane.

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

## Notes and limitations

- Command discovery currently reads `/usr/share/man/...`; systems with custom man paths may need adaptation.
- Runtime cache is in-memory only (cleared when app exits).
- If `man`/`col` are unavailable, page loading will fail.

## Development

Run the app locally:

```bash
cargo run
```

Build an optimized binary:

```bash
cargo build --release
```

Recommended checks before opening a pull request:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Python packaging is handled with [maturin](https://www.maturin.rs/):

```bash
python -m pip install maturin
maturin build --release
```

## Contributing

Contributions are welcome. If you want to improve `manrust`, open an issue for bugs or feature ideas, or send a pull request with a focused change.

Good first areas to explore:

- broader man-page discovery paths
- improved keyboard help inside the TUI
- additional themes
- tests for command discovery, config handling, and search behavior

Please keep changes small, documented, and easy to review.

## License

MIT
