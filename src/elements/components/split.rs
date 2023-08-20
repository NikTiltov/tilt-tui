use crate::prelude::*;

pub fn vsplit<'a>(
    first: impl Into<Element>,
    second: impl Into<Element>,
) -> Element {
    split(Axis::V, first, second)
}

pub fn hsplit<'a>(
    first: impl Into<Element>,
    second: impl Into<Element>,
) -> Element {
    split(Axis::H, first, second)
}

pub fn split<'a>(
    axis: Axis,
    first: impl Into<Element>,
    second: impl Into<Element>,
) -> Element {
    let (w, h) = Size::new(Var(1), Max).align(axis).into();
    let fill = match axis {
        Axis::H => '\u{02502}',
        Axis::V => '\u{02500}',
    };
    linear(axis)
        .size(Max, Max)
        .child(first.into())
        .child(panel(fill).size(w, h))
        .child(second.into())
        .into()
}
