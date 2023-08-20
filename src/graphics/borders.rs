pub struct Borders {
    pub top: char,
    pub left: char,
    pub right: char,
    pub bottom: char,
    pub upper_left: char,
    pub upper_right: char,
    pub lower_left: char,
    pub lower_right: char,
}

impl Borders {
    pub const EMPTY: Self = Self {
        top: ' ',
        left: ' ',
        right: ' ',
        bottom: ' ',
        upper_left: ' ',
        upper_right: ' ',
        lower_left: ' ',
        lower_right: ' ',
    };

    pub const NORMAL: Self = Self {
        top: '\u{02500}',
        left: '\u{02502}',
        right: '\u{02502}',
        bottom: '\u{02500}',
        upper_left: '\u{0250c}',
        upper_right: '\u{02510}',
        lower_left: '\u{02514}',
        lower_right: '\u{02518}',
    };

    pub const BOLD: Self = Self {
        top: '\u{02501}',
        left: '\u{02503}',
        right: '\u{02503}',
        bottom: '\u{02501}',
        upper_left: '\u{0250f}',
        upper_right: '\u{02513}',
        lower_left: '\u{02517}',
        lower_right: '\u{0251b}',
    };
}
