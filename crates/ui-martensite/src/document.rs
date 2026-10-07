//! Document pagination and typography models.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub alignment: Alignment,
}

pub struct DocumentState {
    pub title: String,
    pub style: TextStyle,
    pub word_count: usize,
    pub page_count: usize,
}

impl DocumentState {
    pub fn new() -> Self {
        Self {
            title: "Untitled Document".to_string(),
            style: TextStyle {
                font_size: 11.0,
                bold: false,
                italic: false,
                underline: false,
                alignment: Alignment::Left,
            },
            word_count: 0,
            page_count: 1,
        }
    }

    pub fn toggle_bold(&mut self) -> bool {
        self.style.bold = !self.style.bold;
        self.style.bold
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.style.font_size = size.clamp(4.0, 144.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_styles() {
        let mut doc = DocumentState::new();
        assert!(!doc.style.bold);
        assert!(doc.toggle_bold());
        assert!(doc.style.bold);

        doc.set_font_size(16.0);
        assert_eq!(doc.style.font_size, 16.0);
    }
}
