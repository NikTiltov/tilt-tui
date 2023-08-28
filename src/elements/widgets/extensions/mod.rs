mod borderbox;
mod infobox;
mod sizebox;
mod stylebox;

pub use borderbox::BorderExt;
pub use infobox::InfoExt;
pub use sizebox::SizeExt;
pub use stylebox::StyleExt;

use crate::core::Element;

pub trait IfExt: Into<Element> {
    fn apply_if<T, F>(self, condition: bool, f: F) -> Element
    where
        T: Into<Element>,
        F: Fn(Self) -> T,
    {
        if condition {
            f(self).into()
        } else {
            self.into()
        }
    }
}

impl<T: Into<Element>> IfExt for T {}
