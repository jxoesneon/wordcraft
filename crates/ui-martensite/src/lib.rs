//! Sovereign retained-mode word processor UI for WordCraft built on Martensite.

pub mod command_reg;
pub mod document;
pub mod menus;
pub mod theme;

pub struct WordcraftApp {
    pub doc: document::DocumentState,
    pub zoom: f32,
    pub show_rulers: bool,
}

impl WordcraftApp {
    pub fn new() -> Self {
        Self {
            doc: document::DocumentState::new(),
            zoom: 1.0,
            show_rulers: true,
        }
    }

    pub fn set_zoom(&mut self, z: f32) {
        self.zoom = z.clamp(0.2, 5.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordcraft_app() {
        let mut app = WordcraftApp::new();
        assert_eq!(app.zoom, 1.0);
        app.set_zoom(1.5);
        assert_eq!(app.zoom, 1.5);
    }
}
