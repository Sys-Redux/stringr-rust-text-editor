// Cursor position and movement logic

// Allow unused - these are API methods for future use
#![allow(dead_code)]

/// A position in the text buffer (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// Line index (0-based)
    pub line: usize,
    /// Column index (0-based, in characters)
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// Cursor state including position and optional selection
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    /// Current cursor position
    pub position: Position,
    /// Selection anchor (if selecting)
    pub anchor: Option<Position>,
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get the selection range (start, end) if any
    pub fn selection_range(&self) -> Option<(Position, Position)> {
        self.anchor.map(|anchor| {
            if self.position.line < anchor.line
               || (self.position.line == anchor.line && self.position.col < anchor.col) {
                (self.position, anchor)
            } else {
                (anchor, self.position)
            }
        })
    }

    /// Clear any selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }
}