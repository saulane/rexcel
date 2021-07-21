mod terminal;
mod editor;
mod cell;
mod row;
mod document;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use cell::Cell;
pub use row::Row;
pub use document::Document;

fn main() {
    let mut editor = Editor::new().unwrap();
    editor.run();
}
