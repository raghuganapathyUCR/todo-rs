use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::domain::TodoList;

pub fn draw(f: &mut Frame, todo_list: &TodoList, selected_index: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let title = Paragraph::new("Todo-rs")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = todo_list
        .todos
        .iter()
        .map(|t| {
            let style = if t.completed {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(&t.title, style))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index));

    let todos = List::new(items)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");
    f.render_stateful_widget(todos, chunks[1], &mut state);
}
