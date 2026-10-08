//! Martensite widget suite for WordCraft.

pub mod dock_panel;
pub mod indent_ruler;
pub mod options_bar;
pub mod outline_tree;
pub mod page_view;
pub mod scrubby_input;
pub mod tool_strip;

pub use dock_panel::DockPanelGroup;
pub use indent_ruler::IndentRulerWidget;
pub use options_bar::OptionsBarWidget;
pub use outline_tree::{HeadingItemDef, OutlineTreeWidget};
pub use page_view::PageViewWidget;
pub use scrubby_input::ScrubbyInputWidget;
pub use tool_strip::ToolStripWidget;
