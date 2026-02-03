// Find & replace implementation
use crate::editor::Buffer;

// Direction for navigating search results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    Forward,
    Backward,
}

// Single search match representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchMatch {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl SearchMatch {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

// Search/replace state
#[derive(Debug, Clone)]
pub struct SearchState {
    pub query: String,
    pub is_open: bool,
    pub matches: Vec<SearchMatch>,
    pub current_match_idx: Option<usize>,
    pub case_sensitive: bool,
    pub replace_text: String,
    pub replace_mode: bool,
    /// Flag to trigger focus on the search input (set to true, component will reset to false)
    pub should_focus: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            is_open: false,
            matches: Vec::new(),
            current_match_idx: None,
            case_sensitive: false,
            should_focus: false,
            replace_text: String::new(),
            replace_mode: false,
        }
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    // Open/focus search bar
    pub fn open(&mut self) {
        self.is_open = true;
        self.should_focus = true;
    }

    // Open search bar in replace mode
    pub fn open_replace(&mut self) {
        self.is_open = true;
        self.replace_mode = true;
        self.should_focus = true;
    }

    // Close search bar & clear state
    pub fn close(&mut self) {
        self.is_open = false;
        self.matches.clear();
        self.current_match_idx = None;
        // Keep query & replace text for next time
    }

    // Update search query & refresh matches
    pub fn set_query(&mut self, query: String, buffer: &Buffer) {
        self.query = query;
        self.find_all(buffer);
    }

    // Find all matches in the buffer
    pub fn find_all(&mut self, buffer: &Buffer) {
        self.matches.clear();
        self.current_match_idx = None;

        if self.query.is_empty() {
            return;
        }

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        // Search through buffer lines
        for (line_idx, line) in buffer.lines().enumerate() {
            let search_line = if self.case_sensitive {
                line.clone()
            } else {
                line.to_lowercase()
            };

            let mut start = 0;
            while let Some(pos) = search_line[start..].find(&query) {
                let match_start = start + pos;
                let match_end = match_start + query.len();

                self.matches.push(SearchMatch::new(
                    line_idx,
                    match_start,
                    line_idx,
                    match_end,
                ));

                // Move past match and find next
                start = match_start + 1;
            }
        }

        // Select first match if any
        if !self.matches.is_empty() {
            self.current_match_idx = Some(0);
        }
    }

    // Get current match
    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.current_match_idx
            .and_then(|idx| self.matches.get(idx))
    }

    // Navigate to next/previous match
    pub fn find_next(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        self.current_match_idx = Some(match self.current_match_idx {
            Some(idx) => (idx + 1) % self.matches.len(),
            None => 0,
        });
    }

    // Move to previous match
    pub fn find_previous(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        self.current_match_idx = Some(match self.current_match_idx {
            Some(idx) => {
                if idx == 0 {
                    self.matches.len() - 1
                } else {
                    idx - 1
                }
            }
            None => self.matches.len() - 1,
        });
    }

    // Replace current match & move to next
    pub fn replace_current(&mut self, buffer: &mut Buffer) {
        if let Some(idx) = self.current_match_idx {
            if let Some(m) = self.matches.get(idx).copied() {
                // Delete matched text
                buffer.set_cursor_position(m.start_line, m.start_col);
                buffer.begin_selection(m.start_line, m.start_col);
                buffer.extend_selection_to(m.end_line, m.end_col);
                buffer.delete_selection();

                // Insert replacement text
                buffer.insert_str(&self.replace_text);

                // Refresh matches after replacement
                self.find_all(buffer);

                // Try to stay at same index, or wrap
                if !self.matches.is_empty() {
                    self.current_match_idx = Some(idx.min(self.matches.len() - 1));
                }
            }
        }
    }

    // Replace all matches
    pub fn replace_all(&mut self, buffer: &mut Buffer) {
        if self.matches.is_empty() || self.query.is_empty() {
            return;
        }

        // Replace from last to first to avoid shifting indices
        let matches_reversed: Vec<SearchMatch> = self.matches.iter().copied().rev().collect();

        for m in matches_reversed {
            // Delete matched text
            buffer.set_cursor_position(m.start_line, m.start_col);
            buffer.begin_selection(m.start_line, m.start_col);
            buffer.extend_selection_to(m.end_line, m.end_col);
            buffer.delete_selection();

            // Insert replacement text
            buffer.insert_str(&self.replace_text);
        }

        // Clear matches
        self.matches.clear();
        self.current_match_idx = None;
    }

    // Get match count for display
    pub fn match_info(&self) -> String {
        if self.query.is_empty() {
            String::new()
        } else if self.matches.is_empty() {
            "No results".to_string()
        } else {
            match self.current_match_idx {
                Some(idx) => format!("{} of {}", idx + 1, self.matches.len()),
                None => format!("0 of {}", self.matches.len()),
            }
        }
    }

    // Toggle case-sensitivity & re-search
    pub fn toggle_case_sensitive(&mut self, buffer: &Buffer) {
        self.case_sensitive = !self.case_sensitive;
        self.find_all(buffer);
    }

    // Check if position is w/in any match
    pub fn is_in_match(&self, line: usize, col: usize) -> Option<bool> {
        for (idx, m) in self.matches.iter().enumerate() {
            if line == m.start_line && col >= m.start_col && col < m.end_col {
                return Some(self.current_match_idx == Some(idx));
            }
        }
        None
    }

    // Get all matches on specific line
    pub fn matches_on_line(&self, line: usize) -> Vec<(usize, usize, bool)> {
        self.matches
            .iter()
            .enumerate()
            .filter(|(_, m)| m.start_line == line)
            .map(|(idx, m)| (m.start_col, m.end_col, self.current_match_idx == Some(idx)))
            .collect()
    }
}