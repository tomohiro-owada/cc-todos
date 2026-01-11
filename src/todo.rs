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

pub fn get_projects_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".claude").join("projects"))
}

/// Convert a working directory path to Claude's project folder name
/// e.g., /Users/towada/projects/cc-todos -> -Users-towada-projects-cc-todos
fn path_to_project_name(path: &PathBuf) -> String {
    path.to_string_lossy()
        .replace('/', "-")
}

/// Find the latest session ID for a given working directory
pub fn find_session_for_cwd(cwd: &PathBuf) -> Result<String> {
    let projects_dir = get_projects_dir()?;
    let project_name = path_to_project_name(cwd);
    let project_dir = projects_dir.join(&project_name);

    if !project_dir.exists() {
        anyhow::bail!("No Claude project found for: {:?}", cwd);
    }

    // Find the most recently modified session directory
    let mut entries: Vec<_> = fs::read_dir(&project_dir)
        .context("Could not read project directory")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    entries.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    entries
        .last()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .context("No session found for project")
}

/// Find the TODO file for a specific session ID
pub fn find_todo_file_for_session(session_id: &str) -> Result<PathBuf> {
    let todos_dir = get_todos_dir()?;

    let entries: Vec<_> = fs::read_dir(&todos_dir)
        .context("Could not read todos directory")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with(session_id)
        })
        .collect();

    // Get the main session file (session-agent-session.json)
    entries
        .into_iter()
        .find(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name == format!("{}-agent-{}.json", session_id, session_id)
        })
        .or_else(|| {
            // Fallback to any matching file
            fs::read_dir(&todos_dir)
                .ok()?
                .filter_map(|e| e.ok())
                .find(|e| e.file_name().to_string_lossy().starts_with(session_id))
        })
        .map(|e| e.path())
        .with_context(|| format!("No todo file found for session: {}", session_id))
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
