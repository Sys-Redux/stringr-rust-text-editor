// Title bar component for Stringr
// Custom window chrome with draggable area and window controls

use dioxus::prelude::*;
use dioxus::desktop::window;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdMinus, LdSquare, LdX, LdCopy, LdSearch, LdChevronUp, LdChevronDown,
    LdCaseSensitive, LdReplace
};
use crate::editor::Buffer;
use crate::search::SearchState;

// Title bar showing filename, dirty state, menus, search bar, and window controls
#[component]
pub fn TitleBar(
    // The filename to display (None = "Untitled")
    filename: Option<String>,
    // Whether the buffer has unsaved changes
    is_dirty: bool,
    // Search state signal
    search_state: Signal<SearchState>,
    // Buffer signal for searching
    buffer: Signal<Buffer>,
) -> Element {
    // Display name with fallback
    let display_name = filename.unwrap_or_else(|| "Untitled".to_string());

    // Track window maximized state
    let mut is_maximized = use_signal(|| false);

    // Track which menu is open (None = all closed)
    let mut open_menu = use_signal(|| None::<&'static str>);

    // Window control handlers
    let handle_minimize = move |_| {
        window().set_minimized(true);
    };

    let handle_maximize = move |_| {
        let current = is_maximized();
        window().set_maximized(!current);
        is_maximized.set(!current);
    };

    // Close handler with unsaved changes check
    let handle_close = move |_| {
        if is_dirty {
            // Show confirmation dialog for unsaved changes
            spawn(async move {
                let confirmed = rfd::AsyncMessageDialog::new()
                    .set_title("Unsaved Changes")
                    .set_description("You have unsaved changes. Are you sure you want to quit?")
                    .set_buttons(rfd::MessageButtons::YesNo)
                    .show()
                    .await;

                if confirmed == rfd::MessageDialogResult::Yes {
                    std::process::exit(0);
                }
            });
        } else {
            // No unsaved changes - close directly
            window().close();
        }
    };

    // Handle drag on title bar for window movement
    let handle_drag = move |_| {
        window().drag();
    };

    // Close menus when clicking outside
    let close_menus = move |_| {
        open_menu.set(None);
    };

    rsx! {
        div {
            class: "title-bar",

            // Left side: Menu bar
            div {
                class: "title-bar-menu-section",

                // File menu
                MenuDropdown {
                    label: "File",
                    is_open: open_menu() == Some("file"),
                    on_toggle: move |_| {
                        if open_menu() == Some("file") {
                            open_menu.set(None);
                        } else {
                            open_menu.set(Some("file"));
                        }
                    },
                    MenuItem { label: "New", shortcut: "Ctrl+N" }
                    MenuItem { label: "Open...", shortcut: "Ctrl+O" }
                    MenuItem { label: "Save", shortcut: "Ctrl+S" }
                    MenuItem { label: "Save As...", shortcut: "Ctrl+Shift+S" }
                    MenuSeparator {}
                    MenuItem { label: "Exit", shortcut: "Alt+F4" }
                }

                // Edit menu
                MenuDropdown {
                    label: "Edit",
                    is_open: open_menu() == Some("edit"),
                    on_toggle: move |_| {
                        if open_menu() == Some("edit") {
                            open_menu.set(None);
                        } else {
                            open_menu.set(Some("edit"));
                        }
                    },
                    MenuItem { label: "Undo", shortcut: "Ctrl+Z" }
                    MenuItem { label: "Redo", shortcut: "Ctrl+Y" }
                    MenuSeparator {}
                    MenuItem { label: "Cut", shortcut: "Ctrl+X" }
                    MenuItem { label: "Copy", shortcut: "Ctrl+C" }
                    MenuItem { label: "Paste", shortcut: "Ctrl+V" }
                    MenuSeparator {}
                    MenuItem { label: "Select All", shortcut: "Ctrl+A" }
                }

                // View menu
                MenuDropdown {
                    label: "View",
                    is_open: open_menu() == Some("view"),
                    on_toggle: move |_| {
                        if open_menu() == Some("view") {
                            open_menu.set(None);
                        } else {
                            open_menu.set(Some("view"));
                        }
                    },
                    MenuItem { label: "Zoom In", shortcut: "Ctrl++" }
                    MenuItem { label: "Zoom Out", shortcut: "Ctrl+-" }
                    MenuItem { label: "Reset Zoom", shortcut: "Ctrl+0" }
                }

                // Help menu
                MenuDropdown {
                    label: "Help",
                    is_open: open_menu() == Some("help"),
                    on_toggle: move |_| {
                        if open_menu() == Some("help") {
                            open_menu.set(None);
                        } else {
                            open_menu.set(Some("help"));
                        }
                    },
                    MenuItem { label: "About Stringr", shortcut: "" }
                }
            }

            // Center: Draggable spacer with filename
            div {
                class: "title-bar-center",
                onmousedown: handle_drag,
                ondoubleclick: handle_maximize,

                span {
                    class: "title-bar-filename",
                    "{display_name}"
                }
                if is_dirty {
                    span {
                        class: "title-bar-dirty",
                        title: "Unsaved changes",
                        " •"
                    }
                }
            }

            // Search bar (always visible)
            TitleBarSearch {
                search_state: search_state,
                buffer: buffer,
            }

            // Right side: Window controls
            div {
                class: "title-bar-controls",

                // Minimize button
                button {
                    class: "title-bar-btn title-bar-btn-minimize",
                    title: "Minimize",
                    onclick: handle_minimize,
                    Icon { icon: LdMinus, width: 12, height: 12 }
                }

                // Maximize/Restore button
                button {
                    class: "title-bar-btn title-bar-btn-maximize",
                    title: if is_maximized() { "Restore" } else { "Maximize" },
                    onclick: handle_maximize,
                    if is_maximized() {
                        Icon { icon: LdCopy, width: 12, height: 12 }
                    } else {
                        Icon { icon: LdSquare, width: 12, height: 12 }
                    }
                }

                // Close button
                button {
                    class: "title-bar-btn title-bar-btn-close",
                    title: "Close",
                    onclick: handle_close,
                    Icon { icon: LdX, width: 12, height: 12 }
                }
            }
        }

        // Invisible overlay to close menus when clicking elsewhere
        if open_menu().is_some() {
            div {
                class: "menu-backdrop",
                onclick: close_menus,
            }
        }
    }
}

/// A dropdown menu in the title bar
#[component]
fn MenuDropdown(
    label: &'static str,
    is_open: bool,
    on_toggle: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "menu-dropdown",

            button {
                class: if is_open { "menu-trigger menu-trigger-active" } else { "menu-trigger" },
                onclick: move |e| on_toggle.call(e),
                "{label}"
            }

            if is_open {
                div {
                    class: "menu-content",
                    {children}
                }
            }
        }
    }
}

