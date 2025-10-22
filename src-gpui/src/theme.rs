/// Theme system for AutoEQ GPUI application
/// Provides different color schemes that can be switched at runtime
use gpui::*;
use std::sync::RwLock;

/// Available theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    Light,
    Dark,
    Blue,
    HighContrast,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::Light => "Light",
            ThemeVariant::Dark => "Dark",
            ThemeVariant::Blue => "Blue",
            ThemeVariant::HighContrast => "High Contrast",
        }
    }

    pub fn all() -> Vec<ThemeVariant> {
        vec![
            ThemeVariant::Light,
            ThemeVariant::Dark,
            ThemeVariant::Blue,
            ThemeVariant::HighContrast,
        ]
    }
}

/// Global theme state
static CURRENT_THEME: RwLock<ThemeVariant> = RwLock::new(ThemeVariant::Light);

/// Get the current theme variant
pub fn current_theme() -> ThemeVariant {
    *CURRENT_THEME.read().unwrap()
}

/// Set the current theme variant
pub fn set_theme(variant: ThemeVariant) {
    *CURRENT_THEME.write().unwrap() = variant;
}

/// Theme colors structure
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Background colors
    pub bg_primary: Hsla,
    pub bg_secondary: Hsla,
    pub bg_accent: Hsla,

    // Text colors
    pub text_primary: Hsla,
    pub text_secondary: Hsla,

    // Border colors
    pub border: Hsla,
    pub border_active: Hsla,

    // Button colors
    pub button_primary: Hsla,
    pub button_primary_hover: Hsla,
    pub button_secondary: Hsla,
    pub button_secondary_hover: Hsla,

    // Status colors
    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
    pub info: Hsla,

    // Selection/highlight colors
    pub select_bg: Hsla,
    pub select_border: Hsla,
    pub hover_bg: Hsla,
}

