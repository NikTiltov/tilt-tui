use crate::prelude::*;

pub fn split<'a>(
    axis: Axis,
    first: impl Into<Element>,
    second: impl Into<Element>,
) -> Element {
    let fill = match axis {
        Axis::H => '\u{02502}',
        Axis::V => '\u{02500}',
    };
    let (w, h) = match axis {
        Axis::H => (1, usize::MAX),
        Axis::V => (usize::MAX, 1),
    };

    linear(axis)
        .child(first.width(Max).height(Max))
        .child(panel(fill).width(w).height(h))
        .child(second.width(Max).height(Max))
        .into()
}
