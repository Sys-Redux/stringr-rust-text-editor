// Snapshot based approach to undo/redo functionality
// Groups edits by words like VS Code does

use ropey::Rope;
use crate::editor::cursor::Position;
use std::time::{Duration, Instant};

// Represents snapshot of editor state
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    pub rope: Rope,                // Text content
    pub cursor_position: Position, // Cursor position
}

// Types of edit actions (for grouping logic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditAction {
    Insert,
    Delete,
    Other,
}

// History manager for undo/redo with word-level grouping
#[derive(Debug, Clone)]
pub struct History {
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
    max_size: usize,
    // For grouping consecutive edits
    last_edit_type: Option<EditAction>,
    last_edit_time: Option<Instant>,
    pending_snapshot: Option<EditorSnapshot>,
    chars_since_snapshot: usize,
}

// Time threshold for grouping (300ms like VS Code)
const GROUP_TIMEOUT: Duration = Duration::from_millis(300);
// Max chars before forcing a snapshot (safety net)
const MAX_CHARS_PER_GROUP: usize = 50;

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
            last_edit_type: None,
            last_edit_time: None,
            pending_snapshot: None,
            chars_since_snapshot: 0,
        }
    }

    // Check if a character is a word boundary (triggers new undo group)
    fn is_word_boundary(ch: char) -> bool {
        ch.is_whitespace() || ch == '\n' || ch == '\t' ||
        matches!(ch, '.' | ',' | ';' | ':' | '!' | '?' | '(' | ')' |
                     '[' | ']' | '{' | '}' | '"' | '\'' | '/' | '\\' |
                     '+' | '-' | '*' | '=' | '<' | '>' | '&' | '|')
    }

    // Should we start a new undo group?
    fn should_start_new_group(&self, edit_type: EditAction, typed_char: Option<char>) -> bool {
        // Always start new group if:
        // 1. No previous edit
        if self.last_edit_type.is_none() {
            return true;
        }

        // 2. Edit type changed (e.g., insert -> delete)
        if self.last_edit_type != Some(edit_type) {
            return true;
        }

        // 3. Time gap exceeded (user paused typing)
        if let Some(last_time) = self.last_edit_time {
            if last_time.elapsed() > GROUP_TIMEOUT {
                return true;
            }
        }

        // 4. Word boundary character typed
        if let Some(ch) = typed_char {
            if Self::is_word_boundary(ch) {
                return true;
            }
        }

        // 5. Too many chars since last snapshot (safety)
        if self.chars_since_snapshot >= MAX_CHARS_PER_GROUP {
            return true;
        }

        false
    }

    // Save state before an edit (called by buffer)
    // For character inserts, pass the character for word boundary detection
    pub fn save_state_for_edit(
        &mut self,
        rope: &Rope,
        cursor_position: Position,
        edit_type: EditAction,
        typed_char: Option<char>,
    ) {
        let now = Instant::now();

        // Check if we should start a new undo group
        if self.should_start_new_group(edit_type, typed_char) {
            // Commit any pending snapshot first
            self.commit_pending();

            // Start new group - save current state as pending
            self.pending_snapshot = Some(EditorSnapshot {
                rope: rope.clone(),
                cursor_position,
            });
            self.chars_since_snapshot = 0;
        }

        // Update tracking
        self.last_edit_type = Some(edit_type);
        self.last_edit_time = Some(now);
        self.chars_since_snapshot += 1;

        // Clear redo stack on new edit
        self.redo_stack.clear();
    }

    // Legacy method for backwards compatibility
    pub fn save_state(&mut self, rope: &Rope, cursor_position: Position) {
        self.save_state_for_edit(rope, cursor_position, EditAction::Other, None);
    }

    // Commit pending snapshot to undo stack
    fn commit_pending(&mut self) {
        if let Some(snapshot) = self.pending_snapshot.take() {
            self.undo_stack.push(snapshot);

            // Enforce max size
            if self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
            }
        }
    }

    // Force commit (call before undo or when switching operations)
    pub fn force_commit(&mut self, rope: &Rope, cursor_position: Position) {
        // If we have uncommitted changes, commit the pending snapshot
        if self.pending_snapshot.is_some() {
            self.commit_pending();
        } else if self.chars_since_snapshot > 0 {
            // We have edits but no pending snapshot (edge case)
            // Create a snapshot of current state
            self.undo_stack.push(EditorSnapshot {
                rope: rope.clone(),
                cursor_position,
            });
            if self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
            }
        }

        // Reset grouping state
        self.last_edit_type = None;
        self.last_edit_time = None;
        self.chars_since_snapshot = 0;
    }

    // Undo last action returning previous state
    pub fn undo(&mut self, current_rope: &Rope, current_position: Position) -> Option<EditorSnapshot> {
        // First, commit any pending changes so current state is saved
        if self.pending_snapshot.is_some() || self.chars_since_snapshot > 0 {
            // Save current state to redo stack
            self.redo_stack.push(EditorSnapshot {
                rope: current_rope.clone(),
                cursor_position: current_position,
            });

            // Commit pending and pop the last snapshot
            self.commit_pending();
            self.last_edit_type = None;
            self.last_edit_time = None;
            self.chars_since_snapshot = 0;
        }

        if let Some(previous) = self.undo_stack.pop() {
            // Save current state to redo if we haven't already
            if self.redo_stack.last().map(|s| &s.rope) != Some(current_rope) {
                self.redo_stack.push(EditorSnapshot {
                    rope: current_rope.clone(),
                    cursor_position: current_position,
                });
            }
            Some(previous)
        } else {
            None
        }
    }

    // Redo last undone action returning next state
    pub fn redo(&mut self, current_rope: &Rope, current_position: Position) -> Option<EditorSnapshot> {
        // Reset grouping state
        self.last_edit_type = None;
        self.last_edit_time = None;
        self.pending_snapshot = None;
        self.chars_since_snapshot = 0;

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
        !self.undo_stack.is_empty() || self.pending_snapshot.is_some()
    }

    // Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    // Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_edit_type = None;
        self.last_edit_time = None;
        self.pending_snapshot = None;
        self.chars_since_snapshot = 0;
    }

    // Get number of undo levels possible
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len() + if self.pending_snapshot.is_some() { 1 } else { 0 }
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