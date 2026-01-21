// Main application component and state management

use dioxus::prelude::*;
use crate::editor::Buffer;
use crate::shortcuts::{self, ShortcutAction};
use crate::ui::{StatusBar, TitleBar};

// Main application component
pub fn app() -> Element {
    // Initialize with an empty buffer
    let mut buffer = use_signal(Buffer::new);

    // Track if editor is focused
    let mut is_focused = use_signal(|| false);

    // Track cursor position for status bar
    let cursor_line = use_memo(move || buffer.read().cursor_line() + 1);
    let cursor_col = use_memo(move || buffer.read().cursor_col() + 1);
    let line_count = use_memo(move || buffer.read().line_count().max(1));

    // Check if buffer is empty for placeholder
    let is_empty = use_memo(move || buffer.read().is_empty());

    // Handle keyboard input
    let onkeydown = move |evt: Event<KeyboardData>| {
        let key = evt.key();
        let modifiers = evt.modifiers();

        // First, check for keyboard shortcuts
        let action = shortcuts::parse_shortcut(&key, &modifiers);

        match action {
            ShortcutAction::NewFile => {
                evt.prevent_default();
                let buf = buffer;
                spawn(async move { shortcuts::handle_new_file(buf).await });
            }
            ShortcutAction::OpenFile => {
                evt.prevent_default();
                let buf = buffer;
                spawn(async move { shortcuts::handle_open_file(buf).await });
            }
            ShortcutAction::SaveFile => {
                evt.prevent_default();
                let buf = buffer;
                spawn(async move { shortcuts::handle_save_file(buf).await });
            }
            ShortcutAction::Copy => {
                evt.prevent_default();
                shortcuts::handle_copy(&buffer.read());
            }
            ShortcutAction::Paste => {
                evt.prevent_default();
                shortcuts::handle_paste(&mut buffer.write());
            }
            ShortcutAction::Cut => {
                evt.prevent_default();
                shortcuts::handle_cut(&mut buffer.write());
            }
            ShortcutAction::Undo => {
                evt.prevent_default();
                // TODO: Implement undo
                tracing::info!("Undo (not yet implemented)");
            }
            ShortcutAction::Redo => {
                evt.prevent_default();
                // TODO: Implement redo
                tracing::info!("Redo (not yet implemented)");
            }
            ShortcutAction::SelectAll => {
                evt.prevent_default();
                // TODO: Implement select all
                tracing::info!("Select All (not yet implemented)");
            }
            ShortcutAction::None => {
                // Not a shortcut - handle as regular input
                handle_text_input(&key, &modifiers, &mut buffer);
            }
        }
    };

    // Handle focus
    let onfocus = move |_| is_focused.set(true);
    let onblur = move |_| is_focused.set(false);

    // Get cursor position for rendering
    let cursor_line_idx = buffer.read().cursor_line();
    let cursor_col_idx = buffer.read().cursor_col();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        div {
            class: "flex flex-col h-screen bg-background text-text font-mono",

            TitleBar {
                filename: buffer.read().filename(),
                is_dirty: buffer.read().is_dirty(),
            }

            div {
                class: "flex-1 flex flex-col m-2 border-brutal border-border overflow-hidden",

                div {
                    class: "editor-view flex-1 cursor-text whitespace-pre-wrap focus:border-primary focus:outline-none",
                    tabindex: 0,
                    onkeydown,
                    onfocus,
                    onblur,

                    if is_empty() {
                        div {
                            class: "placeholder-text absolute",
                            "Start typing..."
                        }
                        span {
                            class: if is_focused() { "cursor-blink" } else { "cursor-static" },
                        }
                    } else {
                        for (line_idx, line) in buffer.read().lines().enumerate() {
                            div {
                                key: "{line_idx}",
                                class: "editor-line",

                                if line_idx == cursor_line_idx {
                                    span { "{line.chars().take(cursor_col_idx).collect::<String>()}" }
                                    span {
                                        class: if is_focused() { "cursor-blink" } else { "cursor-static" },
                                    }
                                    span { "{line.chars().skip(cursor_col_idx).collect::<String>()}" }
                                } else {
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }

            StatusBar {
                line: cursor_line(),
                column: cursor_col(),
                total_lines: line_count(),
            }
        }
    }
}

/// Handle regular text input (non-shortcut keys)
fn handle_text_input(key: &Key, modifiers: &Modifiers, buffer: &mut Signal<Buffer>) {
    match key {
        // Character input (only when Ctrl/Alt not pressed)
        Key::Character(ref c) if !modifiers.ctrl() && !modifiers.alt() => {
            if let Some(ch) = c.chars().next() {
                buffer.write().insert_char(ch);
            }
        }

        Key::Backspace => buffer.write().delete_backward(),
        Key::Delete => buffer.write().delete_forward(),
        Key::Enter => buffer.write().insert_char('\n'),

        Key::ArrowUp => buffer.write().move_up(),
        Key::ArrowDown => buffer.write().move_down(),
        Key::ArrowLeft => buffer.write().move_left(),
        Key::ArrowRight => buffer.write().move_right(),

        Key::Home => buffer.write().move_to_line_start(),
        Key::End => buffer.write().move_to_line_end(),

        Key::Tab => buffer.write().insert_char('\t'),

        _ => {}
    }
}