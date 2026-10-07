//! Word menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "File", items: &["file.new", "file.open", "file.save"] },
    MenuCategory { title: "Format", items: &["text.bold", "text.italic", "text.underline"] },
    MenuCategory { title: "Insert", items: &["insert.page_break"] },
];
