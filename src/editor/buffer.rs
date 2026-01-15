// Text buffer implementation using Ropey

// Allow unused - these are API methods for future use
#![allow(dead_code)]

use ropey::Rope;
use std::path::PathBuf;
use super::cursor::Cursor;

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
}

impl Buffer {
    // Create new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            cursor: Cursor::new(),
            dirty: false,
            path: None,
        }
    }

    // Create a buffer w/ initial text
    pub fn new_with_text(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            cursor: Cursor::new(),
            dirty: false,
            path: None,
        }
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

        self.rope.remove(idx..idx + 1);
        self.dirty = true;
        // Cursor stays in place
    }

    /// Move cursor up one line
    pub fn move_up(&mut self) {
        if self.cursor.position.line > 0 {
            self.cursor.position.line -= 1;
            // Clamp column to line length
            let max_col = self.line_len(self.cursor.position.line);
            self.cursor.position.col = self.cursor.position.col.min(max_col);
        }
    }

    /// Move cursor down one line
    pub fn move_down(&mut self) {
        if self.cursor.position.line < self.rope.len_lines().saturating_sub(1) {
            self.cursor.position.line += 1;
            // Clamp column to line length
            let max_col = self.line_len(self.cursor.position.line);
            self.cursor.position.col = self.cursor.position.col.min(max_col);
        }
    }

    /// Move cursor left one character
    pub fn move_left(&mut self) {
        if self.cursor.position.col > 0 {
            self.cursor.position.col -= 1;
        } else if self.cursor.position.line > 0 {
            // Move to end of previous line
            self.cursor.position.line -= 1;
            self.cursor.position.col = self.line_len(self.cursor.position.line);
        }
    }

    /// Move cursor right one character
    pub fn move_right(&mut self) {
        let line_len = self.line_len(self.cursor.position.line);
        if self.cursor.position.col < line_len {
            self.cursor.position.col += 1;
        } else if self.cursor.position.line < self.rope.len_lines().saturating_sub(1) {
            // Move to start of next line
            self.cursor.position.line += 1;
            self.cursor.position.col = 0;
        }
    }

    /// Move cursor to start of current line
    pub fn move_to_line_start(&mut self) {
        self.cursor.position.col = 0;
    }

    /// Move cursor to end of current line
    pub fn move_to_line_end(&mut self) {
        self.cursor.position.col = self.line_len(self.cursor.position.line);
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
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}