mod app;
mod man;
mod search;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let mut should_quit = false;

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        app.process_search_debounce();

        if crossterm::event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    match app.focus {
                        app::FocusState::Search => {
                            match key.code {
                                KeyCode::Char('q') => {
                                    should_quit = true;
                                }
                                KeyCode::Char(c) => {
                                    app.update_search_query(format!("{}{}", app.search_query, c));
                                }
                                KeyCode::Backspace => {
                                    app.search_query.pop();
                                    app.update_search_query(app.search_query.clone());
                                }
                                KeyCode::Enter => {
                                    app.switch_focus();
                                }
                                KeyCode::Tab => {
                                    app.switch_focus();
                                }
                                KeyCode::Down => {
                                    app.switch_focus();
                                }
                                _ => {}
                            }
                        }
                        app::FocusState::List => {
                            match key.code {
                                KeyCode::Char('q') => {
                                    should_quit = true;
                                }
                                KeyCode::Up => {
                                    app.move_selection_up();
                                }
                                KeyCode::Down => {
                                    app.move_selection_down();
                                }
                                KeyCode::Enter => {
                                    app.load_man_page()?;
                                    app.switch_focus();
                                }
                                KeyCode::Tab => {
                                    app.switch_focus();
                                }
                                KeyCode::Char('/') => {
                                    app.focus_search();
                                }
                                _ => {}
                            }
                        }
                        app::FocusState::Viewer => {
                            match key.code {
                                KeyCode::Char('q') => {
                                    should_quit = true;
                                }
                                KeyCode::Up => {
                                    app.move_selection_up();
                                }
                                KeyCode::Down => {
                                    app.move_selection_down();
                                }
                                KeyCode::PageUp => {
                                    app.scroll_viewer_page_up();
                                }
                                KeyCode::PageDown => {
                                    app.scroll_viewer_page_down();
                                }
                                KeyCode::Tab => {
                                    app.switch_focus();
                                }
                                KeyCode::Char('/') => {
                                    app.focus_search();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Resize(_, _) => {
                    // Terminal was resized, will be handled on next draw
                }
                _ => {}
            }
        }

        if should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

