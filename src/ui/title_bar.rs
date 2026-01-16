// Title bar component for Stringr
// Displays the current filename and unsaved changes indicator.

use dioxus::prelude::*;

// Title bar showing filename and dirty state
#[component]
pub fn TitleBar(
    // The filename to display (None = "Untitled")
    filename: Option<String>,
    // Whether the buffer has unsaved changes
    is_dirty: bool,
) -> Element {
    // Display name with fallback
    let display_name = filename.unwrap_or_else(|| "Untitled".to_string());

    // Dirty indicator - yellow dot for unsaved changes
    let dirty_indicator = if is_dirty { " •" } else { "" };

    rsx! {
        div {
            class: "title-bar",

            // Window controls placeholder (for future custom chrome)
            div {
                class: "title-bar-controls",
                // Placeholder for custom window controls if we want them
            }

            // Center: filename and dirty indicator
            div {
                class: "title-bar-title",
                span {
                    class: "font-bold",
                    "{display_name}"
                }
                if is_dirty {
                    span {
                        class: "text-primary ml-1",
                        title: "Unsaved changes",
                        "•"
                    }
                }
            }

            // Right side: spacer for balance
            div {
                class: "title-bar-controls",
            }
        }
    }
}