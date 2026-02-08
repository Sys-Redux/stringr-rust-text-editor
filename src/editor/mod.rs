// Editor module - text buffer & cursor management
mod buffer;
pub mod cursor;
mod formatting;

pub use buffer::Buffer;
#[allow(unused_imports)]
pub use cursor::{Cursor, Position, Selection};
#[allow(unused_imports)]
pub use formatting::{FormatStyle, FormatInfo, apply_format, toggle_format};