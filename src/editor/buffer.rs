// Text buffer implementation using Ropey

// Allow unused - these are API methods for future use
#![allow(dead_code)]

use ropey::Rope;
use std::path::PathBuf;
use super::cursor::Cursor;
use crate::history::{History, EditAction};

// Text buffer structure
#[derive(Debug, Clone)]
pub struct Buffer {
    // Rope that contains the text
    rope: Rope,
    // Cursor state
    cursor: Cursor,
    // If rope has unsaved changes
    dirty: bool,
    // File path if associated w/ a file
    path: Option<PathBuf>,
    // Undo/redo history
    history: History,
}

impl Buffer {
    // Create new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            cursor: Cursor::new(),
            dirty: false,
            path: None,
            history: History::new(),
        }
    }

    // Create a buffer w/ initial text
    pub fn new_with_text(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            cursor: Cursor::new(),
            dirty: false,
            path: None,
            history: History::new(),
        }
    }

    /// Create a buffer from file content with associated path
    pub fn from_file(path: PathBuf, content: String) -> Self {
        Self {
            rope: Rope::from_str(&content),
            cursor: Cursor::new(),
            dirty: false,
            path: Some(path),
            history: History::new(),
        }
    }

    /// Load content from a file, replacing current buffer contents
    pub fn load_content(&mut self, path: PathBuf, content: String) {
        self.rope = Rope::from_str(&content);
        self.path = Some(path);
        self.cursor = Cursor::new();
        self.dirty = false;
        self.history.clear();
    }

    // Save current state to history for insert operations
    fn save_to_history_insert(&mut self, ch: Option<char>) {
        self.history.save_state_for_edit(&self.rope, self.cursor.position, EditAction::Insert, ch);
    }

    // Save current state to history for delete operations
    fn save_to_history_delete(&mut self) {
        self.history.save_state_for_edit(&self.rope, self.cursor.position, EditAction::Delete, None);
    }

    // Save current state to history for other operations (selection delete, paste, etc.)
    fn save_to_history_other(&mut self) {
        self.history.save_state_for_edit(&self.rope, self.cursor.position, EditAction::Other, None);
    }

    /// Get the filename (just the name, not full path)
    pub fn filename(&self) -> Option<String> {
        self.path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }

    // Get current cursor line
    pub fn cursor_line(&self) -> usize {
        self.cursor.position.line
    }

    // Get current cursor column
    pub fn cursor_col(&self) -> usize {
        self.cursor.position.col
    }

    // Total line count
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    // Total character count
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    // Get the char index from current cursor position
    fn cursor_char_idx(&self) -> usize {
        let line_start = self.rope
            .line_to_char(self.cursor.position.line);
        line_start + self.cursor.position.col
    }

    // Get length of a specific line (in chars, excluding newline)
    fn line_len(&self, line_idx: usize) -> usize {
        if line_idx >= self.rope.len_lines() {
            return 0;
        }
        let line = self.rope.line(line_idx);
        // -1 if line ends w/ newline (except last line)
        let len = line.len_chars();
        if len > 0 && line.char(len - 1) == '\n' {
            len - 1
        } else {
            len
        }
    }

    // Insert char at cursor position
    pub fn insert_char(&mut self, ch: char) {
        self.save_to_history_insert(Some(ch));

        let idx = self.cursor_char_idx();
        self.rope.insert_char(idx, ch);
        self.dirty = true;

        // Move cursor forward
        if ch == '\n' {
            self.cursor.position.line += 1;
            self.cursor.position.col = 0;
        } else {
            self.cursor.position.col += 1;
        }
    }

    // Insert str at cursor position
    pub fn insert_str(&mut self, text: &str) {
        self.save_to_history_other();

        let idx = self.cursor_char_idx();
        self.rope.insert(idx, text);
        self.dirty = true;

        // Update cursor position based on inserted text
        for ch in text.chars() {
            if ch == '\n' {
                self.cursor.position.line += 1;
                self.cursor.position.col = 0;
            } else {
                self.cursor.position.col += 1;
            }
        }
    }

    /// Delete the character before the cursor (backspace)
    pub fn delete_backward(&mut self) {
        let idx = self.cursor_char_idx();
        if idx == 0 {
            return; // Nothing to delete
        }

        self.save_to_history_delete();

        // Check if we're deleting a newline
        let char_to_delete = self.rope.char(idx - 1);

        self.rope.remove(idx - 1..idx);
        self.dirty = true;

        // Move cursor back
        if char_to_delete == '\n' {
            self.cursor.position.line -= 1;
            self.cursor.position.col = self.line_len(self.cursor.position.line);
        } else {
            self.cursor.position.col -= 1;
        }
    }

    /// Delete the character at the cursor (delete key)
    pub fn delete_forward(&mut self) {
        let idx = self.cursor_char_idx();
        if idx >= self.rope.len_chars() {
            return; // Nothing to delete
        }

        self.save_to_history_delete();

        self.rope.remove(idx..idx + 1);
        self.dirty = true;
        // Cursor stays in place
    }

    // Undo last edit
    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.history.undo(&self.rope, self.cursor.position) {
            self.rope = snapshot.rope;
            self.cursor.position = snapshot.cursor_position;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    // Redo last undone edit
    pub fn redo(&mut self) -> bool {
        if let Some(snapshot) = self.history.redo(&self.rope, self.cursor.position) {
            self.rope = snapshot.rope;
            self.cursor.position = snapshot.cursor_position;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    // Check if undo is possible
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    // Check if redo is possible
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    // Move cursor up one line
    pub fn move_up(&mut self) {
        self.move_up_with_selection(false);
    }

    // Move cursor down one line
    pub fn move_down(&mut self) {
        self.move_down_with_selection(false);
    }

    // Move cursor left one character
    pub fn move_left(&mut self) {
        self.move_left_with_selection(false);
    }

    // Move cursor right one character
    pub fn move_right(&mut self) {
        self.move_right_with_selection(false);
    }

    /// Move cursor to start of current line
    pub fn move_to_line_start(&mut self) {
        self.move_to_line_start_with_selection(false);
    }

    /// Move cursor to end of current line
    pub fn move_to_line_end(&mut self) {
        self.move_to_line_end_with_selection(false);
    }

    /// Get an iterator over all lines as strings
    pub fn lines(&self) -> impl Iterator<Item = String> + '_ {
        self.rope.lines().map(|line| {
            // Remove trailing newline for display
            let s = line.to_string();
            s.trim_end_matches('\n').to_string()
        })
    }

    /// Get the full text content
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Check if buffer has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark as saved (clear dirty flag)
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Set the file path
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    /// Get the file path
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.rope = Rope::new();
        self.cursor = Cursor::new();
        self.dirty = false;
        self.history.clear();
    }

    // Get current selection as Selection struct
    pub fn selection(&self) -> Option<crate::editor::cursor::Selection> {
        self.cursor.selection()
    }

    // Check if active selection
    pub fn has_selection(&self) -> bool {
        self.cursor.has_selection()
    }

    // Get selected text
    pub fn selected_text(&self) -> Option<String> {
        let (start, end) = self.cursor.selection_range()?;

        let start_idx = self.position_to_char_idx(&start);
        let end_idx = self.position_to_char_idx(&end);

        if start_idx < end_idx && end_idx <= self.rope.len_chars() {
            Some(self.rope.slice(start_idx..end_idx).to_string())
        } else {
            None
        }
    }

    // Convert Position to char index
    fn position_to_char_idx(&self, pos: &crate::editor::cursor::Position) -> usize {
        if pos.line >= self.rope.len_lines() {
            return self.rope.len_chars();
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.line_len(pos.line);
        line_start + pos.col.min(line_len)
    }

    // Delete selected text
    pub fn delete_selection(&mut self) -> Option<String> {
        let (start, end) = self.cursor.selection_range()?;

        self.save_to_history_other();

        let start_idx = self.position_to_char_idx(&start);
        let end_idx = self.position_to_char_idx(&end);

        if start_idx < end_idx && end_idx <= self.rope.len_chars() {
            let deleted = self.rope.slice(start_idx..end_idx).to_string();
            self.rope.remove(start_idx..end_idx);

            // Move cursor to start of selection
            self.cursor.position = start;
            self.cursor.clear_selection();
            self.dirty = true;
            Some(deleted)
        } else {
            self.cursor.clear_selection();
            None
        }
    }

    // Select all
    pub fn select_all(&mut self) {
        let last_line = self.rope.len_lines().saturating_sub(1);
        let last_col = self.line_len(last_line);
        self.cursor.select_all(crate::editor::cursor::Position::new(last_line, last_col));
    }

    // Clear selection w/o deleting
    pub fn clear_selection(&mut self) {
        self.cursor.clear_selection();
    }

    // Start or extend selection
    pub fn start_selection(&mut self) {
        self.cursor.start_selection();
    }

    // Move cursor up, optionally extending selection
    pub fn move_up_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((start, _)) = self.cursor.selection_range() {
            // Collapse to start of selection, don't move further
            self.cursor.position = start;
            self.cursor.clear_selection();
            return;
        }

        if self.cursor.position.line > 0 {
            self.cursor.position.line -= 1;
            let max_col = self.line_len(self.cursor.position.line);
            self.cursor.position.col = self.cursor.position.col.min(max_col);
        }
    }

    // Move cursor down, optionally extending selection
    pub fn move_down_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((_, end)) = self.cursor.selection_range() {
            // Collapse to end of selection, don't move further
            self.cursor.position = end;
            self.cursor.clear_selection();
            return;
        }

        if self.cursor.position.line < self.rope.len_lines().saturating_sub(1) {
            self.cursor.position.line += 1;
            let max_col = self.line_len(self.cursor.position.line);
            self.cursor.position.col = self.cursor.position.col.min(max_col);
        }
    }

    // Move cursor left, optionally extending selection
    pub fn move_left_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((start, _)) = self.cursor.selection_range() {
            // Collapse to start of selection, don't move further
            self.cursor.position = start;
            self.cursor.clear_selection();
            return;
        }

        if self.cursor.position.col > 0 {
            self.cursor.position.col -= 1;
        } else if self.cursor.position.line > 0 {
            self.cursor.position.line -= 1;
            self.cursor.position.col = self.line_len(self.cursor.position.line);
        }
    }

    // Move cursor right, optionally extending selection
    pub fn move_right_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((_, end)) = self.cursor.selection_range() {
            // Collapse to end of selection, don't move further
            self.cursor.position = end;
            self.cursor.clear_selection();
            return;
        }

        let line_len = self.line_len(self.cursor.position.line);
        if self.cursor.position.col < line_len {
            self.cursor.position.col += 1;
        } else if self.cursor.position.line < self.rope.len_lines().saturating_sub(1) {
            self.cursor.position.line += 1;
            self.cursor.position.col = 0;
        }
    }

    // Move line to start, optionally extending selection
    pub fn move_to_line_start_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((start, _)) = self.cursor.selection_range() {
            // Collapse to start of selection first
            self.cursor.position = start;
            self.cursor.clear_selection();
        }
        self.cursor.position.col = 0;
    }

    // Move line to end, optionally extending selection
    pub fn move_to_line_end_with_selection(&mut self, extend: bool) {
        if extend {
            self.cursor.extend_selection();
        } else if let Some((_, end)) = self.cursor.selection_range() {
            // Collapse to end of selection first
            self.cursor.position = end;
            self.cursor.clear_selection();
        }
        self.cursor.position.col = self.line_len(self.cursor.position.line);
    }

    // Insert char, replacing selection if any
    pub fn insert_char_replacing_selection(&mut self, ch: char) {
        // Try to delete selection first (will do nothing if no selection)
        self.delete_selection();
        self.insert_char(ch);
    }

    // Insert str, replacing selection if any
    pub fn insert_str_replacing_selection(&mut self, text: &str) {
        // Try to delete selection first (will do nothing if no selection)
        self.delete_selection();
        self.insert_str(text);
    }

    // Get selection start and end positions
    pub fn selection_positions(&self) -> Option<(usize, usize, usize, usize)> {
        let (start, end) = self.cursor.selection_range()?;
        Some((start.line, start.col, end.line, end.col))
    }

    // Set cursor position directly (for mouse click)
    pub fn set_cursor_position(&mut self, line: usize, col: usize) {
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursor.position = crate::editor::cursor::Position::new(line, col);
        self.cursor.clear_selection();
    }

    // Start selection at current position (for mouse drag start)
    pub fn begin_selection(&mut self, line: usize, col: usize) {
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let col = col.min(self.line_len(line));
        self.cursor.position = crate::editor::cursor::Position::new(line, col);
        self.cursor.anchor = Some(self.cursor.position);
    }

    // Extend selection to position (for mouse drag)
    pub fn extend_selection_to(&mut self, line: usize, col: usize) {
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let col = col.min(self.line_len(line));
        // Keep anchor where it is, move position
        if self.cursor.anchor.is_none() {
            self.cursor.anchor = Some(self.cursor.position);
        }
        self.cursor.position = crate::editor::cursor::Position::new(line, col);
    }

    // Select word at position (for double-click)
    pub fn select_word_at(&mut self, line: usize, col: usize) {
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let line_text: String = self.rope.line(line).chars().collect();
        let line_len = self.line_len(line);

        if line_len == 0 {
            // Empty line, just position cursor
            self.set_cursor_position(line, 0);
            return;
        }

        let col = col.min(line_len.saturating_sub(1));

        // Find word boundaries
        let chars: Vec<char> = line_text.chars().collect();

        // Check if we clicked on a word character
        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        if col >= chars.len() || !is_word_char(chars[col]) {
            // Clicked on non-word char, just position cursor
            self.set_cursor_position(line, col);
            return;
        }

        // Find start of word
        let mut start = col;
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }

        // Find end of word
        let mut end = col;
        while end < chars.len() && is_word_char(chars[end]) {
            end += 1;
        }

        // Set selection
        self.cursor.anchor = Some(crate::editor::cursor::Position::new(line, start));
        self.cursor.position = crate::editor::cursor::Position::new(line, end);
    }

    // Select word backward from current position (for Shift+Down)
    pub fn select_word_backward(&mut self) {
        // Set anchor if not already set
        if self.cursor.anchor.is_none() {
            self.cursor.anchor = Some(self.cursor.position);
        }

        let line = self.cursor.position.line;
        let col = self.cursor.position.col;
        let line_text: String = self.rope.line(line).chars().collect();
        let chars: Vec<char> = line_text.chars().collect();

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        if col == 0 {
            // At start of line, nothing to select backward
            return;
        }

        let mut pos = col;

        // Skip whitespace/non-word chars going backward
        while pos > 0 && !is_word_char(chars[pos - 1]) {
            pos -= 1;
        }

        // Select to start of previous word
        while pos > 0 && is_word_char(chars[pos - 1]) {
            pos -= 1;
        }

        self.cursor.position = crate::editor::cursor::Position::new(line, pos);
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}