//! Tool strip widget: tool palette with color chips for font/highlight ink.

use wordcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Select, alternatives: &[] },
    ToolSlot { primary: Tool::Type, alternatives: &[] },
    ToolSlot { primary: Tool::FormatPainter, alternatives: &[] },
    ToolSlot { primary: Tool::Highlighter, alternatives: &[] },
    ToolSlot { primary: Tool::Pen, alternatives: &[Tool::Eraser, Tool::Lasso] },
    ToolSlot { primary: Tool::Crop, alternatives: &[] },
    ToolSlot { primary: Tool::DrawTable, alternatives: &[] },
    ToolSlot { primary: Tool::Comment, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Automatic font color is near-black.
    pub font_color: [u8; 4],
    /// Default highlight is yellow.
    pub highlight_color: [u8; 4],
    pub focus_mode: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self { active_tool: Tool::Select, double_column: false, font_color: [0, 0, 0, 255], highlight_color: [255, 255, 0, 255], focus_mode: false }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.font_color, &mut self.highlight_color);
    }

    pub fn reset_default_colors(&mut self) {
        self.font_color = [0, 0, 0, 255];
        self.highlight_color = [255, 255, 0, 255];
    }

    pub fn toggle_focus_mode(&mut self) -> bool {
        self.focus_mode = !self.focus_mode;
        self.focus_mode
    }
}

impl Default for ToolStripWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Select);
        assert!(!strip.double_column);
        assert_eq!(strip.font_color, [0, 0, 0, 255]);
        assert_eq!(strip.highlight_color, [255, 255, 0, 255]);

        strip.swap_colors();
        assert_eq!(strip.font_color, [255, 255, 0, 255]);
        assert_eq!(strip.highlight_color, [0, 0, 0, 255]);

        strip.reset_default_colors();
        assert_eq!(strip.font_color, [0, 0, 0, 255]);

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());
    }
}
