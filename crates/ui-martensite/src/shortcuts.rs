//! Keystroke state machine for document-canvas tool switching.
//!
//! Single-key tool switches apply when the caret is not capturing text
//! (navigation mode, read mode, Draw-tab inking). Space and Z are
//! spring-loaded: held down they temporarily switch to Hand/Zoom and restore
//! the prior tool on release.

use wordcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Single-key tool switches (non-editing contexts only)
            "v" | "V" => Some(Tool::Select),
            "t" | "T" => Some(Tool::Type),
            "f" | "F" => Some(Tool::FormatPainter),
            "m" | "M" => Some(Tool::Highlighter),
            "p" | "P" => Some(Tool::Pen),
            "e" | "E" => Some(Tool::Eraser),
            "l" | "L" => Some(Tool::Lasso),
            "c" | "C" => Some(Tool::Crop),
            "d" | "D" => Some(Tool::DrawTable),
            "n" | "N" => Some(Tool::Comment),
            "h" | "H" => Some(Tool::Hand),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

impl Default for KeyboardEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Pen), Some(Tool::Select));
        assert_eq!(k.on_key_down("T", Tool::Select), Some(Tool::Type));
        assert_eq!(k.on_key_down("f", Tool::Pen), Some(Tool::FormatPainter));
        assert_eq!(k.on_key_down("m", Tool::Pen), Some(Tool::Highlighter));
        assert_eq!(k.on_key_down("p", Tool::Select), Some(Tool::Pen));
        assert_eq!(k.on_key_down("e", Tool::Pen), Some(Tool::Eraser));
        assert_eq!(k.on_key_down("l", Tool::Pen), Some(Tool::Lasso));
        assert_eq!(k.on_key_down("c", Tool::Select), Some(Tool::Crop));
        assert_eq!(k.on_key_down("d", Tool::Select), Some(Tool::DrawTable));
        assert_eq!(k.on_key_down("n", Tool::Select), Some(Tool::Comment));
        assert_eq!(k.on_key_down("h", Tool::Select), Some(Tool::Hand));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Select;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Type;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
