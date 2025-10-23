/// Design system for AutoEQ GPUI components
/// Matches the design language of the TypeScript/Tauri UI
use gpui::*;

// Color Palette (Theme-aware - dynamically loads current theme)
pub mod colors {
    use gpui::*;
    use crate::app_theme;

    // Primary colors
    pub fn bg_primary() -> Hsla {
        app_theme::colors().bg_primary
    }
    pub fn bg_secondary() -> Hsla {
        app_theme::colors().bg_secondary
    }
    pub fn bg_accent() -> Hsla {
        app_theme::colors().bg_accent
    }

    // Text colors
    pub fn text_primary() -> Hsla {
        app_theme::colors().text_primary
    }
    pub fn text_secondary() -> Hsla {
        app_theme::colors().text_secondary
    }

    // Border colors
    pub fn border() -> Hsla {
        app_theme::colors().border
    }
    pub fn border_active() -> Hsla {
        app_theme::colors().border_active
    }

    // Button colors
    pub fn button_primary() -> Hsla {
        app_theme::colors().button_primary
    }
    pub fn button_primary_hover() -> Hsla {
        app_theme::colors().button_primary_hover
    }
    pub fn button_secondary() -> Hsla {
        app_theme::colors().button_secondary
    }
    pub fn button_secondary_hover() -> Hsla {
        app_theme::colors().button_secondary_hover
    }

    // Status colors
    pub fn success() -> Hsla {
        app_theme::colors().success
    }
    pub fn warning() -> Hsla {
        app_theme::colors().warning
    }
    pub fn danger() -> Hsla {
        app_theme::colors().danger
    }
    pub fn info() -> Hsla {
        app_theme::colors().info
    }

    // Selection/highlight colors
    pub fn select_bg() -> Hsla {
        app_theme::colors().select_bg
    }
    pub fn select_border() -> Hsla {
        app_theme::colors().select_border
    }
    pub fn hover_bg() -> Hsla {
        app_theme::colors().hover_bg
    }
}

// Spacing constants (with compact variants for dense layouts)
pub mod spacing {
    use gpui::*;

    pub const XXS: Pixels = px(2.0);
    pub const XS: Pixels = px(4.0);
    pub const SM: Pixels = px(6.0);  // Reduced from 8px
    pub const MD: Pixels = px(8.0);  // Reduced from 12px
    pub const LG: Pixels = px(12.0); // Reduced from 16px
    pub const XL: Pixels = px(16.0); // Reduced from 24px
    pub const XXL: Pixels = px(24.0);
}

// Border radius
pub const RADIUS: Pixels = px(6.0);

// Typography
pub mod fonts {
    // Font sizes
    pub const SIZE_XS: Pixels = px(11.0);
    pub const SIZE_SM: Pixels = px(12.0);
    pub const SIZE_BASE: Pixels = px(13.0);
    pub const SIZE_MD: Pixels = px(14.0);
    pub const SIZE_LG: Pixels = px(16.0);
    pub const SIZE_XL: Pixels = px(20.0);
    pub const SIZE_2XL: Pixels = px(24.0);

    use gpui::*;
}

// Common component styles
pub trait StyledDiv {
    /// Apply card-like styling (white background, border, shadow)
    fn card(self) -> Self;

    /// Apply section group styling (with subtle background)
    fn section_group(self) -> Self;

    /// Apply panel styling
    fn panel(self) -> Self;
}

impl StyledDiv for Div {
    fn card(self) -> Self {
        self.bg(colors::bg_secondary())
            .border_1()
            .border_color(colors::border())
            .rounded(RADIUS)
            .shadow_sm()
    }

    fn section_group(self) -> Self {
        self.bg(colors::bg_secondary())
            .border_1()
            .border_color(colors::border())
            .rounded(RADIUS)
            .p(spacing::MD)
    }

    fn panel(self) -> Self {
        self.bg(colors::bg_secondary())
            .border_r_1()
            .border_color(colors::border())
    }
}

// Component helpers
pub mod components {
    use super::*;

    /// Create a section header matching the TypeScript UI
    pub fn section_header(title: impl Into<SharedString>) -> Div {
        div()
            .bg(colors::bg_accent())
            .px(spacing::MD)
            .py(spacing::SM)
            .font_weight(FontWeight::SEMIBOLD)
            .text_size(fonts::SIZE_MD)
            .text_color(colors::text_primary())
            .border_b_1()
            .border_color(colors::border())
            .child(title.into())
    }

    /// Create a primary button matching the TypeScript UI
    pub fn primary_button(label: impl Into<SharedString>) -> Div {
        div()
            .px(spacing::MD)
            .py(spacing::MD)
            .rounded(RADIUS)
            .bg(colors::button_primary())
            .text_color(rgb(0xffffff))
            .text_size(fonts::SIZE_BASE)
            .font_weight(FontWeight::SEMIBOLD)
            .cursor_pointer()
            .hover(|s| s.bg(colors::button_primary_hover()))
            .child(label.into())
    }

    /// Create a secondary button
    pub fn secondary_button(label: impl Into<SharedString>) -> Div {
        div()
            .px(spacing::MD)
            .py(spacing::MD)
            .rounded(RADIUS)
            .bg(colors::button_secondary())
            .text_color(rgb(0xffffff))
            .text_size(fonts::SIZE_BASE)
            .font_weight(FontWeight::SEMIBOLD)
            .cursor_pointer()
            .hover(|s| s.bg(colors::button_secondary_hover()))
            .child(label.into())
    }

    /// Create a text button
    pub fn text_button(label: impl Into<SharedString>) -> Div {
        div()
            .px(spacing::SM)
            .py(spacing::XS)
            .rounded(RADIUS)
            .text_color(colors::button_primary())
            .text_size(fonts::SIZE_BASE)
            .cursor_pointer()
            .hover(|s| s.bg(colors::hover_bg()))
            .child(label.into())
    }

    /// Create an input field
    pub fn input_field() -> Div {
        div()
            .px(spacing::MD)
            .py(spacing::SM)
            .rounded(RADIUS)
            .border_1()
            .border_color(colors::border())
            .bg(colors::bg_secondary())
            .text_color(colors::text_primary())
            .text_size(fonts::SIZE_BASE)
    }

    /// Create a label
    pub fn label(text: impl Into<SharedString>) -> Div {
        div()
            .text_size(fonts::SIZE_SM)
            .text_color(colors::text_secondary())
            .mb(spacing::XS)
            .child(text.into())
    }
}
