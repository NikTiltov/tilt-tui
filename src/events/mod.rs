mod key_code;
mod key_event;
mod key_mods;

pub use key_code::KeyCode;
pub use key_event::KeyEvent;
pub use key_mods::KeyMods;

use crate::graphics::Size;

pub enum Event {
    KeyEvent(KeyEvent),
    Resize(Size),
}
