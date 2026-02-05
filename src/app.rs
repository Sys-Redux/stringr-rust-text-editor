// Main application component and state management

use dioxus::prelude::*;
use std::path::PathBuf;
use crate::editor::Buffer;
use crate::shortcuts::{self, ShortcutAction};
use crate::ui::{StatusBar, TitleBar, FileExplorer, ActivityBar, ActivityPanel, SearchPanel};
use crate::workspace::FileTree;
use crate::search::SearchState;
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

    // Activity bar panel state - which sidebar panel is active (None = collapsed)
    let mut active_panel = use_signal(|| Some(ActivityPanel::Files));

    // Explorer panel visibility - derived from active panel
    let show_explorer = active_panel().map_or(false, |p| p == ActivityPanel::Files);
    let show_search_panel = active_panel().map_or(false, |p| p == ActivityPanel::Search);

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

    // Search state
    let mut search_state = use_signal(SearchState::new);

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
            ShortcutAction::Find => {
                evt.prevent_default();
                search_state.write().open();
                tracing::info!("Opened search");
            }
            ShortcutAction::Replace => {
                evt.prevent_default();
                search_state.write().open_replace();
                tracing::info!("Opened search & replace");
            }
            ShortcutAction::FindNext => {
                evt.prevent_default();
                search_state.write().find_next();
            }
            ShortcutAction::FindPrevious => {
                evt.prevent_default();
                search_state.write().find_previous();
            }
            ShortcutAction::CloseSearch => {
                evt.prevent_default();
                if search_state.read().is_open {
                    search_state.write().close();
                }
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

    // Handle activity bar panel selection (toggle behavior)
    let handle_panel_select = move |panel: ActivityPanel| {
        // If clicking the already-active panel, collapse sidebar; otherwise switch to it
        if active_panel() == Some(panel) {
            active_panel.set(None);
        } else {
            active_panel.set(Some(panel));
        }
    };

    // Get cursor position for rendering - read directly from buffer for reactivity
    // Note: We'll read these inside the rsx! to ensure proper subscription

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        // Window resize handles (invisible hit areas at edges/corners)
        WindowResizeHandles {}

        div {
            class: "flex flex-col h-screen bg-background text-text font-mono",

            // Title bar with integrated search
            TitleBar {
                filename: buffer.read().filename(),
                is_dirty: buffer.read().is_dirty(),
                search_state: search_state,
                buffer: buffer,
            }

            // Main content area with activity bar + explorer + editor
            div {
                class: "flex flex-1 overflow-hidden",

                // Activity Bar (leftmost, VS Code style)
                ActivityBar {
                    active_panel: active_panel,
                    on_panel_select: handle_panel_select,
                }

                // File Explorer Panel
                FileExplorer {
                    tree: file_tree,
                    on_file_open: handle_file_open,
                    on_open_folder: handle_open_folder,
                    is_visible: show_explorer,
                }

                // Search Panel
                SearchPanel {
                    is_visible: show_search_panel,
                    search_state: search_state,
                    buffer: buffer,
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

                    if is_empty() {
                        // Line number gutter (just line 1 for empty)
                        div {
                            class: "line-number-gutter",
                            div {
                                class: "line-number active",
                                "1"
                            }
                        }
                        // Editor content - mouse events attached here for accurate coordinates
                        div {
                            class: "editor-content",
                            onmousedown,
                            onmousemove,
                            onmouseup,
                            ondoubleclick,
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
                            for line_idx in 0..buffer.read().line_count() {
                                div {
                                    key: "ln-{line_idx}",
                                    class: if line_idx == buffer.read().cursor_line() { "line-number active" } else { "line-number" },
                                    "{line_idx + 1}"
                                }
                            }
                        }
                        // Editor content - mouse events attached here for accurate coordinates
                        div {
                            class: "editor-content",
                            onmousedown,
                            onmousemove,
                            onmouseup,
                            ondoubleclick,
                            for (line_idx, line) in buffer.read().lines().enumerate() {
                                {
                                    let cur_line = buffer.read().cursor_line();
                                    let cur_col = buffer.read().cursor_col();
                                    let sel = buffer.read().selection_positions();
                                    rsx! {
                                        div {
                                            key: "{line_idx}",
                                            class: if line_idx == cur_line { "editor-line active" } else { "editor-line" },

                                            // Render line w/ cursor, selection, and search highlights
                                            {render_line(
                                                line_idx,
                                                &line,
                                                cur_line,
                                                cur_col,
                                                sel,
                                                is_focused(),
                                                search_state.read().matches_on_line(line_idx),
                                            )}
                                        }
                                    }
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
    search_matches: Vec<(usize, usize, bool)>, // (start_col, end_col, is_current_match)
) -> Element {
    let cursor_class = if is_focused { "cursor-blink" } else { "cursor-static" };
    let line_chars: Vec<char> = line.chars().collect();
    let line_len = line_chars.len();

    // Debug: log what render_line receives
    if line_idx == cursor_line {
    }

    // Check if this line has selection
    let (sel_start_line, sel_start_col, sel_end_line, sel_end_col) =
        selection.unwrap_or((usize::MAX, 0, 0, 0));

    let has_selection_on_line = selection.is_some()
        && line_idx >= sel_start_line
        && line_idx <= sel_end_line;

    let has_cursor = line_idx == cursor_line;
    let has_search_matches = !search_matches.is_empty();

    // Fast path: nothing special on this line
    if !has_selection_on_line && !has_cursor && !has_search_matches {
        return rsx! { "{line}" };
    }

    // Build spans for this line by collecting "regions" with different styles
    // Each char position can have: normal, selected, search-match, current-match, cursor

    // For each character, determine its styling
    #[derive(Clone, Copy, PartialEq)]
    enum CharStyle {
        Normal,
        Selected,
        SearchMatch,
        CurrentMatch,
    }

    let mut char_styles: Vec<CharStyle> = vec![CharStyle::Normal; line_len];

    // Apply search match styles (lowest priority)
    for (start, end, is_current) in &search_matches {
        for i in *start..*end.min(&line_len) {
            char_styles[i] = if *is_current {
                CharStyle::CurrentMatch
            } else {
                CharStyle::SearchMatch
            };
        }
    }

    // Apply selection styles (higher priority - overrides search)
    if has_selection_on_line {
        let sel_start = if line_idx == sel_start_line { sel_start_col } else { 0 };
        let sel_end = if line_idx == sel_end_line { sel_end_col } else { line_len };
        for i in sel_start..sel_end.min(line_len) {
            char_styles[i] = CharStyle::Selected;
        }
    }

    // Group consecutive characters with same style into spans
    let mut spans: Vec<(CharStyle, String)> = vec![];
    let mut current_style = if line_len > 0 { char_styles[0] } else { CharStyle::Normal };
    let mut current_text = String::new();

    for (i, ch) in line_chars.iter().enumerate() {
        if char_styles[i] != current_style {
            if !current_text.is_empty() {
                spans.push((current_style, current_text));
            }
            current_style = char_styles[i];
            current_text = String::new();
        }
        current_text.push(*ch);
    }
    if !current_text.is_empty() {
        spans.push((current_style, current_text));
    }

    // Now render with cursor insertion if needed
    if has_cursor {
        // Find where to insert cursor
        let mut char_pos = 0;
        let mut result_spans: Vec<Element> = vec![];
        let mut cursor_inserted = false;

        for (style, text) in spans {
            let span_len = text.chars().count();
            let span_end = char_pos + span_len;

            if !cursor_inserted && cursor_col >= char_pos && cursor_col < span_end {
                // Cursor is within this span - split it
                let offset = cursor_col - char_pos;
                let before: String = text.chars().take(offset).collect();
                let after: String = text.chars().skip(offset).collect();

                let class = match style {
                    CharStyle::Normal => "",
                    CharStyle::Selected => "selection-highlight",
                    CharStyle::SearchMatch => "search-highlight",
                    CharStyle::CurrentMatch => "search-highlight-current",
                };

                if !before.is_empty() {
                    if class.is_empty() {
                        result_spans.push(rsx! { span { "{before}" } });
                    } else {
                        result_spans.push(rsx! { span { class: "{class}", "{before}" } });
                    }
                }

                result_spans.push(rsx! { span { class: "{cursor_class}" } });
                cursor_inserted = true;

                if !after.is_empty() {
                    if class.is_empty() {
                        result_spans.push(rsx! { span { "{after}" } });
                    } else {
                        result_spans.push(rsx! { span { class: "{class}", "{after}" } });
                    }
                }
            } else {
                // Cursor not in this span
                let class = match style {
                    CharStyle::Normal => "",
                    CharStyle::Selected => "selection-highlight",
                    CharStyle::SearchMatch => "search-highlight",
                    CharStyle::CurrentMatch => "search-highlight-current",
                };

                if class.is_empty() {
                    result_spans.push(rsx! { span { "{text}" } });
                } else {
                    result_spans.push(rsx! { span { class: "{class}", "{text}" } });
                }
            }

            char_pos = span_end;
        }

        // If cursor is at end of line (and wasn't inserted yet)
        if !cursor_inserted && cursor_col >= line_len {
            result_spans.push(rsx! { span { class: "{cursor_class}" } });
        }

        return rsx! {
            for (idx, span) in result_spans.into_iter().enumerate() {
                Fragment { key: "{idx}", {span} }
            }
        };
    }

    // No cursor, just render styled spans
    rsx! {
        for (idx, (style, text)) in spans.into_iter().enumerate() {
            match style {
                CharStyle::Normal => rsx! {
                    span { key: "{idx}", "{text}" }
                },
                CharStyle::Selected => rsx! {
                    span { key: "{idx}", class: "selection-highlight", "{text}" }
                },
                CharStyle::SearchMatch => rsx! {
                    span { key: "{idx}", class: "search-highlight", "{text}" }
                },
                CharStyle::CurrentMatch => rsx! {
                    span { key: "{idx}", class: "search-highlight-current", "{text}" }
                },
            }
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