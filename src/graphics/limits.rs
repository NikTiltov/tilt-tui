use super::Size;

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub min: Size,
    pub max: Size,
}

impl Limits {
    pub fn new(min: Size, max: Size) -> Self {
        Self { min, max }
    }

    pub fn from_max(max: Size) -> Self {
        Self::new(Size::new(0, 0), max)
    }

    pub fn clamp(&self, size: Size) -> Size {
        Size::new(
            size.w.clamp(self.min.w, self.max.w),
            size.h.clamp(self.min.h, self.max.h),
        )
    }
}
