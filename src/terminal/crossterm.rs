use crate::{
    events::{Event, KeyCode, KeyEvent, KeyMods},
    graphics::{Color, Size},
};
use crossterm::{event as ct, style::Color as ctColor};

impl TryFrom<ct::Event> for Event {
    type Error = ();

    fn try_from(event: ct::Event) -> Result<Self, Self::Error> {
        match event {
            ct::Event::Key(key_event) => Ok(Event::KeyEvent(key_event.try_into()?)),
            ct::Event::Resize(w, h) => Ok(Event::Resize(Size::new(w as usize, h as usize))),
            _ => Err(()),
        }
    }
}

impl TryFrom<ct::KeyEvent> for KeyEvent {
    type Error = ();

    fn try_from(event: ct::KeyEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            code: event.code.try_into()?,
            mods: event.modifiers.try_into()?,
        })
    }
}

impl TryFrom<ct::KeyCode> for KeyCode {
    type Error = ();

    fn try_from(code: ct::KeyCode) -> Result<Self, Self::Error> {
        use ct::KeyCode::*;
        match code {
            Char(char) => Ok(KeyCode::CHAR(char)),
            Backspace => Ok(KeyCode::BACKSPACE),
            Enter => Ok(KeyCode::ENTER),
            Right => Ok(KeyCode::RIGHT),
            Left => Ok(KeyCode::LEFT),
            Down => Ok(KeyCode::DOWN),
            Up => Ok(KeyCode::UP),
            PageDown => Ok(KeyCode::PD),
            PageUp => Ok(KeyCode::PD),
            Home => Ok(KeyCode::HOME),
            End => Ok(KeyCode::END),
            Tab => Ok(KeyCode::TAB),
            BackTab => Ok(KeyCode::BACKTAB),
            Delete => Ok(KeyCode::DELETE),
            Insert => Ok(KeyCode::INSERT),
            F(n) => Ok(KeyCode::F(n)),
            Esc => Ok(KeyCode::ESC),
            _ => Err(()),
        }
    }
}

impl TryFrom<ct::KeyModifiers> for KeyMods {
    type Error = ();

    fn try_from(mods: ct::KeyModifiers) -> Result<Self, Self::Error> {
        Ok(KeyMods::from_bits_truncate(mods.bits()))
    }
}

impl From<Color> for ctColor {
    fn from(color: Color) -> Self {
        ctColor::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}
