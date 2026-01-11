use crate::todo::{Todo, TodoStatus};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Padding},
    Frame,
};

pub fn draw(frame: &mut Frame, todos: &[Todo], file_path: &str) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3),  // Progress bar
        Constraint::Min(3),     // TODO list
        Constraint::Length(1),  // Status bar
    ]).split(area);

    draw_progress(frame, chunks[0], todos);
    draw_todo_list(frame, chunks[1], todos);
    draw_status_bar(frame, chunks[2], file_path);
}

fn draw_progress(frame: &mut Frame, area: Rect, todos: &[Todo]) {
    let total = todos.len();
    let completed = todos.iter().filter(|t| matches!(t.status, TodoStatus::Completed)).count();

    let ratio = if total > 0 { completed as f64 / total as f64 } else { 0.0 };

    let label = if total > 0 {
        format!("{}/{} completed", completed, total)
    } else {
        "No tasks".to_string()
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(ratio)
        .label(label);

    frame.render_widget(gauge, area);
}

fn draw_todo_list(frame: &mut Frame, area: Rect, todos: &[Todo]) {
    let block = Block::default()
        .title(" Claude Code TODOs ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    if todos.is_empty() {
        let empty_msg = ListItem::new(Line::from(vec![
            Span::styled("  No TODOs in this session", Style::default().fg(Color::DarkGray)),
        ]));
        let list = List::new(vec![empty_msg]).block(block);
        frame.render_widget(list, area);
        return;
    }

    let mut items: Vec<ListItem> = Vec::new();

    // In Progress section
    let in_progress: Vec<_> = todos.iter().filter(|t| matches!(t.status, TodoStatus::InProgress)).collect();
    if !in_progress.is_empty() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled("▶ IN PROGRESS", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ])));
        for todo in &in_progress {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ● ", Style::default().fg(Color::Yellow)),
                Span::styled(&todo.active_form, Style::default().fg(Color::Yellow)),
            ])));
        }
        items.push(ListItem::new(Line::from("")));
    }

    // Pending section
    let pending: Vec<_> = todos.iter().filter(|t| matches!(t.status, TodoStatus::Pending)).collect();
    if !pending.is_empty() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled("○ PENDING", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ])));
        for todo in &pending {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(Color::DarkGray)),
                Span::styled(&todo.content, Style::default().fg(Color::White)),
            ])));
        }
        items.push(ListItem::new(Line::from("")));
    }

    // Completed section
    let completed: Vec<_> = todos.iter().filter(|t| matches!(t.status, TodoStatus::Completed)).collect();
    if !completed.is_empty() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled("✓ COMPLETED", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ])));
        for todo in &completed {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(Color::Green)),
                Span::styled(
                    &todo.content,
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT),
                ),
            ])));
        }
    }

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_status_bar(frame: &mut Frame, area: Rect, file_path: &str) {
    let status = Line::from(vec![
        Span::styled(" q: quit  r: reload ", Style::default().fg(Color::DarkGray)),
        Span::raw("| "),
        Span::styled(
            file_path.split('/').last().unwrap_or(file_path),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    frame.render_widget(status, area);
}
