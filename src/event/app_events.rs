use crate::{app::modes::EditorMode, ui::components::FocusableComponent};

pub use crate::app::buffer::BufferEvent;
pub use crate::app::cursor::CursorEvent;
pub use crate::app::file::FileEvent;

#[derive(Clone, Debug)]
pub enum AppEvent {
    Buffer(BufferEvent),
    Cursor(CursorEvent),
    File(FileEvent),
    ChangeFocus(FocusableComponent),
    ChangeToMode(EditorMode),
    Quit,
}