impl ThemeColors {
    /// Get colors for a specific theme variant
    pub fn for_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Light => Self::light(),
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::Blue => Self::blue(),
            ThemeVariant::HighContrast => Self::high_contrast(),
        }
    }

    /// Light theme (default)
    fn light() -> Self {
        Self {
            bg_primary: rgb(0xf8f9fa).into(),
            bg_secondary: rgb(0xffffff).into(),
            bg_accent: rgb(0xe9ecef).into(),
            text_primary: rgb(0x212529).into(),
            text_secondary: rgb(0x6c757d).into(),
            border: rgb(0xdee2e6).into(),
            border_active: rgb(0x007bff).into(),
            button_primary: rgb(0x007bff).into(),
            button_primary_hover: rgb(0x0056b3).into(),
            button_secondary: rgb(0x6c757d).into(),
            button_secondary_hover: rgb(0x545b62).into(),
            success: rgb(0x28a745).into(),
            warning: rgb(0xffc107).into(),
            danger: rgb(0xdc3545).into(),
            info: rgb(0x17a2b8).into(),
            select_bg: rgb(0xe7f3ff).into(),
            select_border: rgb(0x2196f3).into(),
            hover_bg: rgb(0xf5f5f5).into(),
        }
    }

    /// Dark theme
    fn dark() -> Self {
        Self {
            bg_primary: rgb(0x1e1e1e).into(),
            bg_secondary: rgb(0x2d2d2d).into(),
            bg_accent: rgb(0x3e3e3e).into(),
            text_primary: rgb(0xe0e0e0).into(),
            text_secondary: rgb(0xa0a0a0).into(),
            border: rgb(0x404040).into(),
            border_active: rgb(0x4a9eff).into(),
            button_primary: rgb(0x4a9eff).into(),
            button_primary_hover: rgb(0x5aafff).into(),
            button_secondary: rgb(0x5a5a5a).into(),
            button_secondary_hover: rgb(0x6a6a6a).into(),
            success: rgb(0x3fb950).into(),
            warning: rgb(0xffb347).into(),
            danger: rgb(0xff4444).into(),
            info: rgb(0x4ac3db).into(),
            select_bg: rgb(0x264f78).into(),
            select_border: rgb(0x4a9eff).into(),
            hover_bg: rgb(0x383838).into(),
        }
    }

    /// Blue theme (cool tones)
    fn blue() -> Self {
        Self {
            bg_primary: rgb(0xf0f4f8).into(),
            bg_secondary: rgb(0xfcfeff).into(),
            bg_accent: rgb(0xd9e5f2).into(),
            text_primary: rgb(0x1a202c).into(),
            text_secondary: rgb(0x4a5568).into(),
            border: rgb(0xbfd4e8).into(),
            border_active: rgb(0x3182ce).into(),
            button_primary: rgb(0x3182ce).into(),
            button_primary_hover: rgb(0x2c5282).into(),
            button_secondary: rgb(0x5a7a9e).into(),
            button_secondary_hover: rgb(0x4a6a8e).into(),
            success: rgb(0x2f855a).into(),
            warning: rgb(0xd69e2e).into(),
            danger: rgb(0xc53030).into(),
            info: rgb(0x2c5282).into(),
            select_bg: rgb(0xdbeafe).into(),
            select_border: rgb(0x3182ce).into(),
            hover_bg: rgb(0xe6eff8).into(),
        }
    }

    /// High contrast theme (accessibility)
    fn high_contrast() -> Self {
        Self {
            bg_primary: rgb(0xffffff).into(),
            bg_secondary: rgb(0xffffff).into(),
            bg_accent: rgb(0xf0f0f0).into(),
            text_primary: rgb(0x000000).into(),
            text_secondary: rgb(0x333333).into(),
            border: rgb(0x000000).into(),
            border_active: rgb(0x0000ff).into(),
            button_primary: rgb(0x0000ff).into(),
            button_primary_hover: rgb(0x0000cc).into(),
            button_secondary: rgb(0x000000).into(),
            button_secondary_hover: rgb(0x333333).into(),
            success: rgb(0x008000).into(),
            warning: rgb(0xff8c00).into(),
            danger: rgb(0xff0000).into(),
            info: rgb(0x0000ff).into(),
            select_bg: rgb(0xffff99).into(),
            select_border: rgb(0x0000ff).into(),
            hover_bg: rgb(0xeeeeee).into(),
        }
    }
}

/// Get the current theme colors
pub fn colors() -> ThemeColors {
    ThemeColors::for_variant(current_theme())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_variant_name() {
        assert_eq!(ThemeVariant::Light.name(), "Light");
        assert_eq!(ThemeVariant::Dark.name(), "Dark");
        assert_eq!(ThemeVariant::Blue.name(), "Blue");
        assert_eq!(ThemeVariant::HighContrast.name(), "High Contrast");
    }

    #[test]
    fn test_all_theme_variants() {
        let variants = ThemeVariant::all();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&ThemeVariant::Light));
        assert!(variants.contains(&ThemeVariant::Dark));
        assert!(variants.contains(&ThemeVariant::Blue));
        assert!(variants.contains(&ThemeVariant::HighContrast));
    }

    #[test]
    fn test_set_and_get_theme() {
        // Test setting dark theme
        set_theme(ThemeVariant::Dark);
        assert_eq!(current_theme(), ThemeVariant::Dark);

        // Test setting blue theme
        set_theme(ThemeVariant::Blue);
        assert_eq!(current_theme(), ThemeVariant::Blue);

        // Test setting high contrast theme
        set_theme(ThemeVariant::HighContrast);
        assert_eq!(current_theme(), ThemeVariant::HighContrast);

        // Reset to light
        set_theme(ThemeVariant::Light);
        assert_eq!(current_theme(), ThemeVariant::Light);
    }

    #[test]
    fn test_theme_colors_construction() {
        // Test that all theme variants can be constructed without panicking
        let _light = ThemeColors::light();
        let _dark = ThemeColors::dark();
        let _blue = ThemeColors::blue();
        let _hc = ThemeColors::high_contrast();
    }
}
