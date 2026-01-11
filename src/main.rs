mod app;
mod todo;
mod ui;
mod watcher;

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::process::Command;

#[derive(Parser)]
#[command(name = "cc-todos")]
#[command(about = "TUI viewer for Claude Code TODOs")]
struct Cli {
    /// Open in a new tmux pane
    #[arg(long)]
    tmux: bool,

    /// Internal flag: run in TUI mode (used when spawned by tmux)
    #[arg(long, hide = true)]
    tui: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.tui || !cli.tmux {
        // Run TUI directly (always tracks latest file)
        let mut app = app::App::new()?;
        app.run()
    } else {
        // Spawn in tmux pane
        spawn_tmux_pane()
    }
}

fn spawn_tmux_pane() -> Result<()> {
    // Check if we're in tmux
    if env::var("TMUX").is_err() {
        anyhow::bail!("Not in a tmux session. Run without --tmux or start tmux first.");
    }

    let exe = env::current_exe().context("Could not get current executable path")?;

    // Create a new tmux pane on the right and lock layout
    let status = Command::new("tmux")
        .args([
            "split-window",
            "-h",           // horizontal split (right)
            "-d",           // don't switch focus
            "-p", "20",
            &format!("{} --tui", exe.display()),
        ])
        .status()
        .context("Failed to create tmux pane")?;

    if !status.success() {
        anyhow::bail!("tmux split-window failed");
    }

    // Lock the layout to prevent auto-rearrangement
    let _ = Command::new("tmux")
        .args(["select-layout", "-E"])
        .status();

    println!("TODO viewer opened in right pane (tracking latest session)");

    Ok(())
}
