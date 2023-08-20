use super::{KeyCode, KeyMods};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub mods: KeyMods,
}

impl KeyEvent {
    pub fn new(code: KeyCode, mods: KeyMods) -> Self {
        Self { code, mods }
    }

    pub fn code(code: KeyCode) -> Self {
        Self::new(code, KeyMods::NONE)
    }

    pub fn shift(code: KeyCode) -> Self {
        Self::new(code, KeyMods::SHIFT)
    }

    pub fn ctrl(code: KeyCode) -> Self {
        Self::new(code, KeyMods::CTRL)
    }

    pub fn alt(code: KeyCode) -> Self {
        Self::new(code, KeyMods::ALT)
    }
}

#[derive(Debug)]
pub struct ParceKeyEventError;

impl std::fmt::Display for ParceKeyEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string could not be parsed as KeyEvent".fmt(f)
    }
}

impl std::error::Error for ParceKeyEventError {}

impl std::str::FromStr for KeyEvent {
    type Err = ParceKeyEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut key_event = KeyEvent::new(KeyCode::UP, KeyMods::NONE);
        for key in s.split('-') {
            if let Ok(mods) = key.parse::<KeyMods>() {
                key_event.mods |= mods;
            } else if let Ok(code) = key.parse::<KeyCode>() {
                key_event.code = code;
            } else {
                return Err(ParceKeyEventError);
            }
        }
        Ok(key_event)
    }
}
