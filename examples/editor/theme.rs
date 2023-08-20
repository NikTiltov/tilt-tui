use tilt_tui::graphics::Color;

pub mod pallete {
    use tilt_tui::prelude::Color;

    pub const FLAMINGO: Color = Color::hex(0xF2CDCD);
    pub const PINK: Color = Color::hex(0xF5C2E7);
    pub const MAUVE: Color = Color::hex(0xCBA6F7);
    pub const RED: Color = Color::hex(0xF38BA8);

    pub const N0: Color = Color::hex(0xCDD6F4);
    pub const N1: Color = Color::hex(0x585B70);
    pub const D0: Color = Color::hex(0x24273A);
}

pub const VAR: Color = pallete::RED;
pub const KEY: Color = pallete::MAUVE;
pub const OTH: Color = pallete::N1;

pub const FUN: Color = Color::hex(0x89B4FA);
pub const DEF: Color = Color::hex(0x94E2D5);
pub const NUM: Color = Color::hex(0xFAB387);
pub const STR: Color = Color::hex(0xA6E3A1);
