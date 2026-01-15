//! Neo-Brutalist Dark Theme for Stringr
//!
//! This module defines the complete design system following neo-brutalism principles:
//! - Bold, high-contrast colors
//! - Hard shadows (offset, not blurred)
//! - Thick borders (2-4px)
//! - No gradients
//! - Sharp corners (minimal border radius)
//! - Bold typography

// Allow unused - these are design tokens for future use
#![allow(dead_code)]

// ============================================================================
// COLOR PALETTE
// ============================================================================

/// Core background and surface colors
pub mod colors {
    // -------------------------------------------------------------------------
    // Base Colors (Dark Mode)
    // -------------------------------------------------------------------------

    /// Main background - deep dark
    pub const BACKGROUND: &str = "#141419";

    /// Slightly elevated surface
    pub const SURFACE: &str = "#1f1f24";

    /// Higher elevation surface (cards, modals)
    pub const SURFACE_ELEVATED: &str = "#2a2a32";

    /// Highest elevation (dropdowns, popovers)
    pub const SURFACE_OVERLAY: &str = "#35353f";

    // -------------------------------------------------------------------------
    // Primary Accent Colors
    // -------------------------------------------------------------------------

    /// Primary accent - bold yellow (neo-brutalism signature)
    pub const PRIMARY: &str = "#ffd900";

    /// Primary hover state
    pub const PRIMARY_HOVER: &str = "#ffed4a";

    /// Primary active/pressed state
    pub const PRIMARY_ACTIVE: &str = "#e6c300";

    /// Primary muted (for subtle accents)
    pub const PRIMARY_MUTED: &str = "#998200";

    // -------------------------------------------------------------------------
    // Secondary Accent Colors
    // -------------------------------------------------------------------------

    /// Secondary accent - cyan/teal
    pub const ACCENT: &str = "#00e69a";

    /// Accent hover state
    pub const ACCENT_HOVER: &str = "#33ffb8";

    /// Accent active state
    pub const ACCENT_ACTIVE: &str = "#00cc88";

    /// Accent muted
    pub const ACCENT_MUTED: &str = "#008f5f";

    // -------------------------------------------------------------------------
    // Extended Color Palette (Neo-Brutalism)
    // -------------------------------------------------------------------------

    /// Electric blue - for links and interactive elements
    pub const BLUE: &str = "#4d9fff";

    /// Blue hover
    pub const BLUE_HOVER: &str = "#7ab8ff";

    /// Blue muted
    pub const BLUE_MUTED: &str = "#2d5f99";

    /// Hot pink - for highlights and special accents
    pub const PINK: &str = "#ff4d9f";

    /// Pink hover
    pub const PINK_HOVER: &str = "#ff7ab8";

    /// Pink muted
    pub const PINK_MUTED: &str = "#992d5f";

    /// Vivid purple - for creative elements
    pub const PURPLE: &str = "#a855f7";

    /// Purple hover
    pub const PURPLE_HOVER: &str = "#c084fc";

    /// Purple muted
    pub const PURPLE_MUTED: &str = "#6b21a8";

    /// Bright orange - for warnings and attention
    pub const ORANGE: &str = "#ff8c00";

    /// Orange hover
    pub const ORANGE_HOVER: &str = "#ffa033";

    /// Orange muted
    pub const ORANGE_MUTED: &str = "#995300";

    /// Lime green - for positive actions
    pub const LIME: &str = "#a3e635";

    /// Lime hover
    pub const LIME_HOVER: &str = "#bef264";

    /// Lime muted
    pub const LIME_MUTED: &str = "#628a20";

    // -------------------------------------------------------------------------
    // Text Colors
    // -------------------------------------------------------------------------

    /// Primary text color - high contrast white
    pub const TEXT: &str = "#f2f2f2";

    /// Secondary text - slightly muted
    pub const TEXT_SECONDARY: &str = "#b3b3b3";

    /// Muted text - for hints and placeholders
    pub const TEXT_MUTED: &str = "#8a8a8a";

    /// Disabled text
    pub const TEXT_DISABLED: &str = "#5a5a5a";

    /// Inverted text (for primary buttons)
    pub const TEXT_INVERTED: &str = "#141419";

