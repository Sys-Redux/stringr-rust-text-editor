// Snapshot based approach to undo/redo functionality
// Each edit action stores a snapshot of the buffer state

use ropey::Rope;
use super::super::editor::cursor::Position;

// Represents snapshot of editor state
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    pub rope: Rope,                // Text content
    pub cursor_position: Position, // Cursor position
}

// Types of edit actions
#[derive(Debug, Clone)]
pub enum EditAction {
    Insert { position: usize, text: String },
    Delete { start: usize, end: usize, deleted_text: String },
    Replace { start: usize, end: usize, old_text: String, new_text: String },
}

// History manager for undo/redo
#[derive(Debug, Clone)]
pub struct History {
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
    max_size: usize,
}

impl History {
    // Create new history manager
    pub fn new() -> Self {
        Self::with_max_size(100)
    }

    // Create history manager with specified max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_size),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    // Save state before an edit
    // Call BEFORE changes to the buffer
    pub fn save_state(&mut self, rope: &Rope, cursor_position: &Position) {
        // Clear stack when edit made
        self.redo_stack.clear();

        // Create snapshot
        let snapshot = EditorSnapshot {
            rope: rope.clone(),
            cursor_position,
        };

        // Push to undo stack
        self.undo_stack.push(snapshot);

        // Enforce max size
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    // Undo last action returning previous state
    pub fn undo(&mut self, current_rope: &Rope, current_position: &Position) ->
        Option<EditorSnapshot> {
        if let Some(previous) = self.undo_stack.pop() {
            // Save current state to redo stack
            self.redo_stack.push(EditorSnapshot {
                rope: current_rope.clone(),
                cursor_position: current_position,
            });
            Some(previous)
        } else {
            None
        }
    }

    // Redo last undone action returning next state
    pub fn redo(&mut self, current_rope: &Rope, current_position: &Position) ->
        Option<EditorSnapshot> {
        if let Some(next) = self.redo_stack.pop() {
            // Save state to undo stack
            self.undo_stack.push(EditorSnapshot {
                rope: current_rope.clone(),
                cursor_position: current_position,
            });
            Some(next)
        } else {
            None
        }
    }

    // Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    // Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    // Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    // Get number of undo levels possible
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    // Get number of redo levels possible
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}