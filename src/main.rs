#![warn(clippy::pedantic)]

mod cell;
mod document;
mod editor;
mod row;
mod terminal;

pub use cell::Cell;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::new().unwrap();
    editor.run();
}
