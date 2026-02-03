// Search Panel component for the sidebar
// Full search & replace functionality in the sidebar area

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdChevronUp, LdChevronDown, LdCaseSensitive,
    LdChevronRight, LdReplace, LdReplaceAll
};
use crate::editor::Buffer;
use crate::search::SearchState;

/// Search panel component - shows in sidebar when search is active
#[component]
pub fn SearchPanel(
    /// Whether this panel is visible
    is_visible: bool,
    /// Search state signal
    mut search_state: Signal<SearchState>,
    /// Buffer signal for searching
    buffer: Signal<Buffer>,
) -> Element {
    // Don't render if not visible
    if !is_visible {
        return rsx! {};
    }

    // Local state for inputs
    let mut query_input = use_signal(|| search_state.read().query.clone());
    let mut replace_input = use_signal(|| search_state.read().replace_text.clone());

    // Track if replace section is expanded
    let mut show_replace = use_signal(|| false);

    // Derived state
    let match_info = use_memo(move || search_state.read().match_info());
    let is_case_sensitive = use_memo(move || search_state.read().case_sensitive);
    let has_matches = use_memo(move || !search_state.read().matches.is_empty());
    let match_count = use_memo(move || search_state.read().matches.len());

    // Handle search input change
    let on_query_change = move |evt: Event<FormData>| {
        let value = evt.value().clone();
        query_input.set(value.clone());
        search_state.write().set_query(value, &buffer.read());
    };

    // Handle replace input change
    let on_replace_change = move |evt: Event<FormData>| {
        let value = evt.value().clone();
        replace_input.set(value.clone());
        search_state.write().replace_text = value;
    };

    // Handle key events in search input
    let on_search_keydown = move |evt: Event<KeyboardData>| {
        match evt.key() {
            Key::Enter => {
                evt.prevent_default();
                if evt.modifiers().shift() {
                    search_state.write().find_previous();
                } else {
                    search_state.write().find_next();
                }
            }
            _ => {}
        }
    };

    // Handle key events in replace input
    let on_replace_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            evt.prevent_default();
            let mut buf = buffer.write();
            search_state.write().replace_current(&mut buf);
        }
    };

    // Button handlers
    let on_find_previous = move |_| {
        search_state.write().find_previous();
    };

    let on_find_next = move |_| {
        search_state.write().find_next();
    };

    let on_toggle_case = move |_| {
        search_state.write().toggle_case_sensitive(&buffer.read());
    };

    let on_toggle_replace = move |_| {
        show_replace.set(!show_replace());
    };

    let on_replace_current = move |_| {
        let mut buf = buffer.write();
        search_state.write().replace_current(&mut buf);
    };

    let on_replace_all = move |_| {
        let mut buf = buffer.write();
        search_state.write().replace_all(&mut buf);
    };

    rsx! {
        div {
            class: "search-panel",

            // Panel header
            div {
                class: "search-panel-header",

                // Title with expand/collapse toggle for replace
                button {
                    class: "search-panel-toggle",
                    onclick: on_toggle_replace,
                    title: "Toggle Replace",

                    Icon {
                        icon: LdChevronRight,
                        width: 16,
                        height: 16,
                        class: if show_replace() { "rotate-90" } else { "" }
                    }
                }

                span { class: "search-panel-title", "SEARCH" }
            }

            // Search input row
            div {
                class: "search-panel-row",

                div {
                    class: "search-panel-input-wrapper",

                    input {
                        class: "search-panel-input",
                        r#type: "text",
                        placeholder: "Search",
                        value: "{query_input}",
                        oninput: on_query_change,
                        onkeydown: on_search_keydown,
                    }

                    // Case sensitivity toggle inside input
                    button {
                        class: if is_case_sensitive() { "search-input-btn active" } else { "search-input-btn" },
                        onclick: on_toggle_case,
                        title: "Match Case",
                        Icon { icon: LdCaseSensitive, width: 14, height: 14 }
                    }
                }

                // Navigation buttons
                button {
                    class: if !has_matches() { "search-panel-btn disabled" } else { "search-panel-btn" },
                    onclick: on_find_previous,
                    title: "Previous Match (Shift+Enter)",
                    Icon { icon: LdChevronUp, width: 16, height: 16 }
                }
                button {
                    class: if !has_matches() { "search-panel-btn disabled" } else { "search-panel-btn" },
                    onclick: on_find_next,
                    title: "Next Match (Enter)",
                    Icon { icon: LdChevronDown, width: 16, height: 16 }
                }
            }

            // Replace input row (conditionally shown)
            if show_replace() {
                div {
                    class: "search-panel-row",

                    div {
                        class: "search-panel-input-wrapper",

                        input {
                            class: "search-panel-input",
                            r#type: "text",
                            placeholder: "Replace",
                            value: "{replace_input}",
                            oninput: on_replace_change,
                            onkeydown: on_replace_keydown,
                        }
                    }

                    // Replace buttons
                    button {
                        class: if !has_matches() { "search-panel-btn disabled" } else { "search-panel-btn" },
                        onclick: on_replace_current,
                        title: "Replace (Enter)",
                        Icon { icon: LdReplace, width: 16, height: 16 }
                    }
                    button {
                        class: if !has_matches() { "search-panel-btn disabled" } else { "search-panel-btn" },
                        onclick: on_replace_all,
                        title: "Replace All",
                        Icon { icon: LdReplaceAll, width: 16, height: 16 }
                    }
                }
            }

            // Results summary
            if !query_input().is_empty() {
                div {
                    class: "search-panel-results",

                    if match_count() > 0 {
                        span {
                            class: "search-panel-results-text",
                            "{match_info}"
                        }
                    } else {
                        span {
                            class: "search-panel-results-text no-results",
                            "No results found"
                        }
                    }
                }
            }

            // Match list (scrollable)
            if match_count() > 0 {
                div {
                    class: "search-panel-matches",

                    for (idx, m) in search_state.read().matches.iter().enumerate() {
                        SearchMatchItem {
                            key: "{idx}",
                            match_idx: idx,
                            line_num: m.start_line + 1,
                            is_current: search_state.read().current_match_idx == Some(idx),
                            buffer: buffer,
                            search_state: search_state,
                        }
                    }
                }
            }
        }
    }
}

/// Individual search match item in the list
#[component]
fn SearchMatchItem(
    match_idx: usize,
    line_num: usize,
    is_current: bool,
    buffer: Signal<Buffer>,
    mut search_state: Signal<SearchState>,
) -> Element {
    // Get the line content for preview
    let line_preview = buffer.read()
        .lines()
        .nth(line_num - 1)
        .map(|s| {
            let trimmed = s.trim();
            if trimmed.len() > 50 {
                format!("{}...", &trimmed[..50])
            } else {
                trimmed.to_string()
            }
        })
        .unwrap_or_default();

    let on_click = move |_| {
        search_state.write().current_match_idx = Some(match_idx);
        // TODO: Jump cursor to this match in the editor
    };

    rsx! {
        button {
            class: if is_current { "search-match-item current" } else { "search-match-item" },
            onclick: on_click,

            span { class: "search-match-line", "Line {line_num}" }
            span { class: "search-match-preview", "{line_preview}" }
        }
    }
}