    // -------------------------------------------------------------------------
    // Semantic Colors
    // -------------------------------------------------------------------------

    /// Error color - bold red
    pub const ERROR: &str = "#ff4d4d";

    /// Error hover
    pub const ERROR_HOVER: &str = "#ff7070";

    /// Error background (subtle)
    pub const ERROR_BG: &str = "#3d1f1f";

    /// Success color - matches accent
    pub const SUCCESS: &str = "#00e69a";

    /// Success hover
    pub const SUCCESS_HOVER: &str = "#33ffb8";

    /// Success background (subtle)
    pub const SUCCESS_BG: &str = "#1f3d2f";

    /// Warning color - bright orange
    pub const WARNING: &str = "#ff8c00";

    /// Warning hover
    pub const WARNING_HOVER: &str = "#ffa033";

    /// Warning background (subtle)
    pub const WARNING_BG: &str = "#3d2f1f";

    /// Info color - electric blue
    pub const INFO: &str = "#4d9fff";

    /// Info hover
    pub const INFO_HOVER: &str = "#7ab8ff";

    /// Info background (subtle)
    pub const INFO_BG: &str = "#1f2f3d";

    // -------------------------------------------------------------------------
    // Border Colors
    // -------------------------------------------------------------------------

    /// Default border - visible but not harsh
    pub const BORDER: &str = "#4d4d59";

    /// Strong border - for focused/active states
    pub const BORDER_STRONG: &str = "#6b6b7a";

    /// Subtle border - for dividers
    pub const BORDER_SUBTLE: &str = "#35353f";

    /// Border on focus (matches primary)
    pub const BORDER_FOCUS: &str = "#ffd900";

    // -------------------------------------------------------------------------
    // Selection Colors
    // -------------------------------------------------------------------------

    /// Text selection background
    pub const SELECTION_BG: &str = "#ffd90040";

    /// Text selection in focused editor
    pub const SELECTION_BG_FOCUSED: &str = "#ffd90060";

    /// Cursor color
    pub const CURSOR: &str = "#ffd900";

    /// Line highlight (current line)
    pub const LINE_HIGHLIGHT: &str = "#ffffff08";

    // -------------------------------------------------------------------------
    // Syntax Highlighting Colors (for future code support)
    // -------------------------------------------------------------------------

    /// Keywords
    pub const SYNTAX_KEYWORD: &str = "#ff4d9f";

    /// Strings
    pub const SYNTAX_STRING: &str = "#a3e635";

    /// Numbers
    pub const SYNTAX_NUMBER: &str = "#ff8c00";

    /// Comments
    pub const SYNTAX_COMMENT: &str = "#6b6b7a";

    /// Functions
    pub const SYNTAX_FUNCTION: &str = "#4d9fff";

    /// Types
    pub const SYNTAX_TYPE: &str = "#a855f7";

    /// Variables
    pub const SYNTAX_VARIABLE: &str = "#f2f2f2";

    /// Constants
    pub const SYNTAX_CONSTANT: &str = "#00e69a";
}

// ============================================================================
// BORDERS
// ============================================================================

/// Border configuration following neo-brutalism (thick, solid borders)
pub mod borders {
    /// Standard border width - thick for neo-brutalism
    pub const WIDTH: u32 = 3;

    /// Thin border width - for subtle elements
    pub const WIDTH_THIN: u32 = 2;

    /// Thick border width - for emphasis
    pub const WIDTH_THICK: u32 = 4;

    /// Extra thick border - for major UI elements
    pub const WIDTH_HEAVY: u32 = 5;

    /// Default border radius - sharp corners for neo-brutalism
    pub const RADIUS: u32 = 0;

    /// Slight radius for softer elements (optional)
    pub const RADIUS_SM: u32 = 4;

    /// Medium radius (use sparingly)
    pub const RADIUS_MD: u32 = 8;

    /// Border style (always solid in neo-brutalism)
    pub const STYLE: &str = "solid";
}

// ============================================================================
// SHADOWS
// ============================================================================

/// Shadow configuration - neo-brutalism uses hard offset shadows, NOT blurred
pub mod shadows {
    /// No shadow
    pub const NONE: &str = "none";

