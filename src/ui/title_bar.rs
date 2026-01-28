// Title bar component for Stringr
// Custom window chrome with draggable area and window controls

use dioxus::prelude::*;
use dioxus::desktop::window;

/// SVG icons as constants for window controls
const ICON_MINIMIZE: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor"><rect y="5" width="12" height="2"/></svg>"#;
const ICON_MAXIMIZE: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="1" y="1" width="10" height="10"/></svg>"#;
const ICON_RESTORE: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="1" width="8" height="8"/><rect x="1" y="3" width="8" height="8"/></svg>"#;
const ICON_CLOSE: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor"><path d="M1 1L11 11M1 11L11 1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>"#;

// Title bar showing filename, dirty state, menus, and window controls
#[component]
pub fn TitleBar(
    // The filename to display (None = "Untitled")
    filename: Option<String>,
    // Whether the buffer has unsaved changes
    is_dirty: bool,
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

            // Right side: Window controls
            div {
                class: "title-bar-controls",

                // Minimize button
                button {
                    class: "title-bar-btn title-bar-btn-minimize",
                    title: "Minimize",
                    onclick: handle_minimize,
                    dangerous_inner_html: ICON_MINIMIZE
                }

                // Maximize/Restore button
                button {
                    class: "title-bar-btn title-bar-btn-maximize",
                    title: if is_maximized() { "Restore" } else { "Maximize" },
                    onclick: handle_maximize,
                    dangerous_inner_html: if is_maximized() { ICON_RESTORE } else { ICON_MAXIMIZE }
                }

                // Close button
                button {
                    class: "title-bar-btn title-bar-btn-close",
                    title: "Close",
                    onclick: handle_close,
                    dangerous_inner_html: ICON_CLOSE
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