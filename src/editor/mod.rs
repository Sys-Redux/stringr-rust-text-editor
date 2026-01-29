// Editor module - text buffer & cursor management
mod buffer;
pub mod cursor;

pub use buffer::Buffer;
#[allow(unused_imports)]
pub use cursor::{Cursor, Position};