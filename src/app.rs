//! Main application component and state management

use dioxus::prelude::*;
use crate::editor::Buffer;
use crate::ui::StatusBar;

/// Main application component
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

        match key {
            // Character input
            Key::Character(ref c) if !modifiers.ctrl() && !modifiers.alt() => {
                if let Some(ch) = c.chars().next() {
                    buffer.write().insert_char(ch);
                }
            }

            // Backspace
            Key::Backspace => {
                buffer.write().delete_backward();
            }

            // Delete
            Key::Delete => {
                buffer.write().delete_forward();
            }

            // Enter
            Key::Enter => {
                buffer.write().insert_char('\n');
            }

            // Arrow keys
            Key::ArrowUp => buffer.write().move_up(),
            Key::ArrowDown => buffer.write().move_down(),
            Key::ArrowLeft => buffer.write().move_left(),
            Key::ArrowRight => buffer.write().move_right(),

            // Home/End
            Key::Home => buffer.write().move_to_line_start(),
            Key::End => buffer.write().move_to_line_end(),

            // Tab
            Key::Tab => {
                evt.prevent_default();
                buffer.write().insert_char('\t');
            }

            _ => {}
        }
    };

    // Handle focus
    let onfocus = move |_| {
        is_focused.set(true);
    };

    let onblur = move |_| {
        is_focused.set(false);
    };

    // Get cursor position for rendering
    let cursor_line_idx = buffer.read().cursor_line();
    let cursor_col_idx = buffer.read().cursor_col();

    rsx! {
        // Link to Tailwind CSS (compiled by Dioxus CLI)
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        // Main container with neo-brutalist styling using Tailwind classes
        div {
            class: "flex flex-col h-screen bg-background text-text font-mono",

            // Editor area
            div {
                class: "flex-1 flex flex-col m-2 border-brutal border-border overflow-hidden",

                // Editable content area
                div {
                    class: "editor-view flex-1 cursor-text whitespace-pre-wrap focus:border-primary focus:outline-none",
                    tabindex: 0,
                    onkeydown,
                    onfocus,
                    onblur,

                    // Show placeholder when empty
                    if is_empty() {
                        div {
                            class: "placeholder-text absolute",
                            "Start typing..."
                        }
                        // Still show cursor even when empty
                        span {
                            class: if is_focused() { "cursor-blink" } else { "cursor-static" },
                        }
                    } else {
                        // Render each line with cursor
                        for (line_idx, line) in buffer.read().lines().enumerate() {
                            div {
                                key: "{line_idx}",
                                class: "editor-line",

                                if line_idx == cursor_line_idx {
                                    // Line with cursor - split into before and after
                                    span { "{line.chars().take(cursor_col_idx).collect::<String>()}" }
                                    span {
                                        class: if is_focused() { "cursor-blink" } else { "cursor-static" },
                                    }
                                    span { "{line.chars().skip(cursor_col_idx).collect::<String>()}" }
                                } else {
                                    // Regular line without cursor
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }

            // Status bar
            StatusBar {
                line: cursor_line(),
                column: cursor_col(),
                total_lines: line_count(),
            }
        }
    }
}