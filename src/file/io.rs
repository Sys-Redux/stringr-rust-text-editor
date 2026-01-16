// File I/O operations for Stringr
// Provides async file reading and writing using tokio

use std::path::PathBuf;
use tokio::fs;
use std::io;

// Error types for file operations
#[derive(Debug)]
pub enum FileError {
    // File not found
    NotFound(PathBuf),
    // Permission denied
    PermissionDenied(PathBuf),
    // Generic I/O error
    IoError(io::Error),
    // File is not valid UTF-8
    InvalidUtf8(PathBuf),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "File not found: {}", path.display()),
            FileError::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            FileError::IoError(e) => write!(f, "I/O error: {}", e),
            FileError::InvalidUtf8(path) => write!(f, "File is not valid UTF-8: {}", path.display()),
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::IoError(err)
    }
}

// Read a file's contents as a UTF-8 string
pub async fn read_file(path: &PathBuf) -> Result<String, FileError> {
    match fs::read_to_string(path).await {
        Ok(content) => Ok(content),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => Err(FileError::NotFound(path.clone())),
                io::ErrorKind::PermissionDenied => Err(FileError::PermissionDenied(path.clone())),
                _ => Err(FileError::IoError(e)),
            }
        }
    }
}

/// Write content to a file, creating it if it doesn't exist
pub async fn write_file(path: &PathBuf, content: &str) -> Result<(), FileError> {
    match fs::write(path, content).await {
        Ok(()) => Ok(()),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::PermissionDenied => Err(FileError::PermissionDenied(path.clone())),
                _ => Err(FileError::IoError(e)),
            }
        }
    }
}

/// Check if a file exists
pub async fn file_exists(path: &PathBuf) -> bool {
    fs::metadata(path).await.is_ok()
}

/// Get the filename from a path (for display purposes)
pub fn get_filename(path: &PathBuf) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string()
}