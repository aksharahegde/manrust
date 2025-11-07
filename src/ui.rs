use crate::app::{App, FocusState};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_search_bar(f, app, chunks[0]);
    render_main_panes(f, app, chunks[1]);
    render_footer(f, chunks[2]);
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let search_style = if app.focus == FocusState::Search {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let border_style = if app.focus == FocusState::Search {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let search_text = format!("Search: {}", app.search_query);
    let paragraph = Paragraph::new(search_text.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("Search"),
        )
        .style(search_style);

    f.render_widget(paragraph, area);
}

fn render_main_panes(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_command_list(f, app, chunks[0]);
    render_man_viewer(f, app, chunks[1]);
}

fn render_command_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_commands
        .iter()
        .map(|cmd| {
            ListItem::new(cmd.as_str())
        })
        .collect();

    let border_style = if app.focus == FocusState::List {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("Commands"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_man_viewer(f: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus == FocusState::Viewer {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let lines: Vec<Line> = app
        .man_content
        .lines()
        .skip(app.scroll_offset)
        .take(area.height as usize - 2)
        .map(|line| Line::from(line))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("Man Page"),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help_text = "[↑↓] Navigate  [Enter] Open  [Tab] Switch  [q] Quit";
    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(paragraph, area);
}