/// A single menu item
#[component]
fn MenuItem(
    label: &'static str,
    shortcut: &'static str,
) -> Element {
    rsx! {
        button {
            class: "menu-item",
            span { class: "menu-item-label", "{label}" }
            if !shortcut.is_empty() {
                span { class: "menu-item-shortcut", "{shortcut}" }
            }
        }
    }
}

/// A separator line in menus
#[component]
fn MenuSeparator() -> Element {
    rsx! {
        div { class: "menu-separator" }
    }
}

/// Inline search bar component for the title bar
#[component]
fn TitleBarSearch(
    mut search_state: Signal<SearchState>,
    buffer: Signal<Buffer>,
) -> Element {
    // Local state for input value
    let mut query_input = use_signal(|| search_state.read().query.clone());

    // Store the mounted input element for programmatic focus
    let mut input_element: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    // Local trigger to handle focus - checked on each render
    let mut focus_triggered = use_signal(|| false);

    // Check if external should_focus was set and copy to local trigger
    // Use peek() to avoid subscription
    if search_state.peek().should_focus && !*focus_triggered.peek() {
        focus_triggered.set(true);
    }

    // Effect runs after render to handle focus
    use_effect(move || {
        if focus_triggered() {
            // Clear the external flag
            search_state.write().should_focus = false;
            focus_triggered.set(false);

            // Focus the input
            if let Some(element) = input_element.peek().as_ref() {
                let _ = element.set_focus(true);
            }
        }
    });

    // Derived state
    let match_info = use_memo(move || search_state.read().match_info());
    let is_case_sensitive = use_memo(move || search_state.read().case_sensitive);
    let has_matches = use_memo(move || !search_state.read().matches.is_empty());
    let show_replace = use_memo(move || search_state.read().replace_mode);

    // Handle search input change
    let on_query_change = move |evt: Event<FormData>| {
        let value = evt.value().clone();
        query_input.set(value.clone());
        search_state.write().set_query(value, &buffer.read());
    };

    // Handle key events in search input
    let on_search_keydown = move |evt: Event<KeyboardData>| {
        match evt.key() {
            Key::Enter => {
                evt.prevent_default();
                if evt.modifiers().shift() {
                    search_state.write().find_previous();
                } else {
                    search_state.write().find_next();
                }
            }
            Key::Escape => {
                evt.prevent_default();
                // Clear search and unfocus
                query_input.set(String::new());
                search_state.write().set_query(String::new(), &buffer.read());
                search_state.write().replace_mode = false;
            }
            _ => {}
        }
    };

    // Button handlers
    let on_find_previous = move |_| {
        search_state.write().find_previous();
    };

    let on_find_next = move |_| {
        search_state.write().find_next();
    };

    let on_toggle_case = move |_| {
        search_state.write().toggle_case_sensitive(&buffer.read());
    };

    let on_toggle_replace = move |_| {
        let current = search_state.read().replace_mode;
        search_state.write().replace_mode = !current;
    };

    rsx! {
        div {
            class: "title-bar-search",

            // Search icon
            div {
                class: "title-bar-search-icon",
                Icon { icon: LdSearch, width: 14, height: 14 }
            }

            // Search input
            input {
                class: "title-bar-search-input",
                r#type: "text",
                placeholder: "Search...",
                value: "{query_input}",
                oninput: on_query_change,
                onkeydown: on_search_keydown,
                onmounted: move |evt| {
                    input_element.set(Some(evt.data()));
                },
            }

            // Match info (only show if there's a query)
            if !search_state.read().query.is_empty() {
                span {
                    class: "title-bar-search-info",
                    "{match_info}"
                }
            }

            // Navigation buttons
            button {
                class: if !has_matches() { "title-bar-search-btn disabled" } else { "title-bar-search-btn" },
                onclick: on_find_previous,
                title: "Previous Match (Shift+Enter)",
                Icon { icon: LdChevronUp, width: 14, height: 14 }
            }
            button {
                class: if !has_matches() { "title-bar-search-btn disabled" } else { "title-bar-search-btn" },
                onclick: on_find_next,
                title: "Next Match (Enter)",
                Icon { icon: LdChevronDown, width: 14, height: 14 }
            }

            // Case sensitivity toggle
            button {
                class: if is_case_sensitive() { "title-bar-search-btn active" } else { "title-bar-search-btn" },
                onclick: on_toggle_case,
                title: "Match Case",
                Icon { icon: LdCaseSensitive, width: 14, height: 14 }
            }

            // Replace toggle
            button {
                class: if show_replace() { "title-bar-search-btn active" } else { "title-bar-search-btn" },
                onclick: on_toggle_replace,
                title: "Toggle Replace (Ctrl+H)",
                Icon { icon: LdReplace, width: 14, height: 14 }
            }

            // Replace overlay (appears below when active)
            if show_replace() {
                ReplaceOverlay {
                    search_state: search_state,
                    buffer: buffer,
                }
            }
        }
    }
}

