// Status bar component showing cursor position and file info

use dioxus::prelude::*;

/// Status bar at the bottom of the editor
#[component]
pub fn StatusBar(
    /// Current line number (1-based for display)
    line: usize,
    /// Current column number (1-based for display)
    column: usize,
    /// Total number of lines
    total_lines: usize,
) -> Element {
    rsx! {
        div {
            class: "status-bar",

            // Left side - file info
            div {
                class: "status-left",
                span { class: "text-primary font-bold", "Stringr" }
            }

            // Right side - cursor position
            div {
                class: "flex gap-4",

                span { "Ln {line}, Col {column}" }
                span { "{total_lines} lines" }
            }
        }
    }
}