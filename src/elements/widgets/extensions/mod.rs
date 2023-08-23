mod borderbox;
mod infobox;
mod sizebox;

pub use borderbox::BorderExt;
pub use infobox::InfoExt;
pub use sizebox::SizeExt;

use crate::core::Element;

pub trait IfExt: Into<Element> {
    fn apply_if(self, condition: bool, f: impl Fn(Self) -> Self) -> Self {
        if condition {
            f(self)
        } else {
            self
        }
    }
}

impl<T: Into<Element>> IfExt for T {}
