# template-repository
Description placeholder

## Project Status
Project status placeholder

## Features
### Minimal Viable Product
- [ ] Feature placeholder
### Full Release
- [ ] Feature placeholder

## Requirements
- Requirement placeholder

## Building
Clone the repository:
```bash
git clone https://github.com/naxemis/placeholder_repository.git
cd placeholder_repository
```

Build the workspace:
```bash
cargo build
```

## Running
Run the application with:
```bash
cargo run -p placeholer_application
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
Before opening a pull request, make sure that formatting, tests and Clippy checks pass.

To test it locally, run:
```bash
bash ./.github/scripts/check.sh
```

To automatically format the code and apply available Clippy fixes, run:
```bash
bash ./.github/scripts/fix.sh
```

## Contributing
1. Create a feature branch.
2. Implement the change and add or update tests.
3. Run the `check.sh` script to verify formatting, tests, and Clippy checks.
4. Update the documentation if the public behavior changes.
5. Open a pull request describing the change and how it was tested.