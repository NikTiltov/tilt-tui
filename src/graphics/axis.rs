#[derive(Clone, Copy)]
pub enum Axis {
    H,
    V,
}

impl Axis {
    pub fn rev(&self) -> Axis {
        match self {
            Axis::H => Axis::V,
            Axis::V => Axis::H,
        }
    }
}
