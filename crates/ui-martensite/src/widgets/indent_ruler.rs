//! Indent ruler widget: draggable first-line, hanging and right-indent
//! markers on the horizontal ruler, in points. Markers clamp to the text
//! width and keep a valid ordering.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndentMarkers {
    /// First-line indent relative to the left margin (may be negative for
    /// hanging indents).
    pub first_line: f32,
    /// Left (paragraph) indent from the left margin.
    pub left: f32,
    /// Right indent from the right edge of the text column.
    pub right: f32,
}

impl IndentMarkers {
    pub const fn body_text() -> Self {
        Self { first_line: 0.0, left: 0.0, right: 0.0 }
    }

    pub fn is_first_line_indented(&self) -> bool {
        self.first_line > 0.0
    }

    pub fn is_hanging(&self) -> bool {
        self.first_line < 0.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndentRulerWidget {
    /// Text-column width in points the markers travel across.
    pub text_width: f32,
    pub markers: IndentMarkers,
}

impl IndentRulerWidget {
    pub fn new(text_width: f32) -> Self {
        Self { text_width: text_width.max(0.0), markers: IndentMarkers::body_text() }
    }

    /// Set the left indent; the first-line offset is relative to it.
    pub fn set_left_indent(&mut self, pts: f32) {
        self.markers.left = pts.clamp(0.0, self.text_width - self.markers.right);
    }

    /// Set the first-line offset relative to the left indent marker.
    /// Positive = first line indented right; negative = hanging indent.
    pub fn set_first_line(&mut self, pts: f32) {
        let min = -self.markers.left;
        let max = self.text_width - self.markers.left - self.markers.right;
        self.markers.first_line = pts.clamp(min, max.max(min));
    }

    pub fn set_right_indent(&mut self, pts: f32) {
        self.markers.right = pts.clamp(0.0, self.text_width - self.markers.left - self.markers.first_line.max(0.0));
    }

    pub fn reset(&mut self) {
        self.markers = IndentMarkers::body_text();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_markers_and_clamping() {
        // 612pt page minus 2×72pt margins = 468pt text column.
        let mut ruler = IndentRulerWidget::new(468.0);
        assert_eq!(ruler.markers, IndentMarkers::body_text());
        assert!(!ruler.markers.is_first_line_indented());
        assert!(!ruler.markers.is_hanging());

        ruler.set_left_indent(36.0);
        assert_eq!(ruler.markers.left, 36.0);

        ruler.set_first_line(18.0);
        assert_eq!(ruler.markers.first_line, 18.0);
        assert!(ruler.markers.is_first_line_indented());

        // Hanging indent: first line outdented past the left marker.
        ruler.set_first_line(-36.0);
        assert!(ruler.markers.is_hanging());
        // Cannot hang further left than the margin.
        ruler.set_first_line(-72.0);
        assert_eq!(ruler.markers.first_line, -36.0);

        ruler.set_right_indent(48.0);
        assert_eq!(ruler.markers.right, 48.0);
        // Right indent cannot cross the text column edge.
        ruler.set_right_indent(9999.0);
        assert!(ruler.markers.right <= 468.0 - 36.0 - 0.0);

        ruler.reset();
        assert_eq!(ruler.markers, IndentMarkers::body_text());
    }
}
