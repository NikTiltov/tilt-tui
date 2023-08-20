bitflags::bitflags! {
    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct KeyMods: u8 {
        const NONE = 0b0000_0000;
        const SHIFT = 0b0000_0001;
        const CTRL = 0b0000_0010;
        const ALT = 0b0000_0100;
    }
}

#[derive(Debug)]
pub struct ParceKeyModsError;

impl std::fmt::Display for ParceKeyModsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string could not be parsed as KeyMods".fmt(f)
    }
}

impl std::error::Error for ParceKeyModsError {}

impl std::str::FromStr for KeyMods {
    type Err = ParceKeyModsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::NONE),
            "shift" => Ok(Self::SHIFT),
            "ctrl" => Ok(Self::CTRL),
            "alt" => Ok(Self::ALT),
            _ => Err(ParceKeyModsError),
        }
    }
}
