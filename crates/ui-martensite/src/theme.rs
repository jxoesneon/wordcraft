//! Martensite design system tokens calibrated for document editing workflows.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_f32_array(self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }

    pub fn luminance(&self) -> f32 {
        0.2126 * (self.r as f32 / 255.0) + 0.7152 * (self.g as f32 / 255.0) + 0.0722 * (self.b as f32 / 255.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraftTheme {
    pub surface_app_bg: RgbaColor,
    pub surface_panel: RgbaColor,
    pub surface_page: RgbaColor,
    pub surface_desk: RgbaColor,
    pub surface_input: RgbaColor,
    pub surface_active_tab: RgbaColor,
    pub surface_inactive_tab: RgbaColor,
    pub border_divider: RgbaColor,
    pub text_primary: RgbaColor,
    pub text_dimmed: RgbaColor,
    pub text_scrubby: RgbaColor,
    pub accent_active: RgbaColor,
    pub accent_focus: RgbaColor,
    pub selection_highlight: RgbaColor,
    pub corner_radius: f32,
    pub widget_spacing: f32,
}

impl CraftTheme {
    /// Light theme: white page on a neutral desk surround.
    pub fn paper_light() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(243, 243, 243),
            surface_panel: RgbaColor::rgb(250, 250, 250),
            surface_page: RgbaColor::rgb(255, 255, 255),
            surface_desk: RgbaColor::rgb(226, 228, 233),
            surface_input: RgbaColor::rgb(255, 255, 255),
            surface_active_tab: RgbaColor::rgb(250, 250, 250),
            surface_inactive_tab: RgbaColor::rgb(232, 232, 232),
            border_divider: RgbaColor::rgb(210, 210, 210),
            text_primary: RgbaColor::rgb(32, 32, 32),
            text_dimmed: RgbaColor::rgb(96, 96, 96),
            text_scrubby: RgbaColor::rgb(70, 70, 70),
            accent_active: RgbaColor::rgb(43, 87, 154),
            accent_focus: RgbaColor::rgb(70, 120, 200),
            selection_highlight: RgbaColor::rgba(43, 87, 154, 76),
            corner_radius: 4.0,
            widget_spacing: 6.0,
        }
    }

    /// Dark studio theme: dimmed page on an obsidian desk.
    pub fn studio_obsidian() -> Self {
        Self {
            surface_app_bg: RgbaColor::rgb(18, 21, 27),
            surface_panel: RgbaColor::rgb(22, 26, 34),
            surface_page: RgbaColor::rgb(30, 33, 40),
            surface_desk: RgbaColor::rgb(14, 17, 22),
            surface_input: RgbaColor::rgb(10, 12, 16),
            surface_active_tab: RgbaColor::rgb(22, 26, 34),
            surface_inactive_tab: RgbaColor::rgb(33, 39, 52),
            border_divider: RgbaColor::rgb(42, 50, 65),
            text_primary: RgbaColor::rgb(240, 246, 252),
            text_dimmed: RgbaColor::rgb(139, 148, 158),
            text_scrubby: RgbaColor::rgb(170, 180, 195),
            accent_active: RgbaColor::rgb(0, 229, 255),
            accent_focus: RgbaColor::rgb(124, 58, 237),
            selection_highlight: RgbaColor::rgba(0, 229, 255, 76),
            corner_radius: 6.0,
            widget_spacing: 8.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversions() {
        let c = RgbaColor::rgba(255, 128, 0, 255);
        let f = c.to_f32_array();
        assert_eq!(f[0], 1.0);
        assert!((f[1] - 0.50196).abs() < 1e-4);
        assert_eq!(f[2], 0.0);
        assert_eq!(f[3], 1.0);
    }

    #[test]
    fn test_luminance_calculation() {
        let black = RgbaColor::rgb(0, 0, 0);
        let white = RgbaColor::rgb(255, 255, 255);
        assert_eq!(black.luminance(), 0.0);
        assert!((white.luminance() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_theme_contrast_sanity() {
        let theme = CraftTheme::paper_light();
        assert!(theme.surface_page.luminance() > theme.text_primary.luminance() + 0.5);
        assert!(theme.surface_page.luminance() > theme.surface_desk.luminance());
    }
}
