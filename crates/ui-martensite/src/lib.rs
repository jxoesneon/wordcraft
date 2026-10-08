//! Sovereign retained-mode word processor UI for WordCraft built on Martensite.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod command_reg;
pub mod document;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use wordcraft_engine::{Engine, Tool};

/// Application state container managing the Martensite GUI pipeline.
pub struct WordcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub doc: document::DocumentState,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: Tool,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    pub focus_mode_active: bool,
    pub is_dirty: bool,
}

impl WordcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            doc: document::DocumentState::new(),
            theme: theme::CraftTheme::paper_light(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: Tool::Select,
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            focus_mode_active: false,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    /// Word zoom range: 10%–500%.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.1, 5.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
    }

    pub fn toggle_focus_mode(&mut self) -> bool {
        self.focus_mode_active = !self.focus_mode_active;
        self.focus_mode_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> WordcraftApp {
        WordcraftApp::new(Engine::new(wordcraft_doc::Document::new()))
    }

    #[test]
    fn test_app_initialization() {
        let app = app();
        assert_eq!(app.active_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(!app.focus_mode_active);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_zoom_clamping() {
        let mut app = app();

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.001);
        assert_eq!(app.zoom_level, 0.1);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 5.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let mut app = app();

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles() {
        let mut app = app();

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(!app.focus_mode_active);
        assert!(app.toggle_focus_mode());
        assert!(app.focus_mode_active);
        assert!(!app.toggle_focus_mode());
    }
}
