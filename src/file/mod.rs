// File module - I/O operations & workspace management

mod io;
mod operations;

// These are public API methods - some not used yet but will be
#[allow(unused_imports)]
pub use io::{read_file, write_file, file_exists, get_filename, FileError};
#[allow(unused_imports)]
pub use operations::{
    scan_directory, expand_directory, create_file, create_directory,
    delete_path, rename_path,
};