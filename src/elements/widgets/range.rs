use crate::{prelude::*, Widget};

pub fn range(ratio: f64) -> Range {
    Range::new(ratio)
}

pub struct Range {
    ratio: f64,
    color: Color,
    axis: Axis,
}

impl Range {
    pub fn new(ratio: f64) -> Self {
        Self {
            ratio: ratio.clamp(0.0, 1.0),
            color: Color::WHITE,
            axis: Axis::H,
        }
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Range {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        Layout::new(limits.clamp(Size::new(0, 0)))
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        let size = rect.size();
        let main = (size.main(self.axis) as f64 * self.ratio) as usize;
        let cross = size.cross(self.axis);
        let (w, h) = Size::new(main, cross).align(self.axis).into();
        for x in 0..w {
            for y in 0..h {
                canvas.cell_mut(rect.x + x, rect.y + y).bg = self.color;
            }
        }
    }
}
