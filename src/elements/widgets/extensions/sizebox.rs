use crate::{prelude::*, Widget};

pub struct SizeBox {
    inner: Element,
    size: Size<Len>,
}

impl SizeBox {
    pub fn new(widget: impl Into<Element>) -> Self {
        Self {
            inner: widget.into(),
            size: Size::min(),
        }
    }

    pub fn width(mut self, size: impl Into<Len>) -> Self {
        self.size.w = size.into();
        self
    }

    pub fn height(mut self, size: impl Into<Len>) -> Self {
        self.size.h = size.into();
        self
    }
}

impl Widget for SizeBox {
    fn size(&self) -> Size<Len> {
        self.size
    }

    fn layout(&self, limits: Limits) -> Layout {
        let min_max = |axis| match self.size.main(axis) {
            Len::Var(v) => {
                let v = v.clamp(limits.min.main(axis), limits.max.main(axis));
                (v, v)
            }
            Len::Max => (limits.max.main(axis), limits.max.main(axis)),
            Len::Min => (limits.min.main(axis), limits.max.main(axis)),
        };
        let (min_w, max_w) = min_max(Axis::H);
        let (min_h, max_h) = min_max(Axis::V);
        let min = Size::new(min_w, min_h);
        let max = Size::new(max_w, max_h);
        self.inner.layout(Limits::new(min, max))
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        self.inner.render(layout, renderer);
    }
}

pub trait SizeExt: Into<Element> {
    fn width(self, width: impl Into<Len>) -> SizeBox {
        SizeBox::new(self).width(width)
    }

    fn height(self, width: impl Into<Len>) -> SizeBox {
        SizeBox::new(self).height(width)
    }
}

impl<T: Into<Element>> SizeExt for T {}
