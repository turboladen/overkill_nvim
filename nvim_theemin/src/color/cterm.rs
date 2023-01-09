#[derive(Debug, Clone, Copy)]
pub enum Color<'a> {
    SuggestedColorName(SuggestedColorName),
    Fg,
    Bg,
    Number(&'a str),

    /// Removes the color.
    ///
    None,
}

impl AsRef<str> for Color<'_> {
    fn as_ref(&self) -> &str {
        match self {
            Self::SuggestedColorName(scn) => scn.as_ref(),
            Self::Fg => "fg",
            Self::Bg => "bg",
            Self::Number(num) => num,
            Self::None => "NONE",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SuggestedColorName {
    Red,
    LightRed,
    DarkRed,

    Green,
    LightGreen,
    DarkGreen,

    Blue,
    LightBlue,
    DarkBlue,

    Cyan,
    LightCyan,
    DarkCyan,

    Magenta,
    LightMagenta,
    DarkMagenta,

    Yellow,
    LightYellow,
    Brown,
    DarkYellow,

    Gray,
    LightGray,
    DarkGray,

    Black,
    White,
}

impl SuggestedColorName {
    pub fn nr_16(&self) -> usize {
        match self {
            Self::Black => 0,
            Self::DarkBlue => 1,
            Self::DarkGreen => 2,
            Self::DarkCyan => 3,
            Self::DarkRed => 4,
            Self::DarkMagenta => 5,
            Self::Brown | Self::DarkYellow => 6,
            Self::Gray | Self::LightGray => 7,
            Self::DarkGray => 8,
            Self::Blue | Self::LightBlue => 9,
            Self::Green | Self::LightGreen => 10,
            Self::Cyan | Self::LightCyan => 11,
            Self::Red | Self::LightRed => 12,
            Self::Magenta | Self::LightMagenta => 13,
            Self::Yellow | Self::LightYellow => 14,
            Self::White => 15,
        }
    }

    pub fn nr_8(&self) -> usize {
        match self {
            Self::Black | Self::DarkGray => 0,
            Self::Red | Self::LightRed | Self::DarkRed => 1,
            Self::Green | Self::LightGreen | Self::DarkGreen => 2,
            Self::Yellow | Self::LightYellow | Self::Brown | Self::DarkYellow => 3,
            Self::Blue | Self::LightBlue | Self::DarkBlue => 4,
            Self::Magenta | Self::LightMagenta | Self::DarkMagenta => 5,
            Self::Cyan | Self::LightCyan | Self::DarkCyan => 6,
            Self::White | Self::LightGray | Self::Gray => 7,
        }
    }
}

impl AsRef<str> for SuggestedColorName {
    fn as_ref(&self) -> &str {
        match self {
            Self::Red => "Red",
            Self::LightRed => "LightRed",
            Self::DarkRed => "DarkRed",
            Self::Green => "Green",
            Self::LightGreen => "LightGreen",
            Self::DarkGreen => "DarkGreen",
            Self::Blue => "Blue",
            Self::LightBlue => "LightBlue",
            Self::DarkBlue => "DarkBlue",
            Self::Cyan => "Cyan",
            Self::LightCyan => "LightCyan",
            Self::DarkCyan => "DarkCyan",
            Self::Magenta => "Magenta",
            Self::LightMagenta => "LightMagenta",
            Self::DarkMagenta => "DarkMagenta",
            Self::Yellow => "Yellow",
            Self::LightYellow => "LightYellow",
            Self::Brown => "Brown",
            Self::DarkYellow => "DarkYellow",
            Self::Gray => "Gray",
            Self::LightGray => "LightGray",
            Self::DarkGray => "DarkGray",
            Self::Black => "Black",
            Self::White => "White",
        }
    }
}
