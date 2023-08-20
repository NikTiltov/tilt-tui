use crate::{prelude::*, Widget};

pub fn linear(axis: Axis) -> Linear {
    Linear::new(axis)
}

pub struct Linear {
    children: Vec<Element>,
    align_main: Alignment,
    align_cross: Alignment,
    size: Size<Length>,
    spacing: usize,
    axis: Axis,
}

impl Linear {
    pub fn new(axis: Axis) -> Self {
        Self {
            children: Vec::new(),
            align_main: Alignment::Start,
            align_cross: Alignment::Start,
            size: Size::min(),
            spacing: 0,
            axis,
        }
    }

    pub fn children(
        mut self,
        widgets: impl Iterator<Item = impl Into<Element>>,
    ) -> Self {
        for widget in widgets {
            self.children.push(widget.into());
        }
        self
    }

    pub fn child(mut self, widget: impl Into<Element>) -> Self {
        self.children.push(widget.into());
        self
    }

    pub fn size(
        mut self,
        width: impl Into<Length>,
        height: impl Into<Length>,
    ) -> Self {
        self.size = Size::new(width.into(), height.into());
        self
    }

    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn align_main(mut self, align: Alignment) -> Self {
        self.align_main = align;
        self
    }

    pub fn align_cross(mut self, align: Alignment) -> Self {
        self.align_cross = align;
        self
    }
}

impl Widget for Linear {
    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(&self, bound: Size) -> Layout {
        let mut nodes = Vec::with_capacity(self.children.len());
        nodes.resize_with(self.children.len(), Layout::default);

        let len = |axis: Axis| match self.size.main(axis) {
            Length::Min => None,
            Length::Max => Some(bound.main(axis)),
            Length::Var(var) => Some(var),
        };

        let size = {
            let width = len(Axis::H);
            let heigth = len(Axis::V);
            Size::new(width, heigth)
        };

        let bound = {
            let spacing =
                self.spacing * (self.children.len().saturating_sub(1));
            let main = size
                .main(self.axis)
                .unwrap_or(bound.main(self.axis))
                .saturating_sub(spacing);
            let cross = size.cross(self.axis).unwrap_or(bound.cross(self.axis));
            Size::new(main, cross).align(self.axis)
        };

        let mut spent_main = 0;
        let mut max_cross = 0;

        let (fixed, flex): (Vec<_>, _) = self
            .children
            .iter()
            .zip(nodes.iter_mut())
            .partition(|(c, _)| c.size().main(self.axis).is_fixed());

        for (child, node) in fixed {
            *node = child.layout(bound);
            let (main, cross) = node.size().main_pack(self.axis);
            max_cross = max_cross.max(cross);
            spent_main += main;
        }

        if !flex.is_empty() {
            let full_main = bound.main(self.axis).saturating_sub(spent_main);
            let mut last = full_main % flex.len();
            let bound = {
                let main = full_main / flex.len();
                let cross = bound.cross(self.axis);
                Size::new(main, cross).align(self.axis)
            };
            for (child, node) in flex {
                let mut bound = bound;
                if last > 0 {
                    *bound.main_mut(self.axis) += 1;
                    last -= 1;
                }
                *node = child.layout(bound);
                let (main, cross) = node.size().main_pack(self.axis);
                max_cross = max_cross.max(cross);
                spent_main += main;
            }
        }

        let last = size.main(self.axis).unwrap_or(spent_main);
        let cross = size.cross(self.axis).unwrap_or(max_cross);
        let main = match self.align_main {
            Alignment::Start => 0,
            Alignment::Center => (last - spent_main) / 2,
            Alignment::End => last - spent_main,
        };
        nodes.iter_mut().fold(0, |pos, node| {
            let cross = match self.align_cross {
                Alignment::Start => 0,
                Alignment::Center => (cross - node.size().cross(self.axis)) / 2,
                Alignment::End => cross - node.size().cross(self.axis),
            };
            let (x, y) = Size::new(main + pos, cross).align(self.axis).into();
            node.move_to(x, y);
            pos + node.size().main(self.axis) + self.spacing
        });

        let size = {
            let main = size.main(self.axis).unwrap_or(spent_main);
            let cross = size.cross(self.axis).unwrap_or(max_cross);
            Size::new(main, cross).align(self.axis)
        };
        Layout::new(size).with_children(nodes)
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        // let rect = layout.rect();
        // let cell = Cell::symbol('%');
        // for x in rect.x..rect.end_x() {
        //     canvas.draw(x, rect.y, cell);
        //     canvas.draw(x, rect.end_y() - 1, cell);
        // }
        // for y in rect.y..rect.end_y() {
        //     canvas.draw(rect.x, y, cell);
        //     canvas.draw(rect.end_x() - 1, y, cell);
        // }
        for (child, layout) in self.children.iter().zip(layout.children()) {
            child.render(layout, canvas);
        }
    }
}
