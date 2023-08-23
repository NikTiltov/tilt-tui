use std::ops::Mul;

use super::{Axis, Len};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size<T = usize> {
    pub w: T,
    pub h: T,
}

impl<T: Copy> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }

    pub fn main(&self, axis: Axis) -> T {
        if let Axis::H = axis {
            self.w
        } else {
            self.h
        }
    }

    pub fn cross(&self, axis: Axis) -> T {
        self.main(axis.rev())
    }

    pub fn main_mut(&mut self, axis: Axis) -> &mut T {
        if let Axis::H = axis {
            &mut self.w
        } else {
            &mut self.h
        }
    }

    pub fn cross_mut(&mut self, axis: Axis) -> &mut T {
        self.main_mut(axis.rev())
    }

    pub fn main_pack(&self, axis: Axis) -> (T, T) {
        (self.main(axis), self.cross(axis))
    }

    pub fn align(&self, axis: Axis) -> Self {
        Self {
            w: self.main(axis),
            h: self.cross(axis),
        }
    }
}

impl<T> From<Size<T>> for (T, T) {
    fn from(size: Size<T>) -> Self {
        (size.w, size.h)
    }
}

impl<T: Copy> From<(T, T)> for Size<T> {
    fn from(tuple: (T, T)) -> Self {
        let (w, h) = tuple;
        Size::new(w, h)
    }
}

impl<T: Mul<Output = T> + Copy> Size<T> {
    pub fn area(&self) -> T {
        self.w * self.h
    }
}

impl Size<Len> {
    pub fn min() -> Self {
        Self::new(Len::Min, Len::Min)
    }

    pub fn max() -> Self {
        Self::new(Len::Max, Len::Max)
    }
}