/// Replace overlay that appears below the search bar
#[component]
fn ReplaceOverlay(
    search_state: Signal<SearchState>,
    buffer: Signal<Buffer>,
) -> Element {
    let mut replace_input = use_signal(|| search_state.read().replace_text.clone());
    let has_matches = use_memo(move || !search_state.read().matches.is_empty());

    let on_replace_change = move |evt: Event<FormData>| {
        let value = evt.value().clone();
        replace_input.set(value.clone());
        search_state.write().replace_text = value;
    };

    let on_replace_keydown = move |evt: Event<KeyboardData>| {
        match evt.key() {
            Key::Enter => {
                evt.prevent_default();
                search_state.write().replace_current(&mut buffer.write());
            }
            Key::Escape => {
                evt.prevent_default();
                search_state.write().replace_mode = false;
            }
            _ => {}
        }
    };

    let on_replace = move |_| {
        search_state.write().replace_current(&mut buffer.write());
    };

    let on_replace_all = move |_| {
        search_state.write().replace_all(&mut buffer.write());
    };

    let on_close = move |_| {
        search_state.write().replace_mode = false;
    };

    rsx! {
        div {
            class: "replace-overlay",

            input {
                class: "replace-input",
                r#type: "text",
                placeholder: "Replace with...",
                value: "{replace_input}",
                oninput: on_replace_change,
                onkeydown: on_replace_keydown,
                autofocus: true,
            }

            button {
                class: if !has_matches() { "replace-btn disabled" } else { "replace-btn" },
                onclick: on_replace,
                title: "Replace (Enter)",
                "Replace"
            }

            button {
                class: if !has_matches() { "replace-btn disabled" } else { "replace-btn" },
                onclick: on_replace_all,
                title: "Replace All",
                "All"
            }

            button {
                class: "replace-close-btn",
                onclick: on_close,
                title: "Close (Escape)",
                Icon { icon: LdX, width: 12, height: 12 }
            }
        }
    }
}