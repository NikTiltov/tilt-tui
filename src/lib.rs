mod core;
mod elements;
mod terminal;

mod events;
mod graphics;

pub mod prelude {
    pub use crate::{
        core::{Element, UpdateResult},
        elements::{components::*, widgets::*},
        events::*,
        graphics::{Len::*, *},
    };
}

pub use crate::core::{App, Widget};
