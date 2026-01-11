mod app;
mod todo;
mod ui;
mod watcher;

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "cc-todos")]
#[command(about = "TUI viewer for Claude Code TODOs")]
struct Cli {
    /// Working directory to track (defaults to current directory)
    #[arg(short = 'C', long)]
    directory: Option<PathBuf>,

    /// Open in a new tmux pane
    #[arg(long)]
    tmux: bool,

    /// Internal flag: run in TUI mode (used when spawned by tmux)
    #[arg(long, hide = true)]
    tui: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cwd = cli.directory
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    if cli.tui || !cli.tmux {
        // Run TUI directly
        let mut app = app::App::new(cwd)?;
        app.run()
    } else {
        // Spawn in tmux pane
        spawn_tmux_pane(&cwd)
    }
}

fn spawn_tmux_pane(cwd: &PathBuf) -> Result<()> {
    // Check if we're in tmux
    if env::var("TMUX").is_err() {
        anyhow::bail!("Not in a tmux session. Run without --tmux or start tmux first.");
    }

    let exe = env::current_exe().context("Could not get current executable path")?;
    let cwd_str = cwd.to_string_lossy();

    // Create a new tmux pane on the right
    let status = Command::new("tmux")
        .args([
            "split-window",
            "-h",
            "-d",
            "-p", "20",
            &format!("{} --tui -C '{}'", exe.display(), cwd_str),
        ])
        .status()
        .context("Failed to create tmux pane")?;

    if !status.success() {
        anyhow::bail!("tmux split-window failed");
    }

    // Lock the layout
    let _ = Command::new("tmux")
        .args(["select-layout", "-E"])
        .status();

    println!("TODO viewer opened in right pane");
    println!("Tracking: {}", cwd_str);

    Ok(())
}
