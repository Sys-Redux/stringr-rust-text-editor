// File Explorer UI component

use dioxus::prelude::*;
use std::path::PathBuf;
use crate::workspace::{FileTree, FileNode};

// SVG icons for the file explorer - VS Code style
const ICON_FOLDER_CLOSED: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14.5 3H7.71l-.85-.85A.5.5 0 0 0 6.5 2h-5a.5.5 0 0 0-.5.5v11a.5.5 0 0 0 .5.5h13a.5.5 0 0 0 .5-.5v-10a.5.5 0 0 0-.5-.5zm-.5 10H2V5h12v8z"/></svg>"#;
const ICON_FOLDER_OPEN: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1.5 2A.5.5 0 0 0 1 2.5v11a.5.5 0 0 0 .5.5h13a.5.5 0 0 0 .5-.5V5a.5.5 0 0 0-.5-.5H7.707l-.853-.854A.5.5 0 0 0 6.5 3.5H1.5zM2 6v7h12V6H2z"/></svg>"#;
const ICON_FILE: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10.5 1H3.5C2.67 1 2 1.67 2 2.5v11c0 .83.67 1.5 1.5 1.5h9c.83 0 1.5-.67 1.5-1.5V4.5L10.5 1zm3 12.5c0 .28-.22.5-.5.5h-9a.5.5 0 0 1-.5-.5v-11c0-.28.22-.5.5-.5H10v2.5c0 .83.67 1.5 1.5 1.5H14v7.5h-.5z"/></svg>"#;
const ICON_FILE_RUST: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zm0 1.17a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16zm2.93.76l.4.4a.29.29 0 0 1-.41.41l-.4-.4a.29.29 0 0 1 .41-.41zM5.07 2.93a.29.29 0 0 1 .41.41l-.4.4a.29.29 0 0 1-.41-.41l.4-.4zM8 5.5a2.5 2.5 0 0 1 2.45 2H9.27a1.32 1.32 0 0 0-2.54 0H5.55A2.5 2.5 0 0 1 8 5.5zm4.83.67a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16zm-9.66 0a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16zM5.55 8.5h4.9a2.5 2.5 0 0 1-4.9 0zm-2.48.67a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16zm9.66 0a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16zm-7.65 1.9l.4.4a.29.29 0 0 1-.41.41l-.4-.4a.29.29 0 1 1 .41-.41zm7.84 0a.29.29 0 0 1 0 .41l-.4.4a.29.29 0 0 1-.41-.41l.4-.4a.29.29 0 0 1 .41 0zM8 12.67a.58.58 0 1 1 0 1.16.58.58 0 0 1 0-1.16z"/></svg>"#;
const ICON_FILE_CONFIG: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.7-1.3 2 .8.8 2-1.3.7.3.5 2.4h1.2l.5-2.4.7-.3 2 1.3.8-.8-1.3-2 .3-.7 2.4-.5V6.6l-2.4-.5-.3-.7 1.3-2-.8-.8-2 1.3-.7-.3zM8 10a2 2 0 1 1 0-4 2 2 0 0 1 0 4z"/></svg>"#;
const ICON_FILE_TEXT: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4 1h8a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zm1 3v1h6V4H5zm0 2v1h6V6H5zm0 2v1h4V8H5z"/></svg>"#;
const ICON_FILE_STYLE: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zM2.5 8a5.5 5.5 0 0 1 9.27-4.02L4.02 11.77A5.48 5.48 0 0 1 2.5 8zm3.25 5.52a5.5 5.5 0 0 0 7.77-7.77L5.75 13.52z"/></svg>"#;
const ICON_FILE_CODE: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10.478 1.647a.5.5 0 1 0-.956-.294l-4 13a.5.5 0 0 0 .956.294l4-13zM4.854 4.146a.5.5 0 0 1 0 .708L1.707 8l3.147 3.146a.5.5 0 0 1-.708.708l-3.5-3.5a.5.5 0 0 1 0-.708l3.5-3.5a.5.5 0 0 1 .708 0zm6.292 0a.5.5 0 0 0 0 .708L14.293 8l-3.147 3.146a.5.5 0 0 0 .708.708l3.5-3.5a.5.5 0 0 0 0-.708l-3.5-3.5a.5.5 0 0 0-.708 0z"/></svg>"#;
const ICON_FILE_HTML: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 1h12l-1.1 12L8 15l-4.9-2L2 1zm2.15 2l.8 9L8 13.28 11.05 12l.8-9H4.15zM6.5 6H10l-.15 2H7l.1 1.5h2.65l-.2 2.2L8 12.1l-1.55-.4-.1-1.2h1l.05.6.6.15.6-.15.1-1.1H6.4L6.15 6H6.5z"/></svg>"#;
const ICON_FILE_IMAGE: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2.5 2A1.5 1.5 0 0 0 1 3.5v9A1.5 1.5 0 0 0 2.5 14h11a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 13.5 2h-11zM2 3.5a.5.5 0 0 1 .5-.5h11a.5.5 0 0 1 .5.5v5.864l-2.682-2.682a.5.5 0 0 0-.707 0L8 9.293 6.354 7.646a.5.5 0 0 0-.708 0L2 11.293V3.5zM5 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"/></svg>"#;
const ICON_CHEVRON_RIGHT: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>"#;
const ICON_CHEVRON_DOWN: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M4 6l4 4 4-4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>"#;

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

