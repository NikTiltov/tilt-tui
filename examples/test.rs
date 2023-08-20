use tilt_tui::prelude::*;

fn main() {
    println!("rgb   {:?}", Color::rgb(109, 141, 191));
    println!("hex?  {:?}", Color::rgb(0x6D, 0x8D, 0xBF));
    println!("hex   {:?}", Color::hex(0x6D8DBF));
}
