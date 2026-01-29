// File tree data structure for file explorer
use std::path::PathBuf;
use std::cmp::Ordering;

// Type of node in the file tree
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
}

// A node in the file tree
#[derive(Debug, Clone)]
pub struct FileNode {
    // File/folder name
    pub name: String,
    // Full path
    pub path: PathBuf,
    // File or directory
    pub node_type: NodeType,
    // Child nodes (if directory)
    pub children: Vec<FileNode>,
    // Is the directory expanded in the UI
    pub is_expanded: bool,
    // Depth in the tree (for indentation)
    pub depth: usize,
}

impl FileNode {
    // Create new file node
    pub fn new_file(name: String, path: PathBuf, depth: usize) -> Self {
        Self {
            name, path,
            node_type: NodeType::File,
            children: Vec::new(),
            is_expanded: false,
            depth,
        }
    }

    // Create new directory node
    pub fn new_directory(name: String, path: PathBuf, depth: usize) -> Self {
        Self {
            name,
            path,
            node_type: NodeType::Directory,
            children: Vec::new(),
            is_expanded: false,
            depth,
        }
    }

    // Check if directory
    pub fn is_directory(&self) -> bool {
        self.node_type == NodeType::Directory
    }

    // Toggle expanded state
    pub fn toggle_expanded(&mut self) {
        if self.is_directory() {
            self.is_expanded = !self.is_expanded;
        }
    }

    // Sort children: directories -> files -> alphabetically
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| {
            match (a.node_type, b.node_type) {
                (NodeType::Directory, NodeType::File) => Ordering::Less,
                (NodeType::File, NodeType::Directory) => Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        // Recursively sort children's children
        for child in &mut self.children {
            child.sort_children();
        }
    }
}

// Complete file tree for the workspace
#[derive(Debug, Clone)]
pub struct FileTree {
    pub root: Option<FileNode>,
    pub root_path: Option<PathBuf>,
}

impl FileTree {
    // Create empty file tree
    pub fn new() -> Self {
        Self {
            root: None,
            root_path: None,
        }
    }

    // Check if workspace loaded
    pub fn has_workspace(&self) -> bool {
        self.root.is_some()
    }

    // Get workspace name
    pub fn workspace_name(&self) -> Option<String> {
        self.root.as_ref().map(|r| r.name.clone())
    }

    // Find node by path
    pub fn find_node_mut(&mut self, path: &PathBuf) -> Option<&mut FileNode> {
        self.root.as_mut().and_then(|root| find_node_recursive(root, path))
    }

    // Toggle expansion of dir
    pub fn toggle_node(&mut self, path: &PathBuf) {
        if let Some(node) = self.find_node_mut(path) {
            node.toggle_expanded();
        }
    }

    // Flatten tree into a list for UI rendering
    pub fn flatten_visible(&self) -> Vec<&FileNode> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            flatten_recursive(root, &mut result, true);
        }
        result
    }
}

// Helper: recursively find node by path
fn find_node_recursive<'a>(node: &'a mut FileNode, path: &PathBuf) -> Option<&'a mut FileNode> {
    if &node.path == path {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_recursive(child, path) {
            return Some(found);
        }
    }
    None
}

// Helper: flatten tree for rendering
fn flatten_recursive<'a>(node: &'a FileNode, result: &mut Vec<&'a FileNode>, include_self: bool) {
    if include_self {
        result.push(node);
    }
    if node.is_directory && node.is_expanded {
        for child in &node.children {
            flatten_recursive(child, result, true);
        }
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}