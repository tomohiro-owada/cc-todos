# cc-todos

A terminal UI viewer for [Claude Code](https://claude.ai/claude-code) TODOs - real-time task tracking in your terminal.

## Features

- Real-time tracking of Claude Code TODO lists
- Progress bar showing completion status
- Categorized view: In Progress / Pending / Completed
- Auto-detects the latest active session
- tmux integration for side-by-side workflow

## Installation

### From crates.io

```bash
cargo install cc-todos
```

### From source

```bash
git clone https://github.com/twada/cc-todos
cd cc-todos
cargo install --path .
```

## Usage

### Basic usage

```bash
# Run in current terminal (tracks latest session)
cc-todos
```

### With tmux (recommended)

```bash
# Inside a tmux session, open TODO viewer in right pane
cc-todos --tmux
```

### One-liner to start Claude Code with TODO viewer

```bash
tmux new -s claude \; \
  split-window -h -p 20 'cc-todos' \; \
  select-pane -t 0 \; \
  send-keys 'claude' Enter
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `r` | Reload |

## Requirements

- [Claude Code](https://claude.ai/claude-code) installed
- tmux (optional, for side-by-side view)

## How it works

Claude Code stores TODO lists in `~/.claude/todos/` as JSON files. This tool watches that directory and displays the latest session's TODOs in real-time.

## License

MIT
