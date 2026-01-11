use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub content: String,
    pub status: TodoStatus,
    pub active_form: String,
}

pub fn get_todos_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".claude").join("todos"))
}

pub fn find_latest_todo_file() -> Result<PathBuf> {
    let todos_dir = get_todos_dir()?;
    let mut entries: Vec<_> = fs::read_dir(&todos_dir)
        .context("Could not read todos directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();

    entries.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    entries
        .last()
        .map(|e| e.path())
        .context("No todo files found")
}

pub fn load_todos(path: &PathBuf) -> Result<Vec<Todo>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read todo file: {:?}", path))?;
    let todos: Vec<Todo> = serde_json::from_str(&content)
        .with_context(|| format!("Could not parse todo file: {:?}", path))?;
    Ok(todos)
}
