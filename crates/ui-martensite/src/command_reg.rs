//! Decoupled command catalog and taxonomy for WordCraft.
//!
//! Command ids match `wordcraft-engine`'s registry so menu items dispatch
//! straight to the shared command surface. Shortcuts use the portable `Mod`
//! form (Ctrl on Windows/Linux, ⌘ on macOS).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Insert,
    Format,
    Layout,
    References,
    Review,
    View,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "New", category: CommandCategory::File, default_shortcut: Some("Mod+N"), secondary_shortcut: None },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Mod+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Mod+S"), secondary_shortcut: None },
    CommandSpec { id: "file.saveAs", label: "Save As…", category: CommandCategory::File, default_shortcut: Some("F12"), secondary_shortcut: None },
    CommandSpec {
        id: "file.exportPdf", label: "Export as PDF…", category: CommandCategory::File, default_shortcut: None, secondary_shortcut: None
    },
    CommandSpec { id: "file.print", label: "Print…", category: CommandCategory::File, default_shortcut: Some("Mod+P"), secondary_shortcut: None },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Mod+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Mod+Y"), secondary_shortcut: None },
    CommandSpec { id: "edit.cut", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Mod+X"), secondary_shortcut: None },
    CommandSpec { id: "edit.copy", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Mod+C"), secondary_shortcut: None },
    CommandSpec { id: "edit.paste", label: "Paste", category: CommandCategory::Edit, default_shortcut: Some("Mod+V"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.formatPainter",
        label: "Format Painter",
        category: CommandCategory::Edit,
        default_shortcut: Some("Mod+Shift+C"),
        secondary_shortcut: Some("Mod+Shift+V"),
    },
    CommandSpec { id: "select.all", label: "Select All", category: CommandCategory::Edit, default_shortcut: Some("Mod+A"), secondary_shortcut: None },
    CommandSpec { id: "edit.find", label: "Find", category: CommandCategory::Edit, default_shortcut: Some("Mod+F"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.replace", label: "Replace…", category: CommandCategory::Edit, default_shortcut: Some("Mod+H"), secondary_shortcut: None
    },
    // Insert
    CommandSpec {
        id: "insert.pageBreak",
        label: "Page Break",
        category: CommandCategory::Insert,
        default_shortcut: Some("Mod+Enter"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "insert.table", label: "Table…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.picture", label: "Pictures…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.link", label: "Link…", category: CommandCategory::Insert, default_shortcut: Some("Mod+K"), secondary_shortcut: None },
    CommandSpec { id: "insert.header", label: "Header", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.footer", label: "Footer", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "insert.pageNumber",
        label: "Page Number",
        category: CommandCategory::Insert,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "insert.symbol", label: "Symbol…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    // Format (Home ribbon)
    CommandSpec { id: "format.bold", label: "Bold", category: CommandCategory::Format, default_shortcut: Some("Mod+B"), secondary_shortcut: None },
    CommandSpec {
        id: "format.italic",
        label: "Italic",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+I"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "format.underline",
        label: "Underline",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+U"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "format.strikethrough",
        label: "Strikethrough",
        category: CommandCategory::Format,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "format.highlight",
        label: "Text Highlight Color",
        category: CommandCategory::Format,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "format.color", label: "Font Color", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "para.alignLeft",
        label: "Align Left",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+L"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "para.alignCenter",
        label: "Align Center",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+E"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "para.alignRight",
        label: "Align Right",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+R"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "para.justify",
        label: "Justify",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+J"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "para.bullets",
        label: "Bullets",
        category: CommandCategory::Format,
        default_shortcut: Some("Mod+Shift+L"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "para.numbering", label: "Numbering", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "para.indent", label: "Increase Indent", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "para.outdent", label: "Decrease Indent", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    // Layout
    CommandSpec { id: "layout.margins", label: "Margins", category: CommandCategory::Layout, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "layout.orientation",
        label: "Orientation",
        category: CommandCategory::Layout,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "layout.size", label: "Page Size", category: CommandCategory::Layout, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "layout.columns", label: "Columns", category: CommandCategory::Layout, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "layout.break", label: "Breaks", category: CommandCategory::Layout, default_shortcut: None, secondary_shortcut: None },
    // References
    CommandSpec {
        id: "references.toc",
        label: "Table of Contents",
        category: CommandCategory::References,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "references.footnote",
        label: "Insert Footnote",
        category: CommandCategory::References,
        default_shortcut: Some("Mod+Alt+F"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "references.citation",
        label: "Insert Citation",
        category: CommandCategory::References,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "references.bibliography",
        label: "Bibliography",
        category: CommandCategory::References,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "references.caption",
        label: "Insert Caption",
        category: CommandCategory::References,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // Review
    CommandSpec {
        id: "review.spelling",
        label: "Spelling & Grammar",
        category: CommandCategory::Review,
        default_shortcut: Some("F7"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "review.newComment",
        label: "New Comment",
        category: CommandCategory::Review,
        default_shortcut: Some("Mod+Alt+M"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "review.trackChanges",
        label: "Track Changes",
        category: CommandCategory::Review,
        default_shortcut: Some("Mod+Shift+E"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "review.wordCount", label: "Word Count", category: CommandCategory::Review, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "review.comments",
        label: "Show Comments",
        category: CommandCategory::Review,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // View
    CommandSpec { id: "view.printLayout", label: "Print Layout", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.readMode", label: "Read Mode", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.webLayout", label: "Web Layout", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.ruler", label: "Ruler", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "view.navigationPane",
        label: "Navigation Pane",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.zoom100", label: "100%", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "view.onePage", label: "One Page", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "view.multiplePages",
        label: "Multiple Pages",
        category: CommandCategory::View,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.pageWidth", label: "Page Width", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Insert).is_empty());
        assert!(!commands_by_category(CommandCategory::Format).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}
