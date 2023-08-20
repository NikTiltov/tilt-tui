#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Var(usize),
    Min,
    Max,
}

impl Length {
    pub fn is_flex(&self) -> bool {
        matches!(self, Length::Max)
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, Length::Min | Length::Var(_))
    }

    pub fn var(self) -> Option<usize> {
        if let Length::Var(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl From<usize> for Length {
    fn from(value: usize) -> Self {
        Length::Var(value)
    }
}
