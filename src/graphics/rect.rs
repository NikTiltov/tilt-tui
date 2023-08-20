use super::Size;

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }

    pub fn end_x(&self) -> usize {
        self.x + self.w
    }

    pub fn end_y(&self) -> usize {
        self.y + self.h
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Self::new(0, 0, size.w, size.h)
    }
}