    /// Small offset shadow (subtle elevation)
    pub const SM: &str = "3px 3px 0px #000000";

    /// Medium offset shadow (cards, buttons)
    pub const MD: &str = "4px 4px 0px #000000";

    /// Large offset shadow (modals, dropdowns)
    pub const LG: &str = "6px 6px 0px #000000";

    /// Extra large shadow (major elevated elements)
    pub const XL: &str = "8px 8px 0px #000000";

    /// Primary colored shadow (for accent elements)
    pub const PRIMARY: &str = "4px 4px 0px #ffd900";

    /// Accent colored shadow
    pub const ACCENT: &str = "4px 4px 0px #00e69a";

    /// Error colored shadow
    pub const ERROR: &str = "4px 4px 0px #ff4d4d";

    /// Inset shadow (pressed states)
    pub const INSET: &str = "inset 2px 2px 0px #000000";

    /// Hover shadow offset (for interactive feedback)
    pub const HOVER_OFFSET_X: i32 = -2;
    pub const HOVER_OFFSET_Y: i32 = -2;
}

// ============================================================================
// TYPOGRAPHY
// ============================================================================

/// Typography configuration - bold and modern
pub mod typography {
    // -------------------------------------------------------------------------
    // Font Families
    // -------------------------------------------------------------------------

    /// Primary font family - for UI elements
    pub const FONT_FAMILY: &str = "'Inter', 'SF Pro Display', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif";

    /// Monospace font - for editor content
    pub const FONT_FAMILY_MONO: &str = "'JetBrains Mono', 'Fira Code', 'SF Mono', 'Consolas', 'Monaco', monospace";

    /// Display font - for headings (more brutalist)
    pub const FONT_FAMILY_DISPLAY: &str = "'Space Grotesk', 'Inter', sans-serif";

    // -------------------------------------------------------------------------
    // Font Sizes (in rem)
    // -------------------------------------------------------------------------

    /// Extra small text
    pub const SIZE_XS: f32 = 0.75;

    /// Small text
    pub const SIZE_SM: f32 = 0.875;

    /// Base/body text
    pub const SIZE_BASE: f32 = 1.0;

    /// Large text
    pub const SIZE_LG: f32 = 1.125;

    /// Extra large text
    pub const SIZE_XL: f32 = 1.25;

    /// 2XL text
    pub const SIZE_2XL: f32 = 1.5;

    /// 3XL text (headings)
    pub const SIZE_3XL: f32 = 1.875;

    /// 4XL text (large headings)
    pub const SIZE_4XL: f32 = 2.25;

    /// 5XL text (hero headings)
    pub const SIZE_5XL: f32 = 3.0;

    /// 6XL text (display headings)
    pub const SIZE_6XL: f32 = 3.75;

    // -------------------------------------------------------------------------
    // Font Weights
    // -------------------------------------------------------------------------

    /// Light weight
    pub const WEIGHT_LIGHT: u32 = 300;

    /// Normal weight
    pub const WEIGHT_NORMAL: u32 = 400;

    /// Medium weight
    pub const WEIGHT_MEDIUM: u32 = 500;

    /// Semi-bold weight
    pub const WEIGHT_SEMIBOLD: u32 = 600;

    /// Bold weight - common in neo-brutalism
    pub const WEIGHT_BOLD: u32 = 700;

    /// Extra bold weight
    pub const WEIGHT_EXTRABOLD: u32 = 800;

    /// Black weight - maximum impact
    pub const WEIGHT_BLACK: u32 = 900;

    // -------------------------------------------------------------------------
    // Line Heights
    // -------------------------------------------------------------------------

    /// Tight line height (headings)
    pub const LINE_HEIGHT_TIGHT: f32 = 1.1;

    /// Snug line height
    pub const LINE_HEIGHT_SNUG: f32 = 1.25;

    /// Normal line height (body)
    pub const LINE_HEIGHT_NORMAL: f32 = 1.5;

    /// Relaxed line height (reading)
    pub const LINE_HEIGHT_RELAXED: f32 = 1.625;

    /// Loose line height
    pub const LINE_HEIGHT_LOOSE: f32 = 2.0;

