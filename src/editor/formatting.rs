// Text-formatting module
// Handles Markdown

// Style types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatStyle {
    Bold,
    Italic,
    Underline,
    Code,
}

// Result of checking formatting at a position
#[derive(Debug, Clone)]
pub struct FormatInfo {
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_underline: bool,
    pub is_code: bool,
}

impl FormatStyle {
    // Get markdown markers for this style
    pub fn markers(&self) -> (&'static str, &'static str) {
        match self {
            FormatStyle::Bold => ("**", "**"),
            FormatStyle::Italic => ("*", "*"),
            FormatStyle::Underline => ("__", "__"),
            FormatStyle::Code => ("`", "`"),
        }
    }
}

// Appply formatting to selected text
pub fn apply_format(text: &str, style: FormatStyle) -> String {
    let (prefix, suffix) = style.markers();
    format!("{}{}{}", prefix, text, suffix)
}

// Remove formatting from selected text
pub fn remove_format(text: &str, style: FormatStyle) -> Option<String> {
    let (prefix, suffix) = style.markers();
    if text.starts_with(prefix) && text.ends_with(suffix) {
        let inner = &text[prefix.len()..text.len() - suffix.len()];
        Some(inner.to_string())
    } else {
        None
    }
}

// Toggle formatting
pub fn toggle_format(text: &str, style: FormatStyle) -> String {
    if let Some(unformatted) = remove_format(text, style) {
        unformatted
    } else {
        apply_format(text, style)
    }
}

// Check formatting at position
pub fn check_formatting(_text: &str, _position: usize) -> FormatInfo {
    // TODO: Implement logic to check formatting at the given position
    FormatInfo {
        is_bold: false,
        is_italic: false,
        is_underline: false,
        is_code: false,
    }
}