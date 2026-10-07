//! Word processor theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub page_background: Color,
    pub desk_surround: Color,
    pub selection_highlight: Color,
}

impl Theme {
    pub fn word_studio() -> Self {
        Self {
            page_background: Color(255, 255, 255),
            desk_surround: Color(32, 35, 42),
            selection_highlight: Color(0, 120, 215),
        }
    }
}
