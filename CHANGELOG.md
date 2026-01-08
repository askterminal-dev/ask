# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-08

### Added
- Initial Rust port from Python implementation
- Intent detection system with regex-based routing
  - `config` - View and modify configuration
  - `prompt` - Interactive y/n prompts for scripts
  - `system` - System info (disk, memory, cpu, ports, uptime, os)
  - `howto` - Command suggestions with pattern matching
  - `explain` - Command help via --help or man pages
  - `ai` - AI queries via Anthropic API
- Streaming AI responses with real-time output
- Natural language system queries ("what is using port 8080")
- Configuration management
  - XDG-compliant config path (`~/.config/ask/config.json`)
  - Legacy `.askrc` support
  - Environment variable overrides (ANTHROPIC_API_KEY, ASK_MODEL, ASK_NO_COLOR)
- Multiple input modes
  - Command-line arguments (unquoted supported)
  - Interactive mode (`-i`)
  - Pipe mode (stdin)
- Cross-platform support (macOS and Linux)
- 27 tests (12 unit + 15 integration)
- CI/CD workflow for build, test, and release
- Release profile with LTO (~2.9MB binary)

### Notes
- Full feature parity with Python version
- Reads existing Python config files
- ~10x faster startup than Python version

[Unreleased]: https://github.com/USER/ask-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/USER/ask-rs/releases/tag/v0.1.0
