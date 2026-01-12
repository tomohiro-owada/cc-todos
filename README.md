# cc-todos

A terminal UI viewer for [Claude Code](https://claude.ai/claude-code) TODOs - real-time task tracking in your terminal.

<img width="1570" alt="cc-todos screenshot" src="https://github.com/user-attachments/assets/16f8a52f-697d-4e02-8c66-359036a5593e" />

## Features

- Real-time tracking of Claude Code TODO lists
- Progress bar showing completion status
- Categorized view: In Progress / Pending / Completed
- **Project-aware**: Tracks TODOs for the specific working directory
- tmux integration for side-by-side workflow

## Installation

### From crates.io

```bash
cargo install cc-todos --locked
```

### From source

```bash
git clone https://github.com/tomohiro-owada/cc-todos
cd cc-todos
cargo install --path .
```

## Usage

### Basic usage

```bash
# Run in current directory (tracks TODOs for this project)
cc-todos

# Track a specific directory
cc-todos -C /path/to/project
```

### With tmux (recommended)

The best way to use cc-todos is in a tmux pane alongside Claude Code.

#### Quick start

```bash
# Start a new tmux session with Claude Code and TODO viewer
tmux new -s dev -c ~/your/project \; \
  send-keys 'claude' Enter \; \
  split-window -h -p 20 \; \
  send-keys 'cc-todos' Enter \; \
  select-pane -t 0
```

#### If cc-todos is not in PATH

```bash
# Using full path (adjust to your installation)
tmux new -s dev -c ~/your/project \; \
  send-keys 'claude' Enter \; \
  split-window -h -p 20 \; \
  send-keys '~/.cargo/bin/cc-todos' Enter \; \
  select-pane -t 0
```

#### Multiple projects

You can run multiple sessions for different projects:

```bash
# Terminal 1: Project A
tmux new -s project-a -c ~/projects/project-a \; \
  send-keys 'claude' Enter \; \
  split-window -h -p 20 \; \
  send-keys 'cc-todos' Enter \; \
  select-pane -t 0

# Terminal 2: Project B
tmux new -s project-b -c ~/projects/project-b \; \
  send-keys 'claude' Enter \; \
  split-window -h -p 20 \; \
  send-keys 'cc-todos' Enter \; \
  select-pane -t 0
```

Each cc-todos instance will track the TODOs for its respective project.

### Options

| Option | Description |
|--------|-------------|
| `-C, --directory <PATH>` | Working directory to track (defaults to current directory) |
| `--tmux` | Open TODO viewer in a new tmux pane |

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `r` | Reload |

## Requirements

- [Claude Code](https://claude.ai/claude-code) installed
- Rust toolchain (for installation)
- tmux (optional, for side-by-side view)

## How it works

Claude Code stores session information in `~/.claude/projects/` organized by working directory, and TODO lists in `~/.claude/todos/`. This tool:

1. Maps your working directory to the corresponding Claude Code project
2. Finds the latest session for that project
3. Displays and watches the TODO file for real-time updates

## License

MIT
