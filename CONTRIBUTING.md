# Contributing to man-tui

Thanks for checking out `man-tui`. This project is a fast, keyboard-first Rust TUI for browsing Linux and macOS man pages, packaged as both a Cargo crate and a PyPI-installable command.

Contributions are welcome at every size: bug reports, docs fixes, design notes, small UI polish, tests, platform compatibility improvements, and larger feature experiments. The spirit here is open, rapid, iterative development: ship useful slices, learn from real terminal use, and make it easy for the next person to keep moving.

## Current project shape

- Language: Rust 2021
- TUI stack: Ratatui and crossterm
- Package name and binary: `man-tui`
- Repository: `github.com/aksharahegde/manrust`
- License: MIT
- Python packaging: maturin with `bindings = "bin"`

The app currently includes command discovery, fuzzy command search, section selection, in-page search, favorites, command history, dark/light themes, and runtime man-page caching.

## Prerequisites

Install these before working locally:

- Rust stable, via `rustup`
- `cargo`
- The `man` command on your `PATH`
- The `col` command on your `PATH`
- Optional: Python 3.8+ and `maturin` if you want to test PyPI packaging

On macOS and most Linux distributions, `man` and `col` are usually already available. If the app opens but pages fail to load, check those commands first.

## Set up a local dev environment

Fork the repo, then clone your fork:

```bash
git clone https://github.com/YOUR-USERNAME/manrust.git
cd manrust
```

Build the project:

```bash
cargo build
```

Run the TUI in development:

```bash
cargo run
```

Run the release binary:

```bash
cargo build --release
./target/release/man-tui
```

Optional PyPI packaging check:

```bash
python -m pip install maturin
maturin build
```

## Development checks

Please run these before opening a pull request:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

If one of these fails because of a platform-specific issue, mention your OS, terminal, shell, and the exact error in the PR.

## Coding style

Keep changes small, readable, and easy to review.

- Follow standard Rust formatting with `cargo fmt`.
- Prefer clear names over clever shortcuts.
- Keep terminal state safe: raw mode, alternate screen, and mouse capture must be restored on exit.
- Keep UI rendering in `src/ui.rs` where possible.
- Keep app state and behavior in `src/app.rs`.
- Keep man-page discovery and command execution in `src/man.rs`.
- Keep fuzzy filtering in `src/search.rs`.
- Keep config loading and persistence in `src/config.rs`.
- Use `anyhow::Result` and contextual errors for fallible I/O.
- Avoid panics in normal user flows.
- Handle empty lists, missing man pages, missing commands, and unusual terminals gracefully.

When adding a feature, try to keep it useful in a minimal first version. A focused PR that opens a path for iteration is better than a huge PR that is hard to land.

## Good first contributions

These are especially approachable:

- Improve README examples or keybinding docs.
- Add tests around fuzzy filtering or config behavior.
- Improve errors when `man` or `col` is missing.
- Make command discovery work with more man-page locations.
- Polish status/help text in the footer.
- Add small accessibility improvements such as clearer focus labels or theme contrast.

## Submitting issues

Before opening an issue, search existing issues to avoid duplicates.

For bugs, include:

- What you expected to happen
- What actually happened
- Steps to reproduce
- OS and version
- Terminal app and shell
- `man-tui` version
- Any relevant command output

For feature ideas, include:

- The workflow you want to improve
- A rough example of how the feature might feel in the TUI
- Whether you would like to work on it

Short, practical issues are welcome. You do not need a perfect proposal before starting the conversation.

## Pull request flow

1. Open or comment on an issue if the change is more than a small fix.
2. Create a focused branch from `main`.
3. Make the smallest useful change.
4. Run the development checks.
5. Open a PR with a clear summary and screenshots or terminal recordings when UI behavior changes.

Helpful PR descriptions include:

- What changed
- Why it changed
- How you tested it
- Any tradeoffs or follow-up ideas

If your PR touches terminal behavior, please mention the terminal you tested in. TUI bugs can be surprisingly environment-specific.

## Testing notes

Automated tests are still a good area for growth. When adding logic that can be tested without a terminal, prefer a small unit test.

Manual testing is still valuable for TUI changes. At minimum, check:

- Launch and quit restore the terminal cleanly.
- Search focus accepts typing and backspace.
- List navigation works.
- Opening a man page works.
- Viewer scrolling works.
- `/`, `n`, and `N` work for in-page search when applicable.
- `s` opens section selection when sections are available.
- `t` toggles theme.
- `f` toggles favorite for the current command.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Community tone

Assume curiosity and good intent. Ask direct questions, share rough ideas early, and leave useful notes for the next contributor. `man-tui` should be a small tool that grows through public iteration: practical, fast, open, and easy to improve.
