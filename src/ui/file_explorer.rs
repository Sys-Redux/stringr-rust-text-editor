// File Explorer UI component

use dioxus::prelude::*;
use std::path::PathBuf;
use crate::workspace::{FileTree, FileNode};

// SVG icons for the file explorer
const ICON_FOLDER_CLOSED: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1.5 2A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5V5a1.5 1.5 0 0 0-1.5-1.5H7.707l-.853-.854A.5.5 0 0 0 6.5 2.5H1.5z"/></svg>"#;
const ICON_FOLDER_OPEN: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M.54 3.87L.5 3a2 2 0 0 1 2-2h3.672a2 2 0 0 1 1.414.586l.828.828A2 2 0 0 0 9.828 3H14.5A1.5 1.5 0 0 1 16 4.5v1.384l-4.243 4.243a1.5 1.5 0 0 1-.914.433L2.5 11.5A1.5 1.5 0 0 1 1 10V4.5a1.5 1.5 0 0 1 .54-.63z"/></svg>"#;
const ICON_FILE: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V4.707A1 1 0 0 0 13.707 4L10 .293A1 1 0 0 0 9.293 0H4zm5 1.5v2a1 1 0 0 0 1 1h2l-3-3z"/></svg>"#;
const ICON_CHEVRON_RIGHT: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor"><path d="M4.5 2L8.5 6L4.5 10" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round"/></svg>"#;
const ICON_CHEVRON_DOWN: &str = r#"<svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor"><path d="M2 4.5L6 8.5L10 4.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round"/></svg>"#;

// Props for the FileExplorer component
#[component]
pub fn FileExplorer(
    // The file tree to display
    tree: Signal<FileTree>,
    // Callback when a file is clicked (to open it)
    on_file_open: EventHandler<PathBuf>,
    // Callback when requesting to open a folder
    on_open_folder: EventHandler<()>,
    // Whether the explorer panel is visible
    is_visible: bool,
) -> Element {
    if !is_visible {
        return rsx! {};
    }

    let has_workspace = tree.read().has_workspace();
    let workspace_name = tree.read().workspace_name();

    rsx! {
        div {
            class: "file-explorer",

            // Header
            div {
                class: "file-explorer-header",

                span {
                    class: "file-explorer-title",
                    if let Some(name) = workspace_name {
                        "{name}"
                    } else {
                        "EXPLORER"
                    }
                }
            }

            // Content area
            div {
                class: "file-explorer-content",

                if has_workspace {
                    // Render the file tree
                    FileTreeView {
                        tree: tree,
                        on_file_open: on_file_open,
                    }
                } else {
                    // No workspace open - show prompt
                    div {
                        class: "file-explorer-empty",

                        p { "No folder opened" }

                        button {
                            class: "btn-brutal-sm",
                            onclick: move |_| on_open_folder.call(()),
                            "Open Folder"
                        }
                    }
                }
            }
        }
    }
}

// Component to render the file tree
#[component]
fn FileTreeView(
    tree: Signal<FileTree>,
    on_file_open: EventHandler<PathBuf>,
) -> Element {
    // Clone nodes to avoid borrowing issues with the signal
    let visible_nodes: Vec<FileNode> = tree.read()
        .flatten_visible()
        .into_iter()
        .skip(1)  // Skip root, it's shown in header
        .cloned()
        .collect();

    rsx! {
        div {
            class: "file-tree",

            for node in visible_nodes.iter() {
                FileTreeItem {
                    key: "{node.path.display()}",
                    node: node.clone(),
                    tree: tree,
                    on_file_open: on_file_open,
                }
            }
        }
    }
}

// Individual tree item (file or folder)
#[component]
fn FileTreeItem(
    node: FileNode,
    tree: Signal<FileTree>,
    on_file_open: EventHandler<PathBuf>,
) -> Element {
    let indent = node.depth * 16; // 16px per level
    let is_dir = node.is_directory();
    let path = node.path.clone();
    let path_for_click = path.clone();

    let handle_click = move |_| {
        if is_dir {
            // Toggle expansion
            tree.write().toggle_node(&path);
        } else {
            // Open file
            on_file_open.call(path_for_click.clone());
        }
    };

    rsx! {
        div {
            class: "file-tree-item",
            style: "padding-left: {indent}px;",
            onclick: handle_click,

            // Chevron for directories
            span {
                class: "file-tree-chevron",
                if is_dir {
                    if node.is_expanded {
                        span { dangerous_inner_html: ICON_CHEVRON_DOWN }
                    } else {
                        span { dangerous_inner_html: ICON_CHEVRON_RIGHT }
                    }
                }
            }

            // Icon
            span {
                class: if is_dir { "file-tree-icon folder" } else { "file-tree-icon file" },
                if is_dir {
                    if node.is_expanded {
                        span { dangerous_inner_html: ICON_FOLDER_OPEN }
                    } else {
                        span { dangerous_inner_html: ICON_FOLDER_CLOSED }
                    }
                } else {
                    span { dangerous_inner_html: ICON_FILE }
                }
            }

            // Name
            span {
                class: "file-tree-name",
                "{node.name}"
            }
        }
    }
}