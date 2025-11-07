use crate::app::{App, FocusState, Theme};
use crate::colorize::ManColorizer;
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

    if app.focus == FocusState::SectionSelect {
        render_section_select(f, app, chunks[1]);
    } else {
        render_main_panes(f, app, chunks[1]);
    }

    render_footer(f, app, chunks[2]);
}

fn get_colors(theme: Theme) -> (Color, Color, Color, Color) {
    match theme {
        Theme::Dark => (Color::Black, Color::White, Color::Cyan, Color::Yellow),
        Theme::Light => (Color::White, Color::Black, Color::Blue, Color::Magenta),
    }
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let (bg_color, fg_color, accent_color, _) = get_colors(app.theme);

    let search_style = if app.focus == FocusState::Search || app.focus == FocusState::InPageSearch {
        Style::default().fg(accent_color).bg(bg_color)
    } else {
        Style::default().fg(fg_color).bg(bg_color)
    };

    let border_style = if app.focus == FocusState::Search || app.focus == FocusState::InPageSearch {
        Style::default().fg(accent_color)
    } else {
        Style::default().fg(fg_color)
    };

    let search_text = if app.focus == FocusState::InPageSearch {
        format!("In-page search: {}", app.in_page_search_query)
    } else {
        format!("Search: {}", app.search_query)
    };

    let title = if app.focus == FocusState::InPageSearch {
        "In-Page Search"
    } else {
        "Search"
    };

    let paragraph = Paragraph::new(search_text.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
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
    let (bg_color, fg_color, _, highlight_color) = get_colors(app.theme);

    let items: Vec<ListItem> = app
        .filtered_commands
        .iter()
        .map(|cmd| {
            let text = if app.config.is_favorite(cmd) {
                format!("★ {}", cmd)
            } else {
                cmd.clone()
            };
            ListItem::new(text)
        })
        .collect();

    let border_style = if app.focus == FocusState::List {
        Style::default().fg(highlight_color)
    } else {
        Style::default().fg(fg_color)
    };

    let section_text = if let Some(sec) = app.selected_section {
        format!("Commands (Section {})", sec)
    } else {
        "Commands (All Sections)".to_string()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(section_text),
        )
        .highlight_style(
            Style::default()
                .bg(highlight_color)
                .fg(bg_color)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_man_viewer(f: &mut Frame, app: &App, area: Rect) {
    let (bg_color, fg_color, _, highlight_color) = get_colors(app.theme);
    let colorizer = ManColorizer::new(app.theme);

    let border_style = if app.focus == FocusState::Viewer {
        Style::default().fg(highlight_color)
    } else {
        Style::default().fg(fg_color)
    };

    let viewer_height = area.height as usize - 2;
    let lines: Vec<Line> = app
        .man_content
        .lines()
        .enumerate()
        .skip(app.scroll_offset)
        .take(viewer_height)
        .map(|(line_idx, line)| {
            if !app.in_page_search_query.is_empty()
                && app.in_page_search_matches.contains(&line_idx)
            {
                // Apply search highlighting on top of colorized content
                let query_lower = app.in_page_search_query.to_lowercase();
                let line_lower = line.to_lowercase();

                // Merge search highlighting with existing colorization
                let mut spans = Vec::new();
                let mut last_end = 0;
                let mut search_start = 0;

                while let Some(start) = line_lower[search_start..].find(&query_lower) {
                    let actual_start = search_start + start;
                    let actual_end = actual_start + query_lower.len();

                    // Add text before match (preserving original colors)
                    if actual_start > last_end {
                        let before_text = &line[last_end..actual_start];
                        spans.push(Span::styled(before_text, Style::default().fg(fg_color)));
                    }

                    // Highlight the match
                    spans.push(Span::styled(
                        &line[actual_start..actual_end],
                        Style::default()
                            .fg(bg_color)
                            .bg(highlight_color)
                            .add_modifier(Modifier::BOLD),
                    ));

                    last_end = actual_end;
                    search_start = actual_end;
                }

                // Add remaining text
                if last_end < line.len() {
                    spans.push(Span::styled(
                        &line[last_end..],
                        Style::default().fg(fg_color),
                    ));
                }

                if spans.is_empty() {
                    Line::from(line)
                } else {
                    Line::from(spans)
                }
            } else {
                // Apply colorization - inline to avoid lifetime issues
                let trimmed = line.trim();
                let mut spans = Vec::new();

                // Get colors based on theme
                let section_color = match app.theme {
                    crate::app::Theme::Dark => Color::Cyan,
                    crate::app::Theme::Light => Color::Blue,
                };
                let option_color = Color::Green;
                let code_color = match app.theme {
                    crate::app::Theme::Dark => Color::Magenta,
                    crate::app::Theme::Light => Color::Red,
                };

                if trimmed.is_empty() {
                    spans.push(Span::styled(line, Style::default().fg(fg_color)));
                } else if colorizer.is_section_header(trimmed) {
                    spans.push(Span::styled(
                        line,
                        Style::default()
                            .fg(section_color)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else if trimmed.starts_with('-') || trimmed.starts_with("--") {
                    let trimmed = line.trim();
                    if let Some(option_end) = trimmed.find(' ') {
                        let line_start = line.len() - trimmed.len();
                        let option_end_abs = line_start + option_end;
                        spans.push(Span::styled(
                            &line[..option_end_abs],
                            Style::default()
                                .fg(option_color)
                                .add_modifier(Modifier::BOLD),
                        ));
                        spans.push(Span::styled(
                            &line[option_end_abs..],
                            Style::default().fg(fg_color),
                        ));
                    } else {
                        spans.push(Span::styled(
                            line,
                            Style::default()
                                .fg(option_color)
                                .add_modifier(Modifier::BOLD),
                        ));
                    }
                } else if line.starts_with("    ") || line.starts_with('\t') {
                    if colorizer.looks_like_code(trimmed) {
                        spans.push(Span::styled(line, Style::default().fg(code_color)));
                    } else {
                        spans.push(Span::styled(line, Style::default().fg(fg_color)));
                    }
                } else {
                    spans.push(Span::styled(line, Style::default().fg(fg_color)));
                }

                if spans.is_empty() {
                    Line::from(line)
                } else {
                    Line::from(spans)
                }
            }
        })
        .collect();

    let title = if app.is_favorite() {
        format!("Man Page ★")
    } else {
        "Man Page".to_string()
    };

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .style(Style::default().fg(fg_color).bg(bg_color))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);

    if !app.in_page_search_query.is_empty() && !app.in_page_search_matches.is_empty() {
        if let Some(current_idx) = app.current_match_index {
            let match_info = format!(
                "Match {}/{}",
                current_idx + 1,
                app.in_page_search_matches.len()
            );
            let info_paragraph = Paragraph::new(match_info.as_str())
                .style(Style::default().fg(highlight_color).bg(bg_color));
            let info_area = Rect {
                x: area.x + 1,
                y: area.y + area.height - 1,
                width: match_info.len() as u16 + 2,
                height: 1,
            };
            f.render_widget(info_paragraph, info_area);
        }
    }
}

fn render_section_select(f: &mut Frame, app: &App, area: Rect) {
    let (bg_color, _fg_color, accent_color, highlight_color) = get_colors(app.theme);

    let items: Vec<ListItem> = app
        .available_sections
        .iter()
        .map(|sec| {
            let text = format!("Section {}", sec);
            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent_color))
                .title("Select Man Section"),
        )
        .highlight_style(
            Style::default()
                .bg(highlight_color)
                .fg(bg_color)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(app.section_select_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let (_, fg_color, _, _) = get_colors(app.theme);

    let help_text = if app.focus == FocusState::InPageSearch {
        "[n] Next  [N] Prev  [Esc] Cancel  [q] Quit"
    } else if app.focus == FocusState::SectionSelect {
        "[Enter] Select  [Esc] Cancel  [q] Quit"
    } else {
        "[↑↓] Navigate  [Enter] Open  [Tab] Switch  [/] Search  [l] List  [s] Section  [f] Favorite  [t] Theme  [q] Quit"
    };

    let paragraph = Paragraph::new(help_text).style(Style::default().fg(fg_color));

    f.render_widget(paragraph, area);
}
