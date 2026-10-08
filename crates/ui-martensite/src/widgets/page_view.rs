//! Paged document view widget with subpixel pan/zoom, rulers, and caret blink.

pub struct PageViewWidget {
    pub zoom: f32,
    pub pan_offset: [f32; 2],
    /// Page size in points (e.g. Letter = 612×792).
    pub page_size: [u32; 2],
    pub rulers_visible: bool,
    pub caret_phase: f32,
}

impl PageViewWidget {
    pub fn new(width: u32, height: u32) -> Self {
        Self { zoom: 1.0, pan_offset: [0.0, 0.0], page_size: [width, height], rulers_visible: true, caret_phase: 0.0 }
    }

    /// Word zoom range: 10%–500%.
    pub fn zoom_at(&mut self, factor: f32, cursor: [f32; 2]) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * factor).clamp(0.1, 5.0);
        let ratio = new_zoom / old_zoom;

        self.pan_offset[0] = cursor[0] - (cursor[0] - self.pan_offset[0]) * ratio;
        self.pan_offset[1] = cursor[1] - (cursor[1] - self.pan_offset[1]) * ratio;
        self.zoom = new_zoom;
    }

    pub fn advance_caret_blink(&mut self, dt: f32) {
        self.caret_phase = (self.caret_phase + dt * 2.0) % 1.0;
    }

    pub fn screen_to_page(&self, screen: [f32; 2]) -> [f32; 2] {
        [(screen[0] - self.pan_offset[0]) / self.zoom, (screen[1] - self.pan_offset[1]) / self.zoom]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_coordinates_and_zoom() {
        let mut page = PageViewWidget::new(612, 792);
        assert_eq!(page.screen_to_page([100.0, 100.0]), [100.0, 100.0]);

        page.zoom_at(2.0, [0.0, 0.0]);
        assert_eq!(page.zoom, 2.0);
        assert_eq!(page.screen_to_page([100.0, 100.0]), [50.0, 50.0]);

        page.advance_caret_blink(0.25);
        assert!((page.caret_phase - 0.5).abs() < 1e-4);
    }
}
