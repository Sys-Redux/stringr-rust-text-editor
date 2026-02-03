// UI Components module

mod status_bar;
mod title_bar;
mod file_explorer;
mod activity_bar;
mod search_panel;
// search_bar is deprecated - search is now integrated into title_bar

pub use status_bar::StatusBar;
pub use title_bar::TitleBar;
pub use file_explorer::FileExplorer;
pub use activity_bar::{ActivityBar, ActivityPanel};
pub use search_panel::SearchPanel;
