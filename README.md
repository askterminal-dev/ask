# ask

**AI-powered CLI assistant — modern Unix meets AI**

[![CI](https://github.com/USER/ask-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/USER/ask-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`ask` is a fast, ergonomic command-line tool that bridges traditional Unix workflows with AI capabilities. Get instant command suggestions, query system information naturally, and tap into AI for complex questions — all from your terminal.

## Features

- **Command suggestions** — Get the right command without leaving the terminal
- **Natural language system queries** — Ask "what is using port 8080" instead of remembering `lsof` flags
- **Streaming AI responses** — Real-time output from Claude, no waiting for complete responses
- **Zero friction** — No quotes needed for queries, works with pipes, respects `NO_COLOR`
- **Fast** — Native Rust binary, ~3MB, starts instantly

## Installation

### From source (recommended)

```bash
cargo install --path .
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/USER/ask-rs/releases).

### From crates.io (coming soon)

```bash
cargo install ask-cli
```

## Quick Start

```bash
# Set your Anthropic API key (for AI features)
export ANTHROPIC_API_KEY="sk-ant-..."

# Or save it to config
ask config api_key=sk-ant-...

# Start asking
ask how do I find large files
ask what is using port 3000
ask "explain the difference between threads and processes"
```

## Usage

### Command Suggestions

Ask how to do something and get the command:

```bash
$ ask how do I compress a folder
tar -czvf archive.tar.gz folder/
zip -r archive.zip folder/

$ ask how to find files by name
find /path -name "pattern"
locate filename

$ ask how do I see disk usage by directory
du -sh */
du -h --max-depth=1
```

### System Information

Query system resources directly:

```bash
ask system disk          # Disk usage (df -h)
ask system memory        # Memory stats
ask system cpu           # CPU info
ask system ports         # Network ports
ask system uptime        # System uptime
ask system os            # OS version
```

### Natural Language System Queries

Ask about your system in plain English:

```bash
$ ask what is using port 8080
COMMAND   PID   USER   FD   TYPE   DEVICE   NODE NAME
node      1234  user   23u  IPv4   0x...    TCP *:8080 (LISTEN)

$ ask what is using memory
# Shows top memory consumers

$ ask what is using cpu
# Shows top CPU consumers
```

### Command Explanation

Get help for any command:

```bash
ask explain grep         # Shows grep --help or man page
ask explain tar
ask describe rsync
```

### AI Queries

For anything else, ask Claude:

```bash
ask "what is the capital of France"
ask "write a bash one-liner to count lines in all .py files"
ask "explain the difference between TCP and UDP"
```

Responses stream in real-time.

### Interactive Prompts

For use in scripts:

```bash
if ask prompt "Deploy to production?"; then
    ./deploy.sh
fi
```

Returns exit code 0 for yes, 1 for no.

### Input Methods

```bash
ask how do I list files     # Direct (quotes optional)
ask "complex query here"    # Quoted
echo "query" | ask          # Piped
ask -i                      # Interactive mode
```

## Configuration

Config file location: `~/.config/ask/config.json`

```bash
ask config show                           # Show current config
ask config path                           # Show config file path
ask config api_key=sk-ant-...             # Set API key
ask config model=claude-sonnet-4-20250514 # Set model
ask config max_tokens=2048                # Set max tokens
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `ANTHROPIC_API_KEY` | API key for AI queries |
| `ASK_MODEL` | Override default model |
| `ASK_NO_COLOR` | Disable colored output |
| `NO_COLOR` | Disable colored output (standard) |

## Why ask?

| Instead of... | Use... |
|---------------|--------|
| Googling "how to tar a directory" | `ask how do I compress a folder` |
| `man lsof`, scrolling for flags | `ask what is using port 8080` |
| Opening ChatGPT in browser | `ask "explain kubernetes pods"` |
| `df -h` (if you remember it) | `ask system disk` |

`ask` keeps you in the terminal and in flow.

## Building from Source

```bash
git clone https://github.com/USER/ask-rs.git
cd ask-rs

# Debug build
cargo build

# Release build (optimized, ~3MB binary)
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Requirements

- macOS or Linux
- Rust 1.70+ (for building)
- Anthropic API key (for AI features)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).

## License

[MIT](LICENSE)

---

Built with Rust. Inspired by the wave of modern Unix tools like [ripgrep](https://github.com/BurntSushi/ripgrep), [fd](https://github.com/sharkdp/fd), [bat](https://github.com/sharkdp/bat), and [eza](https://github.com/eza-community/eza).
