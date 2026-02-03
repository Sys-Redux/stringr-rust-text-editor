// Activity Bar component for Stringr
// VS Code-style sidebar with extension icons

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdFiles, LdSearch};

/// The different panels/views that can be shown in the sidebar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityPanel {
    /// File explorer view
    Files,
    /// Search panel
    Search,
    // Future panels can be added here:
    // SourceControl,
    // Extensions,
    // etc.
}

/// Activity bar component - vertical icon bar on the far left
#[component]
pub fn ActivityBar(
    /// Currently active panel
    active_panel: Signal<Option<ActivityPanel>>,
    /// Callback when a panel icon is clicked
    on_panel_select: EventHandler<ActivityPanel>,
) -> Element {
    // Check if each panel is active
    let files_active = active_panel().map_or(false, |p| p == ActivityPanel::Files);
    let search_active = active_panel().map_or(false, |p| p == ActivityPanel::Search);

    // Handle icon clicks
    let handle_files_click = move |_| {
        on_panel_select.call(ActivityPanel::Files);
    };

    let handle_search_click = move |_| {
        on_panel_select.call(ActivityPanel::Search);
    };

    rsx! {
        div {
            class: "activity-bar",

            // Top section - main navigation icons
            div {
                class: "activity-bar-top",

                // Files icon (always first, like VS Code)
                button {
                    class: if files_active { "activity-bar-item active" } else { "activity-bar-item" },
                    title: "Explorer",
                    onclick: handle_files_click,

                    div {
                        class: "activity-bar-icon",
                        Icon { icon: LdFiles, width: 24, height: 24 }
                    }

                    // Active indicator bar
                    if files_active {
                        div { class: "activity-bar-indicator" }
                    }
                }

                // Search icon
                button {
                    class: if search_active { "activity-bar-item active" } else { "activity-bar-item" },
                    title: "Search",
                    onclick: handle_search_click,

                    div {
                        class: "activity-bar-icon",
                        Icon { icon: LdSearch, width: 24, height: 24 }
                    }

                    // Active indicator bar
                    if search_active {
                        div { class: "activity-bar-indicator" }
                    }
                }

                // Future icons will go here:
                // - Source Control
                // - Extensions
                // - etc.
            }

            // Bottom section - logo/branding
            div {
                class: "activity-bar-bottom",

                // App logo
                div {
                    class: "activity-bar-logo",
                    title: "Stringr",

                    img {
                        src: asset!("/assets/img/beav_no_bkgrnd.png"),
                        alt: "Stringr Logo",
                        class: "activity-bar-logo-img",
                    }
                }
            }
        }
    }
}
