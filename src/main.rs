#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod terminal;
mod row;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run();
}