// Get the appropriate file icon based on extension
fn get_file_icon(filename: &str) -> &'static str {
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "rs" => ICON_FILE_RUST,
        "toml" | "json" | "yaml" | "yml" | "xml" => ICON_FILE_CONFIG,
        "md" | "txt" | "doc" | "docx" => ICON_FILE_TEXT,
        "css" | "scss" | "sass" | "less" => ICON_FILE_STYLE,
        "js" | "ts" | "jsx" | "tsx" => ICON_FILE_CODE,
        "html" | "htm" => ICON_FILE_HTML,
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => ICON_FILE_IMAGE,
        _ => ICON_FILE,
    }
}

// Get icon color class based on file type
fn get_file_icon_class(filename: &str) -> &'static str {
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "rs" => "file-icon-rust",
        "toml" | "json" | "yaml" | "yml" | "xml" => "file-icon-config",
        "md" | "txt" => "file-icon-text",
        "css" | "scss" | "sass" | "less" => "file-icon-style",
        "js" | "ts" | "jsx" | "tsx" => "file-icon-code",
        "html" | "htm" => "file-icon-html",
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => "file-icon-image",
        _ => "file-icon-default",
    }
}

// Individual tree item (file or folder)
#[component]
fn FileTreeItem(
    node: FileNode,
    tree: Signal<FileTree>,
    on_file_open: EventHandler<PathBuf>,
) -> Element {
    let indent = node.depth * 12; // 12px per level for tighter nesting
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

    // Determine which icon and class to use
    let (icon_svg, icon_class): (&str, &str) = if is_dir {
        if node.is_expanded {
            (ICON_FOLDER_OPEN, "file-tree-icon folder-open")
        } else {
            (ICON_FOLDER_CLOSED, "file-tree-icon folder")
        }
    } else {
        (get_file_icon(&node.name), get_file_icon_class(&node.name))
    };

    // Build the full class string for files (add base class)
    let full_icon_class = if is_dir {
        icon_class.to_string()
    } else {
        format!("file-tree-icon {}", icon_class)
    };

    // Determine chevron
    let chevron_svg = if is_dir {
        if node.is_expanded {
            ICON_CHEVRON_DOWN
        } else {
            ICON_CHEVRON_RIGHT
        }
    } else {
        "" // No chevron for files
    };

    rsx! {
        div {
            class: "file-tree-item",
            style: "padding-left: {indent}px;",
            onclick: handle_click,

            // Chevron for directories (or spacer for files)
            div {
                class: if is_dir { "file-tree-chevron has-children" } else { "file-tree-chevron" },
                dangerous_inner_html: chevron_svg,
            }

            // Icon
            div {
                class: "{full_icon_class}",
                dangerous_inner_html: icon_svg,
            }

            // Name
            span {
                class: "file-tree-name",
                "{node.name}"
            }
        }
    }
}