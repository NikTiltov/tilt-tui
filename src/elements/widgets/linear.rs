use crate::{prelude::*, Widget};

pub fn linear(axis: Axis) -> Linear {
    Linear::new(axis)
}

pub struct Linear {
    children: Vec<Element>,
    align_main: Align,
    align_cross: Align,
    spacing: usize,
    axis: Axis,
}

impl Linear {
    pub fn new(axis: Axis) -> Self {
        Self {
            children: Vec::new(),
            align_main: Align::Start,
            align_cross: Align::Start,
            spacing: 0,
            axis,
        }
    }

    pub fn child(mut self, widget: impl Into<Element>) -> Self {
        self.children.push(widget.into());
        self
    }

    pub fn children(
        mut self,
        widgets: impl IntoIterator<Item = impl Into<Element>>,
    ) -> Self {
        for widget in widgets {
            self.children.push(widget.into());
        }
        self
    }

    pub fn align_main(mut self, align: Align) -> Self {
        self.align_main = align;
        self
    }

    pub fn align_cross(mut self, align: Align) -> Self {
        self.align_cross = align;
        self
    }

    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for Linear {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        let mut nodes = Vec::with_capacity(self.children.len());
        nodes.resize_with(self.children.len(), Layout::default);

        let size = {
            let main = limits.max.main(self.axis).saturating_sub(
                self.spacing * self.children.len().saturating_sub(1),
            );
            let cross = limits.max.cross(self.axis);
            Size::new(main, cross).align(self.axis)
        };

        let mut max_main = 0;
        let mut max_cross = 0;

        let (fixed, flex): (Vec<_>, _) = self
            .children
            .iter()
            .zip(nodes.iter_mut())
            .partition(|(c, _)| c.size().main(self.axis).is_fixed());

        let fixed_limits = Limits::from_max(size);
        for (child, node) in fixed {
            *node = child.layout(fixed_limits);
            let (main, cross) = node.size().main_pack(self.axis);
            max_cross = max_cross.max(cross);
            max_main += main;
        }

        if !flex.is_empty() {
            let main = size.main(self.axis).saturating_sub(max_main);
            let mut remainder = main % flex.len();
            let flex_limits = {
                let main = main / flex.len();
                let cross = limits.max.cross(self.axis);
                let max = Size::new(main, cross).align(self.axis);
                Limits::from_max(max)
            };
            for (child, node) in flex {
                let mut flex_limits = flex_limits;
                if remainder > 0 {
                    *flex_limits.max.main_mut(self.axis) += 1;
                    remainder -= 1;
                }
                *node = child.layout(flex_limits);
                let (main, cross) = node.size().main_pack(self.axis);
                max_cross = max_cross.max(cross);
                max_main += main;
            }
        }

        let main = limits.max.main(self.axis);
        let main = match self.align_main {
            Align::Start => 0,
            Align::Center => (main - max_main) / 2,
            Align::End => main - max_main,
        };
        let cross = max_cross;

        let mut pos = 0;
        for node in nodes.iter_mut() {
            let cross = match self.align_cross {
                Align::Start => 0,
                Align::Center => (cross - node.size().cross(self.axis)) / 2,
                Align::End => cross - node.size().cross(self.axis),
            };
            let (x, y) = Size::new(main + pos, cross).align(self.axis).into();
            node.move_to(x, y);
            pos += node.size().main(self.axis) + self.spacing;
        }
        pos -= self.spacing;

        let size = Size::new(pos, max_cross).align(self.axis);
        Layout::new(limits.clamp(size)).with_children(nodes)
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        for (child, layout) in self.children.iter().zip(layout.children()) {
            child.render(layout, renderer);
        }
    }
}
