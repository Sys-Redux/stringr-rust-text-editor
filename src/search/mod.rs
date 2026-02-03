// Search module - find & replace functionality
mod find;
pub use find::SearchState;
// Re-export for potential future use (e.g., search match highlighting)
#[allow(unused)]
pub use find::{SearchDirection, SearchMatch};