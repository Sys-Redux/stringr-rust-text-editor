// Cursor position and movement logic

// Allow unused - these are API methods for future use
#![allow(dead_code)]

// A position in the text buffer (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    // Line index (0-based)
    pub line: usize,
    // Column index (0-based, in characters)
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    // Compare two positions
    pub fn cmp(&self, other: &Position) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }

    // Check if this position comes before another
    pub fn is_before(&self, other: &Position) -> bool {
        self.line < other.line || (self.line == other.line && self.col < other.col)
    }
}

// Selection range (start, end)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub head: Position,
}

impl Selection {
    pub fn new(anchor: Position, head: Position) -> Self {
        Self { anchor, head }
    }

    // Get normalized range
    pub fn normalized(&self) -> (Position, Position) {
        if self.anchor.is_before(&self.head) {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    pub fn start(&self) -> Position {
        self.normalized().0
    }

    pub fn end(&self) -> Position {
        self.normalized().1
    }
}

// Cursor state including position and optional selection
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    // Current cursor position
    pub position: Position,
    // Selection anchor (if selecting)
    pub anchor: Option<Position>,
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    // Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some() && self.anchor != Some(self.position)
    }

    // Get selection
    pub fn selection(&self) -> Option<Selection> {
        self.anchor.map(|anchor| Selection::new(anchor, self.position))
    }

    // Get the selection range (start, end) if any
    pub fn selection_range(&self) -> Option<(Position, Position)> {
        self.selection().map(|s| s.normalized())
    }

    // Start selection at current position
    pub fn start_selection(&mut self) {
        if self.anchor.is_none() {
            self.anchor = Some(self.position);
        }
    }

    // Extend current selection (or start one if none)
    pub fn extend_selection(&mut self) {
        if self.anchor.is_none() {
            self.anchor = Some(self.position);
        }
    }

    // Clear any selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    // Set position and optionally extend selection
    pub fn set_position(&mut self, pos: Position, extend: bool) {
        if extend {
            self.extend_selection();
        } else {
            self.clear_selection();
        }
        self.position = pos;
    }

    // Select all
    pub fn select_all(&mut self, end_position: Position) {
        self.anchor = Some(Position::new(0, 0));
        self.position = end_position;
    }

    // Collapse selection to cursor position
    pub fn collapse_to_position(&mut self) {
        self.anchor = None;
    }

    // Collapse selection to start
    pub fn collapse_to_start(&mut self) {
        if let Some((start, _)) = self.selection_range() {
            self.position = start;
        }
        self.anchor = None;
    }

    // Collapse selection to end
    pub fn collapse_to_end(&mut self) {
        if let Some((_, end)) = self.selection_range() {
            self.position = end;
        }
        self.anchor = None;
    }
}