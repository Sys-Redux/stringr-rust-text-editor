// Main application component and state management

use dioxus::prelude::*;
use std::path::PathBuf;
use crate::editor::Buffer;
use crate::shortcuts::{self, ShortcutAction};
use crate::ui::{StatusBar, TitleBar, FileExplorer};
use crate::workspace::FileTree;
use crate::file;

// Constants for mouse position calculation
const CHAR_WIDTH: f64 = 8.4;  // Approximate width of monospace char at 14px
const LINE_HEIGHT: f64 = 22.4; // 14px * 1.6 line-height
const EDITOR_PADDING: f64 = 16.0; // 1rem padding

// Main application component
pub fn app() -> Element {
    // Initialize with an empty buffer
    let mut buffer = use_signal(Buffer::new);

    // File tree state for the explorer
    let mut file_tree = use_signal(FileTree::new);

    // Explorer panel visibility
    let show_explorer = use_signal(|| true);

    // Track if editor is focused
    let mut is_focused = use_signal(|| false);

    // Track mouse drag state for selection
    let mut is_dragging = use_signal(|| false);

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
                if buffer.write().undo() {
                    tracing::info!("Undo performed");
                } else {
                    tracing::info!("Nothing to undo");
                }
            }
            ShortcutAction::Redo => {
                evt.prevent_default();
                if buffer.write().redo() {
                    tracing::info!("Redo performed");
                } else {
                    tracing::info!("Nothing to redo");
                }
            }
            ShortcutAction::SelectAll => {
                evt.prevent_default();
                buffer.write().select_all();
                tracing::info!("Selected all text");
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

    // Helper to convert mouse coordinates to line/col
    let coords_to_position = |x: f64, y: f64| -> (usize, usize) {
        let line = ((y - EDITOR_PADDING) / LINE_HEIGHT).floor().max(0.0) as usize;
        let col = ((x - EDITOR_PADDING) / CHAR_WIDTH).round().max(0.0) as usize;
        (line, col)
    };

    // Mouse down - start selection or position cursor
    let onmousedown = move |evt: Event<MouseData>| {
        let coords = evt.element_coordinates();
        let (line, col) = coords_to_position(coords.x, coords.y);

        // Start potential drag selection
        is_dragging.set(true);
        buffer.write().begin_selection(line, col);
    };

    // Mouse move - extend selection if dragging
    let onmousemove = move |evt: Event<MouseData>| {
        if is_dragging() {
            let coords = evt.element_coordinates();
            let (line, col) = coords_to_position(coords.x, coords.y);
            buffer.write().extend_selection_to(line, col);
        }
    };

    // Mouse up - end drag
    let onmouseup = move |_evt: Event<MouseData>| {
        if is_dragging() {
            is_dragging.set(false);
            // If selection is empty (anchor == position), it was just a click, clear selection
            if buffer.read().selection_positions().is_none() {
                buffer.write().clear_selection();
            }
        }
    };

    // Double click - select word
    let ondoubleclick = move |evt: Event<MouseData>| {
        let coords = evt.element_coordinates();
        let (line, col) = coords_to_position(coords.x, coords.y);
        buffer.write().select_word_at(line, col);
    };

    // Handle opening a folder for the file explorer
    let handle_open_folder = move |_| {
        spawn(async move {
            if let Some(folder) = rfd::AsyncFileDialog::new()
                .pick_folder()
                .await
            {
                let path = folder.path().to_path_buf();
                match file::scan_directory(&path) {
                    Ok(tree) => {
                        file_tree.set(tree);
                        tracing::info!("Opened folder: {}", path.display());
                    }
                    Err(e) => {
                        tracing::error!("Failed to open folder: {}", e);
                    }
                }
            }
        });
    };

    // Handle file selection from explorer
    let handle_file_open = move |path: PathBuf| {
        spawn(async move {
            match file::read_file(&path).await {
                Ok(content) => {
                    buffer.write().load_content(path.clone(), content);
                    tracing::info!("Opened file from explorer: {}", path.display());
                }
                Err(e) => {
                    tracing::error!("Failed to open file: {}", e);
                }
            }
        });
    };

    // Get cursor position for rendering
    let cursor_line_idx = buffer.read().cursor_line();
    let cursor_col_idx = buffer.read().cursor_col();
    let selection = buffer.read().selection_positions();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        // Window resize handles (invisible hit areas at edges/corners)
        WindowResizeHandles {}

        div {
            class: "flex flex-col h-screen bg-background text-text font-mono",

            TitleBar {
                filename: buffer.read().filename(),
                is_dirty: buffer.read().is_dirty(),
            }

            // Main content area with explorer + editor
            div {
                class: "flex flex-1 overflow-hidden",

                // File Explorer Panel
                FileExplorer {
                    tree: file_tree,
                    on_file_open: handle_file_open,
                    on_open_folder: handle_open_folder,
                    is_visible: show_explorer(),
                }

                // Editor area
                div {
                    class: "flex-1 flex flex-col m-2 border-brutal border-border overflow-hidden",

                    div {
                        class: "editor-container",
                    tabindex: 0,
                    onkeydown,
                    onfocus,
                    onblur,
                    onmousedown,
                    onmousemove,
                    onmouseup,
                    ondoubleclick,

                    if is_empty() {
                        // Line number gutter (just line 1 for empty)
                        div {
                            class: "line-number-gutter",
                            div {
                                class: "line-number active",
                                "1"
                            }
                        }
                        // Editor content
                        div {
                            class: "editor-content",
                            div {
                                class: "editor-line active",
                                span {
                                    class: if is_focused() { "cursor-blink" } else { "cursor-static" },
                                }
                                span {
                                    class: "placeholder-text",
                                    "Start typing..."
                                }
                            }
                        }
                    } else {
                        // Line number gutter
                        div {
                            class: "line-number-gutter",
                            for (line_idx, _line) in buffer.read().lines().enumerate() {
                                div {
                                    key: "ln-{line_idx}",
                                    class: if line_idx == cursor_line_idx { "line-number active" } else { "line-number" },
                                    "{line_idx + 1}"
                                }
                            }
                        }
                        // Editor content
                        div {
                            class: "editor-content",
                            for (line_idx, line) in buffer.read().lines().enumerate() {
                                div {
                                    key: "{line_idx}",
                                    class: if line_idx == cursor_line_idx { "editor-line active" } else { "editor-line" },

                                    // Render line w/ cursor and/or selection
                                    {render_line(
                                        line_idx,
                                        &line,
                                        cursor_line_idx,
                                        cursor_col_idx,
                                        selection,
                                        is_focused()
                                    )}
                                }
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
    let extending = modifiers.shift();

    match key {
        // Character input (only when Ctrl/Alt not pressed)
        Key::Character(ref c) if !modifiers.ctrl() && !modifiers.alt() => {
            if let Some(ch) = c.chars().next() {
                buffer.write().insert_char_replacing_selection(ch);
            }
        }

        Key::Backspace => {
            let mut buf = buffer.write();
            // Try to delete selection first, fall back to single char
            if buf.delete_selection().is_none() {
                buf.delete_backward();
            }
        }

        Key::Delete => {
            let mut buf = buffer.write();
            // Try to delete selection first, fall back to single char
            if buf.delete_selection().is_none() {
                buf.delete_forward();
            }
        }

        Key::Enter => buffer.write().insert_char_replacing_selection('\n'),

        Key::ArrowUp => buffer.write().move_up_with_selection(extending),
        Key::ArrowDown => {
            if extending {
                // Shift+Down: select word backward
                buffer.write().select_word_backward();
            } else {
                buffer.write().move_down_with_selection(false);
            }
        }
        Key::ArrowLeft => buffer.write().move_left_with_selection(extending),
        Key::ArrowRight => buffer.write().move_right_with_selection(extending),

        Key::Home => buffer.write().move_to_line_start_with_selection(extending),
        Key::End => buffer.write().move_to_line_end_with_selection(extending),

        Key::Tab => buffer.write().insert_char_replacing_selection('\t'),

        _ => {}
    }
}

// Render helper function
fn render_line(
    line_idx: usize,
    line: &str,
    cursor_line: usize,
    cursor_col: usize,
    selection: Option<(usize, usize, usize, usize)>,
    is_focused: bool,
) -> Element {
    let cursor_class = if is_focused { "cursor-blink" } else { "cursor-static" };

    // Check if this line has selection
    let (sel_start_line, sel_start_col, sel_end_line, sel_end_col) =
        selection.unwrap_or((usize::MAX, 0, 0, 0));

    let has_selection_on_line = selection.is_some()
    && line_idx >= sel_start_line
    && line_idx <= sel_end_line;

    if !has_selection_on_line && line_idx != cursor_line {
        // Simple case: no selection or cursor on this line
        return rsx! { "{line}" };
    }

    if has_selection_on_line {
        // Calculate selection bounds
        let line_start = if line_idx ==
            sel_start_line { sel_start_col } else { 0 };

        let line_end = if line_idx ==
            sel_end_line { sel_end_col } else { line.chars().count() };

        let before: String = line.chars().take(line_start).collect();
        let selected: String = line.chars().skip(line_start).take(line_end - line_start).collect();
        let after: String = line.chars().skip(line_end).collect();

        // Also render cursor if on this line
        if line_idx == cursor_line {
            // Within range
            rsx! {
                span { "{before}" }
                span { class: "bg-primary/30 text/text", "{selected}" }
                span { class: "{cursor_class}", }
                span { "{after}" }
            }
        } else {
            rsx! {
                span { "{before}" }
                span { class: "bg-primary/30 text/text", "{selected}" }
                span { "{after}" }
            }
        }
    } else {
        // Only cursor, no selection
        let before: String = line.chars().take(cursor_col).collect();
        let after: String = line.chars().skip(cursor_col).collect();

        rsx! {
            span { "{before}" }
            span { class: "{cursor_class}", }
            span { "{after}" }
        }
    }
}

/// Resize direction for window drag-resize
#[derive(Clone, Copy)]
enum ResizeDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

/// Invisible resize handles at window edges and corners
#[component]
fn WindowResizeHandles() -> Element {
    // Size of the resize hit area in pixels
    const EDGE_SIZE: i32 = 5;
    const CORNER_SIZE: i32 = 10;

    // Helper to start resize drag
    let start_resize = move |direction: ResizeDirection| {
        move |_evt: MouseEvent| {
            use dioxus::desktop::tao::window::ResizeDirection as TaoDirection;

            let dir = match direction {
                ResizeDirection::North => TaoDirection::North,
                ResizeDirection::South => TaoDirection::South,
                ResizeDirection::East => TaoDirection::East,
                ResizeDirection::West => TaoDirection::West,
                ResizeDirection::NorthEast => TaoDirection::NorthEast,
                ResizeDirection::NorthWest => TaoDirection::NorthWest,
                ResizeDirection::SouthEast => TaoDirection::SouthEast,
                ResizeDirection::SouthWest => TaoDirection::SouthWest,
            };

            // Get the webview window and start drag resize
            let window = dioxus::desktop::window();
            let _ = window.drag_resize_window(dir);
        }
    };

    rsx! {
        // Corner handles (higher z-index, larger hit area)
        // Top-left corner
        div {
            class: "resize-handle resize-nw",
            style: "position:fixed;top:0;left:0;width:{CORNER_SIZE}px;height:{CORNER_SIZE}px;cursor:nwse-resize;z-index:9999;",
            onmousedown: start_resize(ResizeDirection::NorthWest),
        }
        // Top-right corner
        div {
            class: "resize-handle resize-ne",
            style: "position:fixed;top:0;right:0;width:{CORNER_SIZE}px;height:{CORNER_SIZE}px;cursor:nesw-resize;z-index:9999;",
            onmousedown: start_resize(ResizeDirection::NorthEast),
        }
        // Bottom-left corner
        div {
            class: "resize-handle resize-sw",
            style: "position:fixed;bottom:0;left:0;width:{CORNER_SIZE}px;height:{CORNER_SIZE}px;cursor:nesw-resize;z-index:9999;",
            onmousedown: start_resize(ResizeDirection::SouthWest),
        }
        // Bottom-right corner
        div {
            class: "resize-handle resize-se",
            style: "position:fixed;bottom:0;right:0;width:{CORNER_SIZE}px;height:{CORNER_SIZE}px;cursor:nwse-resize;z-index:9999;",
            onmousedown: start_resize(ResizeDirection::SouthEast),
        }

        // Edge handles
        // Top edge
        div {
            class: "resize-handle resize-n",
            style: "position:fixed;top:0;left:{CORNER_SIZE}px;right:{CORNER_SIZE}px;height:{EDGE_SIZE}px;cursor:ns-resize;z-index:9998;",
            onmousedown: start_resize(ResizeDirection::North),
        }
        // Bottom edge
        div {
            class: "resize-handle resize-s",
            style: "position:fixed;bottom:0;left:{CORNER_SIZE}px;right:{CORNER_SIZE}px;height:{EDGE_SIZE}px;cursor:ns-resize;z-index:9998;",
            onmousedown: start_resize(ResizeDirection::South),
        }
        // Left edge
        div {
            class: "resize-handle resize-w",
            style: "position:fixed;left:0;top:{CORNER_SIZE}px;bottom:{CORNER_SIZE}px;width:{EDGE_SIZE}px;cursor:ew-resize;z-index:9998;",
            onmousedown: start_resize(ResizeDirection::West),
        }
        // Right edge
        div {
            class: "resize-handle resize-e",
            style: "position:fixed;right:0;top:{CORNER_SIZE}px;bottom:{CORNER_SIZE}px;width:{EDGE_SIZE}px;cursor:ew-resize;z-index:9998;",
            onmousedown: start_resize(ResizeDirection::East),
        }
    }
}