    // -------------------------------------------------------------------------
    // Letter Spacing
    // -------------------------------------------------------------------------

    /// Tighter spacing
    pub const TRACKING_TIGHTER: f32 = -0.05;

    /// Tight spacing
    pub const TRACKING_TIGHT: f32 = -0.025;

    /// Normal spacing
    pub const TRACKING_NORMAL: f32 = 0.0;

    /// Wide spacing (uppercase text)
    pub const TRACKING_WIDE: f32 = 0.025;

    /// Wider spacing
    pub const TRACKING_WIDER: f32 = 0.05;

    /// Widest spacing (all caps headings)
    pub const TRACKING_WIDEST: f32 = 0.1;
}

// ============================================================================
// SPACING
// ============================================================================

/// Spacing scale following an 8px grid system
pub mod spacing {
    /// No spacing
    pub const NONE: u32 = 0;

    /// 1px - hairline
    pub const PX: u32 = 1;

    /// 2px - minimal
    pub const SPACE_0_5: u32 = 2;

    /// 4px - extra small
    pub const SPACE_1: u32 = 4;

    /// 6px
    pub const SPACE_1_5: u32 = 6;

    /// 8px - small (base unit)
    pub const SPACE_2: u32 = 8;

    /// 10px
    pub const SPACE_2_5: u32 = 10;

    /// 12px - medium small
    pub const SPACE_3: u32 = 12;

    /// 14px
    pub const SPACE_3_5: u32 = 14;

    /// 16px - medium
    pub const SPACE_4: u32 = 16;

    /// 20px
    pub const SPACE_5: u32 = 20;

    /// 24px - medium large
    pub const SPACE_6: u32 = 24;

    /// 28px
    pub const SPACE_7: u32 = 28;

    /// 32px - large
    pub const SPACE_8: u32 = 32;

    /// 36px
    pub const SPACE_9: u32 = 36;

    /// 40px
    pub const SPACE_10: u32 = 40;

    /// 44px
    pub const SPACE_11: u32 = 44;

    /// 48px - extra large
    pub const SPACE_12: u32 = 48;

    /// 56px
    pub const SPACE_14: u32 = 56;

    /// 64px - 2x large
    pub const SPACE_16: u32 = 64;

    /// 80px
    pub const SPACE_20: u32 = 80;

    /// 96px - 3x large
    pub const SPACE_24: u32 = 96;

    /// 112px
    pub const SPACE_28: u32 = 112;

    /// 128px - 4x large
    pub const SPACE_32: u32 = 128;

    /// 160px
    pub const SPACE_40: u32 = 160;

    /// 192px
    pub const SPACE_48: u32 = 192;

    /// 224px
    pub const SPACE_56: u32 = 224;

    /// 256px - 5x large
    pub const SPACE_64: u32 = 256;
}

// ============================================================================
// SIZING
// ============================================================================

/// Common sizes for UI elements
pub mod sizing {
    // -------------------------------------------------------------------------
    // Button Heights
    // -------------------------------------------------------------------------

    /// Extra small button height
    pub const BUTTON_XS: u32 = 24;

    /// Small button height
    pub const BUTTON_SM: u32 = 32;

    /// Medium button height (default)
    pub const BUTTON_MD: u32 = 40;

    /// Large button height
    pub const BUTTON_LG: u32 = 48;

    /// Extra large button height
    pub const BUTTON_XL: u32 = 56;

    // -------------------------------------------------------------------------
    // Input Heights
    // -------------------------------------------------------------------------

    /// Small input height
    pub const INPUT_SM: u32 = 32;

    /// Medium input height (default)
    pub const INPUT_MD: u32 = 40;

    /// Large input height
    pub const INPUT_LG: u32 = 48;

    // -------------------------------------------------------------------------
    // Icon Sizes
    // -------------------------------------------------------------------------

    /// Extra small icon
    pub const ICON_XS: u32 = 12;

    /// Small icon
    pub const ICON_SM: u32 = 16;

    /// Medium icon (default)
    pub const ICON_MD: u32 = 20;

    /// Large icon
    pub const ICON_LG: u32 = 24;

    /// Extra large icon
    pub const ICON_XL: u32 = 32;

    /// 2x large icon
    pub const ICON_2XL: u32 = 40;

