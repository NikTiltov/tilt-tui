#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Len {
    Var(usize),
    Max,
    Min,
}

impl Len {
    pub fn is_flex(&self) -> bool {
        matches!(self, Self::Max)
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Var(_) | Self::Min)
    }
}

impl From<usize> for Len {
    fn from(value: usize) -> Self {
        Len::Var(value)
    }
}
