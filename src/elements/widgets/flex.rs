use crate::{prelude::*, Widget};

pub fn flex<'a>(axis: Axis) -> Flex<'a> {
    Flex::new(axis)
}

pub struct Flex<'a> {
    children: Vec<Element<'a>>,
    align_main: Alignment,
    align_cross: Alignment,
    spacing: usize,
    axis: Axis,
}

impl<'a> Flex<'a> {
    pub fn new(axis: Axis) -> Self {
        Self {
            children: Vec::new(),
            align_main: Alignment::Start,
            align_cross: Alignment::Start,
            spacing: 0,
            axis,
        }
    }

    pub fn add_child(&mut self, child: impl Into<Element<'a>>) {}
}

impl<'a> Widget for Flex<'a> {
    fn size(&self) -> Size<Length> {
        todo!()
    }

    fn layout(&self, bound: Size) -> Layout {
        todo!()
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        todo!()
    }
}
