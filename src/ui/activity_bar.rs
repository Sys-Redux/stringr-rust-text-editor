// Activity Bar component for Stringr
// VS Code-style sidebar with extension icons

use dioxus::prelude::*;

/// The different panels/views that can be shown in the sidebar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityPanel {
    /// File explorer view
    Files,
    // Future panels can be added here:
    // Search,
    // SourceControl,
    // Extensions,
    // etc.
}

// SVG icons for activity bar
const ICON_FILES: &str = r#"<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
    <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
    <polyline points="13 2 13 9 20 9"/>
</svg>"#;

/// Activity bar component - vertical icon bar on the far left
#[component]
pub fn ActivityBar(
    /// Currently active panel
    active_panel: Signal<Option<ActivityPanel>>,
    /// Callback when a panel icon is clicked
    on_panel_select: EventHandler<ActivityPanel>,
) -> Element {
    // Check if files panel is active
    let files_active = active_panel().map_or(false, |p| p == ActivityPanel::Files);

    // Handle files icon click
    let handle_files_click = move |_| {
        on_panel_select.call(ActivityPanel::Files);
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
                        dangerous_inner_html: ICON_FILES,
                    }

                    // Active indicator bar
                    if files_active {
                        div { class: "activity-bar-indicator" }
                    }
                }

                // Future icons will go here:
                // - Search
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
