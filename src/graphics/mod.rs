mod axis;
mod borders;
mod cell;
mod color;
mod layout;
mod length;
mod limits;
mod rect;
mod renderer;
mod size;
mod span;
mod style;

pub use axis::Axis;
pub use borders::Borders;
pub use cell::{Cell, CellMods};
pub use color::Color;
pub use layout::Layout;
pub use length::Len;
pub use limits::Limits;
pub use rect::Rect;
pub use renderer::Renderer;
pub use size::Size;
pub use span::Span;
pub use style::Style;

#[derive(Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

// #[derive(Default)]
// pub struct Ident {
//     up: usize,
//     down: usize,
//     left: usize,
//     right: usize,
// }

// impl From<usize> for Ident {
//     fn from(value: usize) -> Self {
//         Ident {
//             up: value,
//             down: value,
//             left: value,
//             right: value,
//         }
//     }
// }
