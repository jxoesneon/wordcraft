//! Options bar widget adapting dynamically to the active tool.

use wordcraft_engine::Tool;

use crate::widgets::scrubby_input::ScrubbyInputWidget;

pub struct OptionsBarWidget {
    pub active_tool: Tool,
    pub font_size: ScrubbyInputWidget,
    pub line_spacing: ScrubbyInputWidget,
    pub space_before: ScrubbyInputWidget,
    pub space_after: ScrubbyInputWidget,
    pub ink_width: ScrubbyInputWidget,
    pub keep_with_next: bool,
    pub all_caps: bool,
}

impl OptionsBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Type,
            font_size: ScrubbyInputWidget::new("Size", 11.0, 4.0, 144.0, "pt"),
            line_spacing: ScrubbyInputWidget::new("Spacing", 1.0, 0.5, 3.0, "x"),
            space_before: ScrubbyInputWidget::new("Before", 0.0, 0.0, 200.0, "pt"),
            space_after: ScrubbyInputWidget::new("After", 8.0, 0.0, 200.0, "pt"),
            ink_width: ScrubbyInputWidget::new("Ink", 2.0, 0.5, 24.0, "pt"),
            keep_with_next: false,
            all_caps: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn toggle_keep_with_next(&mut self) -> bool {
        self.keep_with_next = !self.keep_with_next;
        self.keep_with_next
    }

    pub fn toggle_all_caps(&mut self) -> bool {
        self.all_caps = !self.all_caps;
        self.all_caps
    }
}

impl Default for OptionsBarWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_bar_defaults() {
        let mut bar = OptionsBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Type);
        assert_eq!(bar.font_size.value, 11.0);
        assert_eq!(bar.line_spacing.value, 1.0);
        assert!(!bar.keep_with_next);

        bar.set_tool(Tool::Pen);
        assert_eq!(bar.active_tool, Tool::Pen);

        assert!(bar.toggle_keep_with_next());
        assert!(bar.keep_with_next);
        assert!(!bar.toggle_keep_with_next());
    }
}
