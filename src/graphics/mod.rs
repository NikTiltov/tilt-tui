mod axis;
mod borders;
mod canvas;
mod cell;
mod color;
mod layout;
mod length;
mod rect;
mod size;
mod style;

pub use axis::Axis;
pub use borders::Borders;
pub use canvas::Canvas;
pub use cell::Cell;
pub use color::Color;
pub use layout::Layout;
pub use length::Length;
pub use rect::Rect;
pub use size::Size;
pub use style::Style;

#[derive(Default)]
pub enum Alignment {
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
