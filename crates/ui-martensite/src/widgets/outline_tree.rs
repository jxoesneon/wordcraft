//! Heading outline tree widget (navigation pane).

/// A heading entry in the document outline. `level` is the Word heading
/// level (1–9).
#[derive(Clone, Debug, PartialEq)]
pub struct HeadingItemDef {
    pub id: u64,
    pub text: String,
    pub level: u8,
    pub collapsed: bool,
    pub children: Vec<HeadingItemDef>,
}

pub struct OutlineTreeWidget {
    pub headings: Vec<HeadingItemDef>,
    pub selected_heading_id: Option<u64>,
    pub show_page_thumbnails: bool,
    pub filter_query: String,
}

impl OutlineTreeWidget {
    pub fn new() -> Self {
        Self { headings: Vec::new(), selected_heading_id: None, show_page_thumbnails: false, filter_query: String::new() }
    }

    pub fn select_heading(&mut self, id: u64) {
        self.selected_heading_id = Some(id);
    }

    pub fn toggle_collapsed(&mut self, id: u64) {
        if let Some(item) = find_heading_mut(&mut self.headings, id) {
            item.collapsed = !item.collapsed;
        }
    }
}

impl Default for OutlineTreeWidget {
    fn default() -> Self {
        Self::new()
    }
}

fn find_heading_mut(items: &mut [HeadingItemDef], id: u64) -> Option<&mut HeadingItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_heading_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outline_tree_mutation() {
        let mut tree = OutlineTreeWidget::new();
        tree.headings.push(HeadingItemDef {
            id: 1,
            text: "Introduction".to_string(),
            level: 1,
            collapsed: false,
            children: vec![HeadingItemDef { id: 2, text: "Background".to_string(), level: 2, collapsed: false, children: vec![] }],
        });
        tree.headings.push(HeadingItemDef { id: 3, text: "Results".to_string(), level: 1, collapsed: false, children: vec![] });

        tree.select_heading(3);
        assert_eq!(tree.selected_heading_id, Some(3));

        // Collapse a nested heading by id
        tree.toggle_collapsed(2);
        let nested = &tree.headings[0].children[0];
        assert!(nested.collapsed);
        tree.toggle_collapsed(2);
        assert!(!tree.headings[0].children[0].collapsed);
    }
}
