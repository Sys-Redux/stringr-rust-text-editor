// File Explorer UI component

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdFolder, LdFolderOpen, LdFile, LdFileText, LdFileCode, LdFileJson,
    LdImage, LdChevronRight, LdChevronDown, LdSettings, LdPalette, LdGlobe
};
use std::path::PathBuf;
use crate::workspace::{FileTree, FileNode};

/// Enum to represent different file icon types for Lucide
#[derive(Clone, Copy, PartialEq)]
enum FileIconType {
    Folder,
    FolderOpen,
    File,
    FileText,
    FileCode,
    FileConfig,
    FileStyle,
    FileHtml,
    FileImage,
    FileRust,
}

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

// Get the appropriate file icon type based on extension
fn get_file_icon_type(filename: &str) -> FileIconType {
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "rs" => FileIconType::FileRust,
        "toml" | "json" | "yaml" | "yml" | "xml" => FileIconType::FileConfig,
        "md" | "txt" | "doc" | "docx" => FileIconType::FileText,
        "css" | "scss" | "sass" | "less" => FileIconType::FileStyle,
        "js" | "ts" | "jsx" | "tsx" => FileIconType::FileCode,
        "html" | "htm" => FileIconType::FileHtml,
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => FileIconType::FileImage,
        _ => FileIconType::File,
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

/// Render a file icon based on the icon type
#[component]
fn FileIcon(icon_type: FileIconType, class: String) -> Element {
    rsx! {
        div {
            class: "{class}",
            match icon_type {
                FileIconType::Folder => rsx! { Icon { icon: LdFolder, width: 16, height: 16 } },
                FileIconType::FolderOpen => rsx! { Icon { icon: LdFolderOpen, width: 16, height: 16 } },
                FileIconType::File => rsx! { Icon { icon: LdFile, width: 16, height: 16 } },
                FileIconType::FileText => rsx! { Icon { icon: LdFileText, width: 16, height: 16 } },
                FileIconType::FileCode => rsx! { Icon { icon: LdFileCode, width: 16, height: 16 } },
                FileIconType::FileConfig => rsx! { Icon { icon: LdFileJson, width: 16, height: 16 } },
                FileIconType::FileStyle => rsx! { Icon { icon: LdPalette, width: 16, height: 16 } },
                FileIconType::FileHtml => rsx! { Icon { icon: LdGlobe, width: 16, height: 16 } },
                FileIconType::FileImage => rsx! { Icon { icon: LdImage, width: 16, height: 16 } },
                FileIconType::FileRust => rsx! { Icon { icon: LdSettings, width: 16, height: 16 } },
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

    // Determine which icon type and class to use
    let (icon_type, icon_class): (FileIconType, &str) = if is_dir {
        if node.is_expanded {
            (FileIconType::FolderOpen, "file-tree-icon folder-open")
        } else {
            (FileIconType::Folder, "file-tree-icon folder")
        }
    } else {
        (get_file_icon_type(&node.name), get_file_icon_class(&node.name))
    };

    // Build the full class string for files (add base class)
    let full_icon_class = if is_dir {
        icon_class.to_string()
    } else {
        format!("file-tree-icon {}", icon_class)
    };

    rsx! {
        div {
            class: "file-tree-item",
            style: "padding-left: {indent}px;",
            onclick: handle_click,

            // Chevron for directories (or spacer for files)
            div {
                class: if is_dir { "file-tree-chevron has-children" } else { "file-tree-chevron" },
                if is_dir {
                    if node.is_expanded {
                        Icon { icon: LdChevronDown, width: 16, height: 16 }
                    } else {
                        Icon { icon: LdChevronRight, width: 16, height: 16 }
                    }
                }
            }

            // Icon
            FileIcon {
                icon_type: icon_type,
                class: full_icon_class,
            }

            // Name
            span {
                class: "file-tree-name",
                "{node.name}"
            }
        }
    }
}