    // -------------------------------------------------------------------------
    // Touch Target (minimum for accessibility)
    // -------------------------------------------------------------------------

    /// Minimum touch target size (44px per Apple HIG)
    pub const TOUCH_TARGET_MIN: u32 = 44;
}

// ============================================================================
// Z-INDEX
// ============================================================================

/// Z-index layering system
pub mod z_index {
    /// Base layer (default content)
    pub const BASE: i32 = 0;

    /// Sticky elements (headers)
    pub const STICKY: i32 = 10;

    /// Fixed elements (sidebars)
    pub const FIXED: i32 = 20;

    /// Dropdowns and popovers
    pub const DROPDOWN: i32 = 30;

    /// Overlays and backdrops
    pub const OVERLAY: i32 = 40;

    /// Modals and dialogs
    pub const MODAL: i32 = 50;

    /// Tooltips
    pub const TOOLTIP: i32 = 60;

    /// Toast notifications
    pub const TOAST: i32 = 70;

    /// Maximum priority (debugging, loading)
    pub const MAX: i32 = 100;
}

// ============================================================================
// TRANSITIONS
// ============================================================================

/// Animation and transition configuration
pub mod transitions {
    /// Fast transition duration (instant feedback)
    pub const DURATION_FAST: &str = "100ms";

    /// Normal transition duration
    pub const DURATION_NORMAL: &str = "150ms";

    /// Slow transition duration (deliberate animations)
    pub const DURATION_SLOW: &str = "300ms";

    /// Slower transition (page transitions)
    pub const DURATION_SLOWER: &str = "500ms";

    /// Easing function - snappy (neo-brutalism prefers quick, direct)
    pub const EASING_DEFAULT: &str = "cubic-bezier(0.4, 0, 0.2, 1)";

    /// Easing for entering elements
    pub const EASING_IN: &str = "cubic-bezier(0.4, 0, 1, 1)";

    /// Easing for exiting elements
    pub const EASING_OUT: &str = "cubic-bezier(0, 0, 0.2, 1)";

    /// Linear easing (mechanical feel)
    pub const EASING_LINEAR: &str = "linear";

    /// Common transition - color changes
    pub const COLOR: &str = "color 150ms cubic-bezier(0.4, 0, 0.2, 1)";

    /// Common transition - background changes
    pub const BACKGROUND: &str = "background-color 150ms cubic-bezier(0.4, 0, 0.2, 1)";

    /// Common transition - transform (hover effects)
    pub const TRANSFORM: &str = "transform 100ms cubic-bezier(0.4, 0, 0.2, 1)";

    /// Common transition - shadow changes
    pub const SHADOW: &str = "box-shadow 100ms cubic-bezier(0.4, 0, 0.2, 1)";

    /// Common transition - all properties
    pub const ALL: &str = "all 150ms cubic-bezier(0.4, 0, 0.2, 1)";
}

// ============================================================================
// EDITOR SPECIFIC
// ============================================================================

/// Editor-specific configuration
pub mod editor {
    /// Default font size for editor (in pixels)
    pub const FONT_SIZE_DEFAULT: u32 = 14;

    /// Minimum font size
    pub const FONT_SIZE_MIN: u32 = 10;

    /// Maximum font size
    pub const FONT_SIZE_MAX: u32 = 32;

    /// Default line height for editor
    pub const LINE_HEIGHT: f32 = 1.6;

    /// Gutter width (line numbers)
    pub const GUTTER_WIDTH: u32 = 60;

    /// Cursor width
    pub const CURSOR_WIDTH: u32 = 2;

    /// Cursor blink rate (milliseconds)
    pub const CURSOR_BLINK_RATE: u32 = 530;

    /// Tab size (in spaces)
    pub const TAB_SIZE: u32 = 4;

    /// Scroll padding (keep cursor this far from edge)
    pub const SCROLL_PADDING: u32 = 5;
}

// ============================================================================
// DEPRECATED - Keeping for backwards compatibility
// ============================================================================

/// Border width (use borders::WIDTH instead)
#[deprecated(since = "0.1.0", note = "Use borders::WIDTH instead")]
pub const BORDER_WIDTH: u32 = 3;