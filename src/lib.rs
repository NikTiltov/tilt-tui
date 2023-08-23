mod core;
mod elements;
mod terminal;

pub mod events;
pub mod graphics;

pub mod prelude {
    pub use crate::{
        core::{App, Element, UpdateResult},
        elements::{components::*, widgets::*},
        events::*,
        graphics::{Len::*, *},
    };
}

pub use crate::core::Widget;
