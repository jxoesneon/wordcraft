//! Word command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "text.bold", label: "Bold", shortcut: Some("Cmd+B") },
    Command { id: "text.italic", label: "Italic", shortcut: Some("Cmd+I") },
    Command { id: "text.underline", label: "Underline", shortcut: Some("Cmd+U") },
    Command { id: "insert.page_break", label: "Page Break", shortcut: Some("Cmd+Enter") },
];
