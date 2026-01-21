# ask

**AI-powered CLI assistant — modern Unix meets AI**

[![CI](https://github.com/askterminal-dev/ask/actions/workflows/ci.yml/badge.svg)](https://github.com/askterminal-dev/ask/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`ask` is a fast, ergonomic command-line tool that bridges traditional Unix workflows with AI capabilities. Get instant command suggestions, query system information naturally, and tap into AI for complex questions — all from your terminal.

## Features

- **Multi-provider support** — Works with Anthropic, OpenAI, Gemini, Ollama, Perplexity, Groq, Mistral, Cohere, Together, or custom endpoints
- **Command suggestions** — Get the right command without leaving the terminal
- **Natural language system queries** — Ask "what is using port 8080" instead of remembering `lsof` flags
- **Streaming AI responses** — Real-time output, no waiting for complete responses
- **Zero friction** — No quotes needed for queries, works with pipes, respects `NO_COLOR`
- **Fast** — Native Rust binary, ~3MB, starts instantly
- **Secure** — Security warnings for untrusted providers, no accidental data leaks

## Installation

### From crates.io (recommended)

```bash
cargo install ask-cmd
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/askterminal-dev/ask/releases).

### From source

```bash
git clone https://github.com/askterminal-dev/ask.git
cd ask
cargo install --path .
```

## Quick Start

```bash
# Set your API key (Anthropic is the default provider)
export ANTHROPIC_API_KEY="sk-ant-..."

# Or save it to config (secure input, not saved in shell history)
ask config api_key

# Start asking
ask how do I find large files
ask what is using port 3000
ask "explain the difference between threads and processes"
```

### Using Other Providers

```bash
# Use OpenAI
ask config provider=openai
export OPENAI_API_KEY="sk-..."

# Use local Ollama (no API key needed)
ask config provider=ollama

# Use Groq for fast inference
ask config provider=groq
export GROQ_API_KEY="gsk_..."
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
ask config api_key                        # Set API key (secure input)
ask config provider=openai                # Switch provider
ask config model=gpt-4o                   # Set model
ask config max_tokens=2048                # Set max tokens
```

### Supported Providers

| Provider | Default Model | API Key Env Var |
|----------|---------------|-----------------|
| `anthropic` (default) | claude-sonnet-4-20250514 | `ANTHROPIC_API_KEY` |
| `openai` | gpt-4o | `OPENAI_API_KEY` |
| `gemini` | gemini-1.5-flash | `GEMINI_API_KEY` |
| `ollama` | llama3.2 | (none required) |
| `perplexity` | llama-3.1-sonar-small-128k-online | `PERPLEXITY_API_KEY` |
| `groq` | llama-3.3-70b-versatile | `GROQ_API_KEY` |
| `mistral` | mistral-small-latest | `MISTRAL_API_KEY` |
| `cohere` | command-r-plus | `COHERE_API_KEY` |
| `together` | meta-llama/Llama-3.3-70B-Instruct-Turbo | `TOGETHER_API_KEY` |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `ASK_PROVIDER` | Override provider (e.g., `openai`, `ollama`) |
| `ASK_API_URL` | Override API endpoint URL |
| `ASK_MODEL` | Override model |
| `ASK_API_KEY` | Fallback API key for any provider |
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `OPENAI_API_KEY` | OpenAI API key |
| `GEMINI_API_KEY` | Google Gemini API key |
| `ASK_NO_COLOR` | Disable colored output |
| `NO_COLOR` | Disable colored output (standard) |

### Custom Providers

You can use custom API endpoints, but `ask` will show a security warning:

```bash
$ ask config api_url=https://my-custom-llm.example.com/v1/chat

SECURITY WARNING

You are configuring a custom api_url that is not in the trusted allowlist:
  api_url = https://my-custom-llm.example.com/v1/chat

This configuration will send your queries and API key to this destination.
Only proceed if you trust this api_url completely.

Type 'yes' to confirm:
```

This prevents accidental data leaks to untrusted endpoints.

## Why ask?

| Instead of... | Use... |
|---------------|--------|
| Googling "how to tar a directory" | `ask how do I compress a folder` |
| `man lsof`, scrolling for flags | `ask what is using port 8080` |
| Opening ChatGPT in browser | `ask "explain kubernetes pods"` |
| `df -h` (if you remember it) | `ask system disk` |

`ask` keeps you in the terminal and in flow.

## Requirements

- macOS or Linux
- Rust 1.70+ (for building from source)
- API key from a supported provider, or local Ollama (only needed for AI queries — system info, command suggestions, and explanations work without any API key)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).

## License

[MIT](LICENSE)

---

Built with Rust. Inspired by the wave of modern Unix tools like [ripgrep](https://github.com/BurntSushi/ripgrep), [fd](https://github.com/sharkdp/fd), [bat](https://github.com/sharkdp/bat), and [eza](https://github.com/eza-community/eza).
