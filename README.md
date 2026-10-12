# tuido
Tuido is a lightweight terminal-based to-do application that helps you organize tasks into lists with custom columns. Its core is written in Rust, and because it does not require a graphical interface, it can run smoothly on a wide range of hardware.

## Project Status
The project's core is under development, and the terminal interface has not been implemented yet.

## Features
### Minimal Viable Product:
- [x] Creating, deleting, and renaming lists
- [ ] Creating, removing, and renaming columns (e.g. backlog, in progress, etc.)
- [ ] Creating, deleting, and renaming cards in lists
- [ ] Moving cards between columns
- [ ] Saving lists in separate JSON files after every change
- [ ] TUI using Ratatui
### Full Release:
- [ ] Importing and exporting lists to and from the user's desktop
- [ ] Adding, deleting, and renaming card properties (e.g. deadline date, priority, etc.)
- [ ] Sorting cards by their properties
- [ ] Manual Git synchronization (commit, push, and pull)

> Checked features are fully implemented.

## Requirements
- Rust 1.85 or newer,
- Cargo.

## Building
Clone the repository:
```bash
git clone https://github.com/naxemis/tuido.git
cd tuido
```

Build the workspace:
```bash
cargo build
```

## Running
Run the terminal application with:
```bash
cargo run -p tui
```

## Testing
Run all tests in this workspace:
```bash
cargo test --workspace --all-targets --all-features
```

Run tests with additional output:
```bash
cargo test --workspace -- --nocapture
```

## Development
Before opening a pull request, make sure that formatting, test and Clippy checks pass.

To test it you can use:
To run the check script through Bash:

```bash
bash ./.github/scripts/check.sh
```

## Contributing
1. Create a feature branch.
2. Implement the change and add or update tests.
3. Run formatting, test and Clippy through the `check.sh` script.
4. Update the documentation if the public behavior changes.
5. Open a pull request describing the change and how it was tested.