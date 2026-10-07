//! Word processing integration tests.

use wordcraft_ui_martensite::{WordcraftApp, document::Alignment};

#[test]
fn test_word_formatting_workflow() {
    let mut app = WordcraftApp::new();
    assert_eq!(app.doc.style.font_size, 11.0);

    app.doc.toggle_bold();
    assert!(app.doc.style.bold);

    app.doc.style.alignment = Alignment::Center;
    assert_eq!(app.doc.style.alignment, Alignment::Center);
}
