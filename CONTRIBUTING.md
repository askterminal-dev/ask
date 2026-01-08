# Contributing to ask

Thank you for your interest in contributing to ask! This document provides
guidelines and information for contributors.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- An Anthropic API key (for testing AI features)

### Building

```bash
# Clone the repository
git clone https://github.com/USER/ask-rs.git
cd ask-rs

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run clippy lints
cargo clippy
```

### Running Locally

```bash
# Run directly with cargo
cargo run -- "your query here"

# Or use the debug binary
./target/debug/ask "your query here"
```

## Making Changes

### Before You Start

1. Check existing [issues](https://github.com/USER/ask-rs/issues) to see if
   your idea is already being discussed
2. For significant changes, open an issue first to discuss the approach
3. Fork the repository and create a branch for your changes

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Address all clippy warnings (`cargo clippy`)
- Write tests for new functionality
- Keep commits focused and atomic

### Commit Messages

Use clear, descriptive commit messages. We loosely follow conventional commits:

```
feat: add new feature
fix: correct bug in handler
docs: update README
test: add tests for config module
refactor: restructure intent detection
```

### Pull Request Process

1. Ensure all tests pass (`cargo test`)
2. Run `cargo fmt` and `cargo clippy`
3. Update documentation if needed
4. Create a pull request with a clear description of changes
5. Link any related issues

## Project Structure

```
ask-rs/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── config.rs         # Configuration management
│   ├── intent.rs         # Intent detection
│   ├── api.rs            # Anthropic API client
│   ├── error.rs          # Error types
│   └── handlers/         # Intent handlers
│       ├── mod.rs
│       ├── ai.rs         # AI queries
│       ├── config.rs     # Config commands
│       ├── explain.rs    # Command explanation
│       ├── howto.rs      # How-to suggestions
│       ├── prompt.rs     # Y/N prompts
│       └── system.rs     # System info
├── tests/
│   └── integration.rs    # Integration tests
└── Cargo.toml
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_load
```

### Writing Tests

- Unit tests go in the same file as the code (in a `#[cfg(test)]` module)
- Integration tests go in `tests/`
- Use descriptive test names that explain what's being tested

## Areas for Contribution

### Good First Issues

Look for issues labeled `good first issue` for entry points.

### Feature Ideas

See [IDEAS.md](IDEAS.md) for potential features (note: this file is gitignored
and may not exist in your clone).

Current areas of interest:
- Shell completions (bash, zsh, fish)
- Additional system queries
- Performance improvements
- Documentation improvements

## Questions?

Open an issue for questions about contributing. We're happy to help!

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.
