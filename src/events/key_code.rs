#[derive(Debug, Hash, PartialEq, Eq)]
pub enum KeyCode {
    CHAR(char),
    F(u8),
    BACKSPACE,
    BACKTAB,
    INSERT,
    DELETE,
    ENTER,
    HOME,
    END,
    TAB,
    ESC,
    RIGHT,
    LEFT,
    DOWN,
    UP,
    PD,
    PU,
}

#[derive(Debug)]
pub struct ParceKeyCodeError;

impl std::fmt::Display for ParceKeyCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string could not be parsed as KeyCode".fmt(f)
    }
}

impl std::error::Error for ParceKeyCodeError {}

impl std::str::FromStr for KeyCode {
    type Err = ParceKeyCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backspace" => Ok(Self::BACKSPACE),
            "backtab" => Ok(Self::BACKTAB),
            "insert" => Ok(Self::INSERT),
            "delete" => Ok(Self::DELETE),
            "enter" => Ok(Self::ENTER),
            "home" => Ok(Self::HOME),
            "end" => Ok(Self::END),
            "tab" => Ok(Self::TAB),
            "esc" => Ok(Self::ESC),
            "right" => Ok(Self::RIGHT),
            "left" => Ok(Self::LEFT),
            "down" => Ok(Self::DOWN),
            "up" => Ok(Self::UP),
            "pd" => Ok(Self::PD),
            "pu" => Ok(Self::PU),
            _ => {
                if let Ok(num) = s.parse::<u8>() {
                    if num <= 24 {
                        return Ok(Self::F(num));
                    }
                }
                if s.chars().count() == 1 {
                    let char = s.chars().next().unwrap();
                    return Ok(Self::CHAR(char));
                }
                Err(ParceKeyCodeError)
            }
        }
    }
}
