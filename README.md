# ask

A multi-purpose CLI query tool bridging Unix and AI.

## Installation

### From source
```bash
cargo install --path .
```

### Pre-built binaries
Download from [Releases](../../releases).

## Usage

```bash
ask <query>              # Direct query
ask -i                   # Interactive mode
echo "query" | ask       # Pipe mode
```

## Examples

### Command suggestions
```bash
ask how do I compress a folder
# Output:
# tar -czvf archive.tar.gz folder/
# zip -r archive.zip folder/

ask how to find files
# Output:
# find /path -name "pattern"
# locate filename
```

### System information
```bash
ask system disk          # Show disk usage
ask system memory        # Show memory usage
ask system cpu           # Show CPU info
ask system ports         # Show network ports
ask system uptime        # Show system uptime
```

### Natural language system queries
```bash
ask what is using port 8080
```

### Command explanation
```bash
ask explain grep         # Show grep --help or man page
ask explain ls
```

### AI queries (requires API key)
```bash
ask "what is the capital of France?"
ask "write a bash script to backup my home directory"
```

### Interactive prompts (for scripts)
```bash
ask prompt "Continue with installation?"
# Returns exit code 0 for yes, 1 for no
```

## Configuration

Config file: `~/.config/ask/config.json`

```bash
ask config show              # Show current config
ask config path              # Show config file path
ask config api_key=sk-ant-...  # Set API key
ask config model=claude-sonnet-4-20250514
ask config max_tokens=2048
```

### Environment variables
- `ANTHROPIC_API_KEY` - API key for AI queries
- `ASK_MODEL` - Override model
- `ASK_NO_COLOR` or `NO_COLOR` - Disable colored output

## Building

```bash
# Debug build
cargo build

# Release build (optimized, smaller binary)
cargo build --release

# Run tests
cargo test
```

## License

MIT
