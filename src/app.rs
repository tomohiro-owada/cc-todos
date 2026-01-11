use crate::todo::{find_session_for_cwd, find_todo_file_for_session, get_todos_dir, load_todos, find_latest_todo_file, Todo};
use crate::ui;
use crate::watcher::DirWatcher;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::path::PathBuf;
use std::time::Duration;

pub struct App {
    pub todos: Vec<Todo>,
    pub file_path: PathBuf,
    pub session_id: Option<String>,
    pub cwd: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new(cwd: PathBuf) -> Result<Self> {
        // Try to find session for this working directory
        let (session_id, file_path) = match find_session_for_cwd(&cwd) {
            Ok(sid) => {
                let fp = find_todo_file_for_session(&sid).unwrap_or_default();
                (Some(sid), fp)
            }
            Err(_) => {
                // Fallback to latest file
                (None, find_latest_todo_file().unwrap_or_default())
            }
        };

        let todos = load_todos(&file_path).unwrap_or_default();

        Ok(App {
            todos,
            file_path,
            session_id,
            cwd,
            should_quit: false,
        })
    }

    pub fn reload_todos(&mut self) {
        // Try to find session for our working directory
        if let Ok(sid) = find_session_for_cwd(&self.cwd) {
            self.session_id = Some(sid.clone());
            if let Ok(fp) = find_todo_file_for_session(&sid) {
                self.file_path = fp;
            }
        }

        if let Ok(todos) = load_todos(&self.file_path) {
            self.todos = todos;
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let todos_dir = get_todos_dir()?;
        let watcher = DirWatcher::new(&todos_dir)?;

        let result = self.run_loop(&mut terminal, &watcher);

        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;

        result
    }

    fn run_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        watcher: &DirWatcher,
    ) -> Result<()> {
        loop {
            if self.should_quit {
                break;
            }

            // Check for file changes
            if watcher.try_recv().is_some() {
                self.reload_todos();
            }

            // Draw UI
            let file_path_str = self.file_path.to_string_lossy().to_string();
            terminal.draw(|frame| {
                ui::draw(frame, &self.todos, &file_path_str);
            })?;

            // Handle input with timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                self.should_quit = true;
                            }
                            KeyCode::Char('r') => {
                                self.reload_todos();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
