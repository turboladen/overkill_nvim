use crate::AsGroupName;

#[derive(Debug, Clone, Copy)]
pub enum Color<'a> {
    /// These are available on most systems.
    ///
    SuggestedColorName(SuggestedColorName),

    Hex(&'a str),

    /// Color name with an embedded space or other special characters (eg. "salmon pink").
    ///
    Name(&'a str),

    /// Use normal background color.
    ///
    Background,

    /// Use normal foreground color.
    ///
    Foreground,

    /// No color; transparent.
    ///
    None,
}

impl AsRef<str> for Color<'_> {
    fn as_ref(&self) -> &str {
        match self {
            Self::SuggestedColorName(scn) => scn.as_group_name(),
            Self::Hex(hex) => hex,
            Self::Name(name) => name,
            Self::Background => "bg",
            Self::Foreground => "fg",
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
    SeaGreen,

    Blue,
    LightBlue,
    DarkBlue,
    SlateBlue,

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

    Orange,
    Purple,
    Violet,
}

impl AsGroupName for SuggestedColorName {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::LightRed => "LightRed",
            Self::DarkRed => "DarkRed",
            Self::Green => "Green",
            Self::LightGreen => "LightGreen",
            Self::DarkGreen => "DarkGreen",
            Self::SeaGreen => "SeaGreen",
            Self::Blue => "Blue",
            Self::LightBlue => "LightBlue",
            Self::DarkBlue => "DarkBlue",
            Self::SlateBlue => "SlateBlue",
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
            Self::Orange => "Orange",
            Self::Purple => "Purple",
            Self::Violet => "Violet",
        }
    }
}
