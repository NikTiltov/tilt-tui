mod line;
mod span;
mod text;

pub use line::{line, Line};
pub use span::{span, Span};
pub use text::Text;

pub type Grapheme = (char, crate::graphics::Style);
