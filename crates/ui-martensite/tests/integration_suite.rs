//! Comprehensive integration test suite for WordCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, indent ruler, outline tree and page coordinates.

use wordcraft_doc::Document;
use wordcraft_engine::{Engine, Tool};
use wordcraft_ui_martensite::{
    WordcraftApp,
    command_reg::find_command,
    document::Alignment,
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{DockPanelGroup, HeadingItemDef, IndentRulerWidget, OptionsBarWidget, OutlineTreeWidget},
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new(Document::new());
    let mut app = WordcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(!app.focus_mode_active);

    // 2. Keystroke Workflow: switch to Pen, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("p", app.active_tool);
    assert_eq!(new_tool, Some(Tool::Pen));
    app.set_tool(Tool::Pen);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores Pen
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::Pen));
    app.set_tool(Tool::Pen);

    // 3. Options Bar Interaction for Active Pen
    let mut options = OptionsBarWidget::new();
    options.set_tool(Tool::Pen);
    options.ink_width.on_pointer_down(0.0);
    options.ink_width.on_pointer_move(4.0, false, false);
    options.ink_width.on_pointer_up();
    assert_eq!(options.ink_width.value, 6.0); // 2.0 + 4

    // 4. Outline Tree & Hierarchy Updates
    let mut outline = OutlineTreeWidget::new();
    outline.headings.push(HeadingItemDef {
        id: 1,
        text: "Introduction".to_string(),
        level: 1,
        collapsed: false,
        children: vec![HeadingItemDef { id: 2, text: "Background".to_string(), level: 2, collapsed: false, children: vec![] }],
    });
    outline.headings.push(HeadingItemDef { id: 3, text: "Results".to_string(), level: 1, collapsed: false, children: vec![] });
    outline.select_heading(3);
    assert_eq!(outline.selected_heading_id, Some(3));
    outline.toggle_collapsed(1);
    assert!(outline.headings[0].collapsed);

    // 5. Indent Ruler Marker Dragging
    let mut ruler = IndentRulerWidget::new(468.0);
    ruler.set_left_indent(36.0);
    ruler.set_first_line(18.0);
    ruler.set_right_indent(24.0);
    assert!(ruler.markers.is_first_line_indented());
    assert_eq!(ruler.markers.right, 24.0);

    // 6. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Headings", "Pages", "Results"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 7. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 8. Text Formatting via Document State
    app.doc.toggle_bold();
    assert!(app.doc.style.bold);
    app.doc.style.alignment = Alignment::Center;
    assert_eq!(app.doc.style.alignment, Alignment::Center);

    // 9. Theme Color Space Consistency
    let theme = CraftTheme::paper_light();
    let obsidian = CraftTheme::studio_obsidian();
    assert_ne!(theme.surface_app_bg, obsidian.surface_app_bg);
    assert_ne!(theme.surface_page, obsidian.surface_page);
}
