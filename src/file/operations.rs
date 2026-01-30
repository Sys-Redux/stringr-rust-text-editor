// File system operations for workspace management
use std::path::PathBuf;
use std::fs;
use crate::workspace::{FileTree, FileNode};
use super::FileError;

// Scan dir and build file tree
pub fn scan_directory(path: &PathBuf) -> Result<FileTree, FileError> {
    if !path.is_dir() {
        return Err(FileError::NotFound(path.clone()));
    }

    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Workspace")
        .to_string();

    let mut root = FileNode::new_directory(name, path.clone(), 0);
    root.is_expanded = true; // Expand root by default

    scan_recursive(&mut root, path, 1, 3)?; // Max depth 3 for performance
    root.sort_children();

    Ok(FileTree {
        root: Some(root),
        root_path: Some(path.clone()),
    })
}

// Recursively scan dir contents
fn scan_recursive(
    parent: &mut FileNode,
    path: &PathBuf,
    depth: usize,
    max_depth: usize,
) -> Result<(), FileError> {
    if depth > max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(path).map_err(|e| FileError::IoError(e))?;

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skipp hidden files
        if name.starts_with('.') {
            continue;
        }

        // Skip non-essential
        if matches!(name.as_str(), "node_modules" | "target" | ".git" | "__pycache__" | ".vscode") {
            continue;
        }

        if entry_path.is_dir() {
            let mut dir_node = FileNode::new_directory(name, entry_path.clone(), depth);
            scan_recursive(&mut dir_node, &entry_path, depth + 1, max_depth)?;
            parent.children.push(dir_node);
        } else {
            let file_node = FileNode::new_file(name, entry_path.clone(), depth);
            parent.children.push(file_node);
        }
    }
    Ok(())
}

// Expand dir node
pub fn expand_directory(tree: &mut FileTree, path: &PathBuf) -> Result<(), FileError> {
    if let Some(node) = tree.find_node_mut(path) {
        if node.is_directory() && node.children.is_empty() {
            scan_recursive(node, path, node.depth + 1, node.depth + 2)?;
            node.sort_children();
        }
        node.is_expanded = true;
    }
    Ok(())
}

// Create new file in workspace
pub async fn create_file(path: &PathBuf) -> Result<(), FileError> {
    tokio::fs::write(path, "").await.map_err(|e| FileError::IoError(e))
}

// Create new dir in workspace
pub async fn create_directory(path: &PathBuf) -> Result<(), FileError> {
    tokio::fs::create_dir_all(path).await.map_err(|e| FileError::IoError(e))
}

// Delete file or dir in workspace
pub async fn delete_path(path: &PathBuf) -> Result<(), FileError> {
    if path.is_dir() {
        tokio::fs::remove_dir_all(path).await.map_err(|e| FileError::IoError(e))
    } else {
        tokio::fs::remove_file(path).await.map_err(|e| FileError::IoError(e))
    }
}

// Rename file or dir in workspace
pub async fn rename_path(old_path: &PathBuf, new_path: &PathBuf) -> Result<(), FileError> {
    tokio::fs::rename(old_path, new_path).await.map_err(|e| FileError::IoError(e))
}