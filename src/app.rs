use crate::man;
use crate::search;
use anyhow::Result;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum FocusState {
    Search,
    List,
    Viewer,
}

pub struct App {
    pub search_query: String,
    pub all_commands: Vec<String>,
    pub filtered_commands: Vec<String>,
    pub selected_index: usize,
    pub man_content: String,
    pub scroll_offset: usize,
    pub focus: FocusState,
    pub last_search_update: Option<Instant>,
}

impl App {
    pub fn new() -> Result<Self> {
        let all_commands = man::discover_man_pages()?;
        let filtered_commands = all_commands.clone();

        Ok(Self {
            search_query: String::new(),
            all_commands,
            filtered_commands,
            selected_index: 0,
            man_content: String::new(),
            scroll_offset: 0,
            focus: FocusState::Search,
            last_search_update: None,
        })
    }

    pub fn update_search_query(&mut self, query: String) {
        self.search_query = query;
        self.last_search_update = Some(Instant::now());
        self.selected_index = 0;
    }

    pub fn process_search_debounce(&mut self) {
        if let Some(last_update) = self.last_search_update {
            if last_update.elapsed() >= Duration::from_millis(150) {
                self.filtered_commands = search::filter_commands(&self.search_query, &self.all_commands);
                if self.selected_index >= self.filtered_commands.len() && !self.filtered_commands.is_empty() {
                    self.selected_index = self.filtered_commands.len() - 1;
                } else if self.filtered_commands.is_empty() {
                    self.selected_index = 0;
                }
                self.last_search_update = None;
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        if self.focus == FocusState::List && !self.filtered_commands.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        } else if self.focus == FocusState::Viewer {
            if self.scroll_offset > 0 {
                self.scroll_offset -= 1;
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.focus == FocusState::List && !self.filtered_commands.is_empty() {
            if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
                self.selected_index += 1;
            }
        } else if self.focus == FocusState::Viewer {
            self.scroll_offset += 1;
        }
    }

    pub fn load_man_page(&mut self) -> Result<()> {
        if let Some(cmd) = self.filtered_commands.get(self.selected_index) {
            self.man_content = man::fetch_man_page(cmd)?;
            self.scroll_offset = 0;
        }
        Ok(())
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            FocusState::Search => FocusState::List,
            FocusState::List => FocusState::Viewer,
            FocusState::Viewer => FocusState::Search,
        };
    }

    pub fn focus_search(&mut self) {
        self.focus = FocusState::Search;
    }

    pub fn scroll_viewer_page_up(&mut self) {
        if self.focus == FocusState::Viewer {
            self.scroll_offset = self.scroll_offset.saturating_sub(10);
        }
    }

    pub fn scroll_viewer_page_down(&mut self) {
        if self.focus == FocusState::Viewer {
            self.scroll_offset += 10;
        }
    }
}

