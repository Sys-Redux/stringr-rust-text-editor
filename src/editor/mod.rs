// Editor module - text buffer & cursor management
mod buffer;
pub mod cursor;

pub use buffer::Buffer;
pub use cursor::{Cursor, Position};