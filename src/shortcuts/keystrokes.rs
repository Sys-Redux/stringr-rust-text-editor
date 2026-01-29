// Keyboard shortcut handling for Stringr

use dioxus::prelude::*;
use crate::editor::Buffer;
use crate::file;

// Actions that can be triggered by keyboard shortcuts
#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutAction {
    // File operations
    NewFile,
    OpenFile,
    SaveFile,

    // Edit operations
    Copy,
    Paste,
    Cut,
    Undo,
    Redo,
    SelectAll,

    // No action matched
    None,
}

// Parse keyboard event and return the corresponding shortcut action
pub fn parse_shortcut(key: &Key, modifiers: &Modifiers) -> ShortcutAction {
    // Only handle Ctrl shortcuts (not Alt)
    if !modifiers.ctrl() || modifiers.alt() {
        return ShortcutAction::None;
    }

    match key {
        Key::Character(ref c) => {
            match c.to_lowercase().as_str() {
                "n" => ShortcutAction::NewFile,
                "o" => ShortcutAction::OpenFile,
                "s" => ShortcutAction::SaveFile,
                "c" => ShortcutAction::Copy,
                "v" => ShortcutAction::Paste,
                "x" => ShortcutAction::Cut,
                "z" => {
                    if modifiers.shift() {
                        ShortcutAction::Redo
                    } else {
                        ShortcutAction::Undo
                    }
                }
                "y" => ShortcutAction::Redo,  // Alternative redo
                "a" => ShortcutAction::SelectAll,
                _ => ShortcutAction::None,
            }
        }
        _ => ShortcutAction::None,
    }
}

// Handle New File action
pub async fn handle_new_file(mut buffer: Signal<Buffer>) {
    let is_dirty = buffer.read().is_dirty();

    if is_dirty {
        let confirmed = rfd::AsyncMessageDialog::new()
            .set_title("Unsaved Changes")
            .set_description("Create a new file? Unsaved changes will be lost.")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show()
            .await;

        if confirmed == rfd::MessageDialogResult::Yes {
            *buffer.write() = Buffer::new();
        }
    } else {
        *buffer.write() = Buffer::new();
    }
}

// Handle Open File action
pub async fn handle_open_file(mut buffer: Signal<Buffer>) {
    if let Some(path) = rfd::AsyncFileDialog::new()
        .add_filter("Text Files", &["txt", "md", "rs", "toml", "json", "css", "html"])
        .add_filter("All Files", &["*"])
        .pick_file()
        .await
    {
        let path_buf = path.path().to_path_buf();

        match file::read_file(&path_buf).await {
            Ok(content) => {
                buffer.write().load_content(path_buf.clone(), content);
                tracing::info!("Opened file: {}", path_buf.display());
            }
            Err(e) => {
                tracing::error!("Failed to open file: {}", e);
                // Show error dialog
                rfd::AsyncMessageDialog::new()
                    .set_title("Error Opening File")
                    .set_description(&format!("Could not open file: {}", e))
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show()
                    .await;
            }
        }
    }
}

// Handle save file action
pub async fn handle_save_file(mut buffer: Signal<Buffer>) {
    // Get path if it exists (clone to avoid holding read lock)
    let existing_path = buffer.read().path().cloned();

    if let Some(path) = existing_path {
        // File has a path - save directly
        let content = buffer.read().text();

        match file::write_file(&path, &content).await {
            Ok(()) => {
                buffer.write().mark_saved();
                tracing::info!("Saved file: {}", path.display());
            }
            Err(e) => {
                tracing::error!("Failed to save file: {}", e);
                rfd::AsyncMessageDialog::new()
                    .set_title("Error Saving File")
                    .set_description(&format!("Could not save file: {}", e))
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show()
                    .await;
            }
        }
    } else {
        // No path - show Save As dialog
        if let Some(path) = rfd::AsyncFileDialog::new()
            .add_filter("Text Files", &["txt", "md"])
            .save_file()
            .await
        {
            let path_buf = path.path().to_path_buf();
            let content = buffer.read().text();

            match file::write_file(&path_buf, &content).await {
                Ok(()) => {
                    buffer.write().set_path(path_buf.clone());
                    buffer.write().mark_saved();
                    tracing::info!("Saved file: {}", path_buf.display());
                }
                Err(e) => {
                    tracing::error!("Failed to save file: {}", e);
                    rfd::AsyncMessageDialog::new()
                        .set_title("Error Saving File")
                        .set_description(&format!("Could not save file: {}", e))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show()
                        .await;
                }
            }
        }
    }
}

// Handle copy action using arboard clipboard
pub fn handle_copy(buffer: &Buffer) {
    // Get selected text, or fall back to current line
    let text_to_copy = if let Some(selected) = buffer.selected_text() {
        selected
    } else {
        // No selection - copy current line
        let line_idx = buffer.cursor_line();
        let lines: Vec<String> = buffer.lines().collect();
        lines.get(line_idx).cloned().unwrap_or_default()
    };

    if text_to_copy.is_empty() {
        return;
    }

    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(&text_to_copy) {
                tracing::error!("Failed to copy to clipboard: {}", e);
            } else {
                tracing::info!("Copied {} characters to clipboard", text_to_copy.len());
            }
        }
        Err(e) => {
            tracing::error!("Failed to access clipboard: {}", e);
        }
    }
}

// Handle paste action using arboard clipboard
pub fn handle_paste(buffer: &mut Buffer) {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.get_text() {
                Ok(text) => {
                    // Use the selection-aware insert
                    buffer.insert_str_replacing_selection(&text);
                    tracing::info!("Pasted {} characters", text.len());
                }
                Err(e) => {
                    tracing::error!("Failed to get clipboard content: {}", e);
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to access clipboard: {}", e);
        }
    }
}

// Handle cut action (copy + delete)
pub fn handle_cut(buffer: &mut Buffer) {
    // Get text to cut (selection or current line)
    let text_to_cut = if buffer.has_selection() {
        buffer.selected_text()
    } else {
        // No selection - we'll cut the current line
        let line_idx = buffer.cursor_line();
        let lines: Vec<String> = buffer.lines().collect();
        lines.get(line_idx).cloned()
    };

    if let Some(text) = text_to_cut {
        // Copy to clipboard first
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(&text) {
                    tracing::error!("Failed to copy to clipboard: {}", e);
                    return;
                }
            }
            Err(e) => {
                tracing::error!("Failed to access clipboard: {}", e);
                return;
            }
        }

        // Delete the selection
        if buffer.has_selection() {
            buffer.delete_selection();
            tracing::info!("Cut {} characters", text.len());
        } else {
            // TODO: Delete entire line when no selection
            tracing::info!("Cut line to clipboard (full line delete not yet implemented)");
        }
    }
}