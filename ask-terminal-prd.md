# Ask Terminal - Product Requirements Document

## Overview

Ask Terminal is a beginner-friendly terminal application that helps non-technical users feel less intimidated by the command line. It wraps a real terminal with contextual explanations, safety indicators, and guided suggestions.

The app consists of two components:
1. **`ask`** — A standalone CLI tool that can be used in any terminal
2. **Ask Terminal** — An Electron-based GUI that wraps the terminal with helper UI

Website: askterminal.dev

## Target Users

- Non-developers who need to use the command line occasionally
- Beginners learning terminal basics
- Users who copy/paste commands from tutorials but don't understand them
- Anyone intimidated by the blank terminal prompt

NOT targeting: experienced developers or power users (they can use `ask` CLI directly)

## Design Philosophy

- The terminal is the star — dark, centered, real
- Helper UI are "wings" that attach to the terminal, not co-equal panels
- Minimal color palette — white/gray wings, color used only for highlighting
- "Review before you run" flow — commands are staged before execution
- Progressive confidence — help fades as users gain experience

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Electron App                         │
├─────────────────────────────────────────────────────────┤
│  React UI (Renderer Process)                            │
│  ├── Terminal Region (xterm.js) — display only          │
│  ├── Draft Panel — command input + explanation          │
│  └── Help Panel — suggestions + natural language input  │
├─────────────────────────────────────────────────────────┤
│  Main Process                                           │
│  ├── node-pty — spawns real shell (zsh/bash)            │
│  ├── Command Parser — generates explanations            │
│  │   ├── Static command database                        │
│  │   ├── Man page parser                                │
│  │   └── LLM fallback (Claude API)                      │
│  └── Output Analyzer — extracts context, suggestions    │
└─────────────────────────────────────────────────────────┘
```

## UI Layout

The interface has three regions:

### 1. Terminal Region (Top Left, 2/3 width)
- Dark background (#111827), real terminal aesthetics
- Displays output from xterm.js
- Shows command history with colored prompt (green arrow)
- Read-only for beginners — all input goes through Draft Panel
- Standard terminal header with traffic light buttons

### 2. Draft Panel (Bottom Left, below Terminal)
- White/light background
- Shows the current command being composed
- Displays:
  - Command with `$` prompt
  - Run button
  - Plain English explanation of what the command does
  - Parts breakdown (e.g., `ls` = list files, `-la` = all + details)
  - Safety indicator (green/amber/red dot with explanation)
- This is where users "stage" commands before running them

### 3. Help Panel (Right side, 1/3 width, full height)
- White/light background
- Header: "What's next?"
- Sections:
  - Suggested commands (clickable, populate Draft Panel)
  - Natural language prompts ("What's in Projects?", "Open notes.txt")
- Footer: Tip of the day / keyboard shortcut hints

## Core Features

### Command Explanation
- Real-time parsing as user types
- Plain English translation of command intent
- Breakdown of each part (command, flags, arguments)
- Safety classification:
  - **Safe** (green): read-only commands (ls, pwd, cat, echo)
  - **Moderate** (amber): commands that modify files (cp, mv, mkdir)
  - **Dangerous** (red): destructive commands (rm, sudo, chmod)

### Command Input Methods
1. Type directly in Draft Panel
2. Click a suggestion from Help Panel
3. Type natural language, press Tab to convert to command
4. (Future) Voice input

### Contextual Suggestions
After each command, Help Panel updates with relevant next steps based on:
- Current directory
- Last command output
- Common workflows

### Integration with `ask` CLI
- Ask Terminal uses the same `ask` engine under the hood
- Users can "graduate" to using `ask` directly in any terminal
- Shared configuration (~/.config/ask/config.json)
- Same Claude API key for both

## Technical Requirements

### Platform Support
- **Phase 1**: macOS (zsh default, bash fallback)
- **Phase 2**: Windows (PowerShell, WSL support)
- **Phase 3**: Linux

### Dependencies
- Electron (latest stable)
- xterm.js — terminal emulation
- node-pty — pseudoterminal for shell spawning
- React — UI framework
- Tailwind CSS — styling

### Shell Integration
- Spawn user's default shell via node-pty
- Support zsh (macOS default) and bash
- Preserve user's shell configuration (.zshrc, .bashrc)
- Handle environment variables correctly

### LLM Integration
- Claude API for:
  - Complex command explanations
  - Natural language to command translation
  - Error message interpretation
- Local fallback for common commands (static database)
- API key stored in config file or environment variable

## Data Flow

### Input Flow
1. User types in Draft Panel (or clicks suggestion)
2. Command Parser analyzes input in real-time
3. Explanation updates in Draft Panel
4. User clicks "Run"
5. Command sent to PTY → Shell executes
6. Output streams to xterm.js

### Output Flow
1. Shell output captured by node-pty
2. Passed to xterm.js for display
3. Also passed to Output Analyzer
4. Analyzer extracts context (cwd, errors, file listings)
5. Help Panel updates with relevant suggestions

## File Structure

```
ask-terminal/
├── package.json
├── electron/
│   ├── main.ts           # Electron main process
│   ├── preload.ts        # Bridge to renderer
│   └── pty.ts            # node-pty wrapper
├── src/
│   ├── App.tsx           # Main React component
│   ├── components/
│   │   ├── Terminal.tsx  # xterm.js wrapper
│   │   ├── DraftPanel.tsx
│   │   └── HelpPanel.tsx
│   ├── services/
│   │   ├── parser.ts     # Command parsing/explanation
│   │   ├── analyzer.ts   # Output analysis
│   │   └── claude.ts     # Claude API client
│   └── data/
│       └── commands.json # Static command database
├── ask                   # CLI tool (Python)
└── README.md
```

## Configuration

Shared config file: `~/.config/ask/config.json`

```json
{
  "api_key": "sk-ant-...",
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 1024,
  "theme": "light",
  "show_safety_indicators": true,
  "beginner_mode": true
}
```

## MVP Scope

### Must Have
- [ ] Terminal display via xterm.js
- [ ] Draft Panel with command input
- [ ] Basic command explanation (static database for top 50 commands)
- [ ] Safety indicators (safe/moderate/dangerous)
- [ ] Run button executes command
- [ ] Help Panel with hardcoded suggestions
- [ ] macOS support

### Should Have
- [ ] Claude API integration for complex explanations
- [ ] Natural language to command translation
- [ ] Contextual suggestions based on output
- [ ] Command history with annotations

### Nice to Have
- [ ] Onboarding tutorial
- [ ] Progress tracking ("You've learned 10 commands!")
- [ ] Windows support
- [ ] Voice input

## Success Metrics

- User can execute their first command within 60 seconds of opening app
- 80% of users understand what a command does before running it
- Users report feeling "less afraid" of the terminal (qualitative)
- Graduation rate: % of users who start using `ask` CLI directly

## Open Questions

1. Should terminal region accept direct input, or only Draft Panel?
2. How aggressive should safety warnings be for `sudo` commands?
3. Should we track command history across sessions for learning insights?
4. Pricing model for Claude API usage?
