// File operations module

mod io;

// These are public API methods - some not used yet but will be
#[allow(unused_imports)]
pub use io::{read_file, write_file, file_exists, get_filename, FileError};