// Search bar overlay component for Stringr
// Positioned at top-right of the editor, like VS Code

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdX, LdChevronUp, LdChevronDown, LdCaseSensitive, LdChevronRight
};
use crate::editor::Buffer;
use crate::search::SearchState;

/// Search bar overlay component
#[component]
pub fn SearchBar(
    /// The search state signal
    search_state: Signal<SearchState>,
    /// The buffer signal for searching
    buffer: Signal<Buffer>,
) -> Element {
    // Local state for input values (we sync to search_state on change)
    let mut query_input = use_signal(|| search_state.read().query.clone());
    let mut replace_input = use_signal(|| search_state.read().replace_text.clone());

    // Derived state
    let match_info = use_memo(move || search_state.read().match_info());
    let is_case_sensitive = use_memo(move || search_state.read().case_sensitive);
    let show_replace = use_memo(move || search_state.read().replace_mode);
    let has_matches = use_memo(move || !search_state.read().matches.is_empty());

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
            Key::Escape => {
                evt.prevent_default();
                search_state.write().close();
            }
            _ => {}
        }
    };

    // Handle key events in replace input
    let on_replace_keydown = move |evt: Event<KeyboardData>| {
        match evt.key() {
            Key::Enter => {
                evt.prevent_default();
                search_state.write().replace_current(&mut buffer.write());
            }
            Key::Escape => {
                evt.prevent_default();
                search_state.write().close();
            }
            _ => {}
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
        let current = search_state.read().replace_mode;
        search_state.write().replace_mode = !current;
    };

    let on_replace = move |_| {
        search_state.write().replace_current(&mut buffer.write());
    };

    let on_replace_all = move |_| {
        search_state.write().replace_all(&mut buffer.write());
    };

    let on_close = move |_| {
        search_state.write().close();
    };

    rsx! {
        div {
            class: "search-overlay",

            // Search row
            div {
                class: "search-row",

                // Toggle replace mode button
                button {
                    class: if show_replace() { "search-toggle-btn active" } else { "search-toggle-btn" },
                    onclick: on_toggle_replace,
                    title: "Toggle Replace",
                    Icon { icon: LdChevronRight, width: 14, height: 14 }
                }

                // Search input
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "Find",
                    value: "{query_input}",
                    oninput: on_query_change,
                    onkeydown: on_search_keydown,
                    autofocus: true,
                }

                // Match info
                span {
                    class: "search-match-info",
                    "{match_info}"
                }

                // Navigation buttons
                button {
                    class: if !has_matches() { "search-btn disabled" } else { "search-btn" },
                    onclick: on_find_previous,
                    title: "Previous Match (Shift+F3)",
                    Icon { icon: LdChevronUp, width: 14, height: 14 }
                }
                button {
                    class: if !has_matches() { "search-btn disabled" } else { "search-btn" },
                    onclick: on_find_next,
                    title: "Next Match (F3)",
                    Icon { icon: LdChevronDown, width: 14, height: 14 }
                }

                // Case sensitivity toggle
                button {
                    class: if is_case_sensitive() { "search-btn active" } else { "search-btn" },
                    onclick: on_toggle_case,
                    title: "Match Case",
                    Icon { icon: LdCaseSensitive, width: 14, height: 14 }
                }

                // Close button
                button {
                    class: "search-btn search-close-btn",
                    onclick: on_close,
                    title: "Close (Escape)",
                    Icon { icon: LdX, width: 14, height: 14 }
                }
            }

            // Replace row (conditional)
            if show_replace() {
                div {
                    class: "search-row replace-row",

                    // Spacer to align with search input
                    div { class: "search-toggle-spacer" }

                    // Replace input
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Replace",
                        value: "{replace_input}",
                        oninput: on_replace_change,
                        onkeydown: on_replace_keydown,
                    }

                    // Replace buttons
                    button {
                        class: if !has_matches() { "search-btn replace-btn disabled" } else { "search-btn replace-btn" },
                        onclick: on_replace,
                        title: "Replace (Enter)",
                        "Replace"
                    }
                    button {
                        class: if !has_matches() { "search-btn replace-btn disabled" } else { "search-btn replace-btn" },
                        onclick: on_replace_all,
                        title: "Replace All",
                        "All"
                    }
                }
            }
        }
    }
}