use crate::config::Config;
use crate::man;
use crate::search;
use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum FocusState {
    Search,
    List,
    Viewer,
    SectionSelect,
    InPageSearch,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn from_str(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
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
    pub selected_section: Option<u8>,
    pub available_sections: Vec<u8>,
    pub section_select_index: usize,
    pub in_page_search_query: String,
    pub in_page_search_matches: Vec<usize>,
    pub current_match_index: Option<usize>,
    pub cache: HashMap<String, String>,
    pub theme: Theme,
    pub config: Config,
    pub current_command: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load().unwrap_or_default();
        let theme = Theme::from_str(&config.theme);
        let available_sections = man::get_available_sections();
        let selected_section = config
            .last_section
            .or_else(|| available_sections.first().copied());
        let all_commands = man::discover_man_pages(selected_section)?;
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
            selected_section,
            available_sections,
            section_select_index: 0,
            in_page_search_query: String::new(),
            in_page_search_matches: Vec::new(),
            current_match_index: None,
            cache: HashMap::new(),
            theme,
            config,
            current_command: None,
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
                self.filtered_commands =
                    search::filter_commands(&self.search_query, &self.all_commands);
                if self.selected_index >= self.filtered_commands.len()
                    && !self.filtered_commands.is_empty()
                {
                    self.selected_index = self.filtered_commands.len() - 1;
                } else if self.filtered_commands.is_empty() {
                    self.selected_index = 0;
                }
                self.last_search_update = None;
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        match self.focus {
            FocusState::List if !self.filtered_commands.is_empty() => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            FocusState::Viewer => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
            FocusState::SectionSelect => {
                if self.section_select_index > 0 {
                    self.section_select_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.focus {
            FocusState::List if !self.filtered_commands.is_empty() => {
                if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
            }
            FocusState::Viewer => {
                self.scroll_offset += 1;
            }
            FocusState::SectionSelect => {
                if self.section_select_index < self.available_sections.len().saturating_sub(1) {
                    self.section_select_index += 1;
                }
            }
            _ => {}
        }
    }

    pub fn load_man_page(&mut self) -> Result<()> {
        if let Some(cmd) = self.filtered_commands.get(self.selected_index) {
            self.current_command = Some(cmd.clone());

            let cache_key = format!(
                "{}:{}",
                cmd,
                self.selected_section
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            );

            if let Some(cached) = self.cache.get(&cache_key) {
                self.man_content = cached.clone();
            } else {
                let content = man::fetch_man_page(cmd, self.selected_section)?;
                self.cache.insert(cache_key.clone(), content.clone());
                self.man_content = content;
            }

            self.scroll_offset = 0;
            self.in_page_search_query.clear();
            self.in_page_search_matches.clear();
            self.current_match_index = None;

            if let Some(ref cmd) = self.current_command {
                self.config.add_to_history(cmd.clone());
            }
        }
        Ok(())
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            FocusState::Search => FocusState::List,
            FocusState::List => FocusState::Viewer,
            FocusState::Viewer => FocusState::Search,
            FocusState::SectionSelect => FocusState::List,
            FocusState::InPageSearch => FocusState::Viewer,
        };
    }

    pub fn focus_search(&mut self) {
        self.focus = FocusState::Search;
    }

    pub fn focus_list(&mut self) {
        self.focus = FocusState::List;
    }

    pub fn focus_section_select(&mut self) {
        self.focus = FocusState::SectionSelect;
        if let Some(selected) = self.selected_section {
            if let Some(idx) = self.available_sections.iter().position(|&s| s == selected) {
                self.section_select_index = idx;
            }
        }
    }

    pub fn select_section(&mut self) -> Result<()> {
        if let Some(&section) = self.available_sections.get(self.section_select_index) {
            self.selected_section = Some(section);
            self.all_commands = man::discover_man_pages(Some(section))?;
            self.filtered_commands =
                search::filter_commands(&self.search_query, &self.all_commands);
            self.selected_index = 0;
            self.focus = FocusState::List;
        }
        Ok(())
    }

    pub fn start_in_page_search(&mut self) {
        self.focus = FocusState::InPageSearch;
        self.in_page_search_query.clear();
        self.in_page_search_matches.clear();
        self.current_match_index = None;
    }

    pub fn update_in_page_search(&mut self, query: String) {
        self.in_page_search_query = query.clone();
        self.find_in_page_matches();
    }

    fn find_in_page_matches(&mut self) {
        if self.in_page_search_query.is_empty() {
            self.in_page_search_matches.clear();
            self.current_match_index = None;
            return;
        }

        let query_lower = self.in_page_search_query.to_lowercase();
        let lines: Vec<&str> = self.man_content.lines().collect();
        self.in_page_search_matches.clear();

        for (idx, line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&query_lower) {
                self.in_page_search_matches.push(idx);
            }
        }

        if !self.in_page_search_matches.is_empty() {
            self.current_match_index = Some(0);
            if let Some(&first_match) = self.in_page_search_matches.first() {
                self.scroll_offset = first_match.saturating_sub(5);
            }
        } else {
            self.current_match_index = None;
        }
    }

    pub fn next_match(&mut self) {
        if let Some(current_idx) = self.current_match_index {
            if current_idx < self.in_page_search_matches.len().saturating_sub(1) {
                self.current_match_index = Some(current_idx + 1);
                if let Some(&line_idx) = self.in_page_search_matches.get(current_idx + 1) {
                    self.scroll_offset = line_idx.saturating_sub(5);
                }
            }
        }
    }

    pub fn prev_match(&mut self) {
        if let Some(current_idx) = self.current_match_index {
            if current_idx > 0 {
                self.current_match_index = Some(current_idx - 1);
                if let Some(&line_idx) = self.in_page_search_matches.get(current_idx - 1) {
                    self.scroll_offset = line_idx.saturating_sub(5);
                }
            }
        }
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

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
        self.config.theme = self.theme.to_str().to_string();
    }

    pub fn toggle_favorite(&mut self) {
        if let Some(ref cmd) = self.current_command {
            let _is_fav = self.config.toggle_favorite(cmd.clone());
            // Could show a message here if needed
        }
    }

    pub fn is_favorite(&self) -> bool {
        self.current_command
            .as_ref()
            .map(|cmd| self.config.is_favorite(cmd))
            .unwrap_or(false)
    }

    pub fn save_config(&mut self) -> Result<()> {
        if let Some(ref cmd) = self.current_command {
            self.config.last_command = Some(cmd.clone());
        }
        self.config.last_section = self.selected_section;
        self.config.save()
    }
}
