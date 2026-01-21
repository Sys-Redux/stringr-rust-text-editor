// Shortcuts module
mod keystrokes;

pub use keystrokes::{
    ShortcutAction,
    parse_shortcut,
    handle_new_file,
    handle_open_file,
    handle_save_file,
    handle_copy,
    handle_paste,
    handle_cut,
};