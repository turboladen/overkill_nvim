pub mod cterm;
pub mod gui;

use nvim_oxi::api as oxi;

#[derive(Debug, Clone, Copy)]
pub struct Color<'a> {
    gui_color: gui::Color<'a>,
    cterm_color: cterm::Color<'a>,
}

impl<'a> Color<'a> {
    pub const fn new(gui_color: gui::Color<'a>, cterm_color: cterm::Color<'a>) -> Self {
        Self {
            gui_color,
            cterm_color,
        }
    }

    pub const fn new_none() -> Self {
        Self {
            gui_color: gui::Color::None,
            cterm_color: cterm::Color::None,
        }
    }

    pub fn gui_color(&self) -> gui::Color {
        self.gui_color
    }

    pub fn gui_string(&self) -> &str {
        self.gui_color.as_ref()
    }

    pub fn cterm_string(&self) -> &str {
        self.cterm_color.as_ref()
    }
}

pub struct TerminalAnsiColors<'a> {
    pub black: &'a Color<'a>,
    pub red: &'a Color<'a>,
    pub yellow: &'a Color<'a>,
    pub green: &'a Color<'a>,
    pub cyan: &'a Color<'a>,
    pub blue: &'a Color<'a>,
    pub purple: &'a Color<'a>,
    pub white: &'a Color<'a>,
}

impl TerminalAnsiColors<'_> {
    pub fn set_globals(&self) -> Result<(), oxi::Error> {
        oxi::set_var("terminal_color_0", self.black.gui_string())?;
        oxi::set_var("terminal_color_1", self.red.gui_string())?;
        oxi::set_var("terminal_color_2", self.green.gui_string())?;
        oxi::set_var("terminal_color_3", self.yellow.gui_string())?;
        oxi::set_var("terminal_color_4", self.blue.gui_string())?;
        oxi::set_var("terminal_color_5", self.purple.gui_string())?;
        oxi::set_var("terminal_color_5", self.cyan.gui_string())?;
        oxi::set_var("terminal_color_6", self.white.gui_string())?;
        oxi::set_var("terminal_color_7", self.black.gui_string())?;
        oxi::set_var("terminal_color_9", self.red.gui_string())?;
        oxi::set_var("terminal_color_10", self.green.gui_string())?;
        oxi::set_var("terminal_color_11", self.yellow.gui_string())?;
        oxi::set_var("terminal_color_12", self.blue.gui_string())?;
        oxi::set_var("terminal_color_13", self.purple.gui_string())?;
        oxi::set_var("terminal_color_14", self.cyan.gui_string())?;
        oxi::set_var("terminal_color_15", self.white.gui_string())?;

        Ok(())
    }
}

/// See `:help highlight-cterm`.
///
#[derive(Debug, Clone, Copy)]
pub enum TuiHighlightArg {
    Bold,
    Underline,
    Undercurl,
    UnderDouble,
    UnderDotted,
    UnderDashed,
    Strikethrough,
    Reverse,
    Inverse,
    Italic,
    Standout,
    NoCombine,
    None,
}

impl AsRef<str> for TuiHighlightArg {
    fn as_ref(&self) -> &str {
        match self {
            TuiHighlightArg::Bold => "bold",
            TuiHighlightArg::Underline => "underline",
            TuiHighlightArg::Undercurl => "undercurl",
            TuiHighlightArg::UnderDouble => "underdouble",
            TuiHighlightArg::UnderDotted => "underdotted",
            TuiHighlightArg::UnderDashed => "underdashed",
            TuiHighlightArg::Strikethrough => "strikethrough",
            TuiHighlightArg::Reverse => "reverse",
            TuiHighlightArg::Inverse => "inverse",
            TuiHighlightArg::Italic => "italic",
            TuiHighlightArg::Standout => "standout",
            TuiHighlightArg::NoCombine => "nocombine",
            TuiHighlightArg::None => "NONE",
        }
    }
}
