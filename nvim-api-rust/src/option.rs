pub mod buf;
pub mod global;

pub use self::global::Global;

use crate::{api::Error, key_code::KeyCode};
use neovim_sys::api::vim::{LuaString, Object};
use std::{borrow::Cow, convert::TryFrom};

/// The trait that all options implement, allowing to define each option's long name (ex.
/// `autoindent`) and short name (ex. `ai`), as well as what type of value they expect. While vim's
/// docs say that options can be a a) `boolean`, b) `integer`, or c) `string`, 1) that's not quite
/// accurate and 2) we can do a bit better with `string`-related values by using Rust types.
///
pub trait VimOption
where
    Object: From<Self::Value>,
    Error: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Value: TryFrom<Object>;

    const SHORT_NAME: &'static str;
    const LONG_NAME: &'static str;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum VimOptionError {
    #[error("Unexpected option value: '{}'", _0)]
    UnexpectedOptionValue(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Aleph;

impl VimOption for Aleph {
    type Value = i64;

    const SHORT_NAME: &'static str = "al";
    const LONG_NAME: &'static str = "aleph";
}

#[derive(Debug, Clone, Copy)]
pub struct AllowRevIns;

impl VimOption for AllowRevIns {
    type Value = bool;

    const SHORT_NAME: &'static str = "ari";
    const LONG_NAME: &'static str = "allowrevins";
}

#[derive(Debug, Clone, Copy)]
pub struct AmbiWidth;

impl VimOption for AmbiWidth {
    type Value = AmbiWidthOption;

    const SHORT_NAME: &'static str = "ambw";
    const LONG_NAME: &'static str = "ambiwidth";
}

#[derive(Debug, Clone, Copy)]
pub enum AmbiWidthOption {
    Single,
    Double,
}

#[allow(clippy::fallible_impl_from)]
impl From<AmbiWidthOption> for Object {
    fn from(value: AmbiWidthOption) -> Self {
        match value {
            AmbiWidthOption::Single => Self::from(LuaString::new("single").unwrap()),
            AmbiWidthOption::Double => (Self::from(LuaString::new("double").unwrap())),
        }
    }
}

impl TryFrom<Object> for AmbiWidthOption {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_string_unchecked().to_string_lossy() {
            Cow::Borrowed("single") => Ok(Self::Single),
            Cow::Borrowed("double") => Ok(Self::Double),
            s => Err(VimOptionError::UnexpectedOptionValue(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BreakIndent;

impl VimOption for BreakIndent {
    type Value = bool;

    const SHORT_NAME: &'static str = "bri";
    const LONG_NAME: &'static str = "breakindent";
}

#[derive(Debug, Clone, Copy)]
pub struct List;

impl VimOption for List {
    type Value = bool;

    const SHORT_NAME: &'static str = "list";
    const LONG_NAME: &'static str = "list";
}

#[derive(Debug, Clone, Copy)]
pub struct ListChars;

impl VimOption for ListChars {
    type Value = ListCharsSettings;

    const SHORT_NAME: &'static str = "lcs";
    const LONG_NAME: &'static str = "listchars";
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCharsSettings {
    eol: Option<char>,
    tab: Option<(char, char, Option<char>)>,
    space: Option<char>,
    lead: Option<char>,
    trail: Option<char>,
    extends: Option<char>,
    precedes: Option<char>,
    conceal: Option<char>,
    nbsp: Option<char>,
}

macro_rules! list_char_settings_def_single_char_fn {
    ($fn_name:ident) => {
        pub const fn $fn_name(self, setting: char) -> Self {
            let mut s = self;
            s.$fn_name = Some(setting);
            s
        }
    };
}

impl ListCharsSettings {
    list_char_settings_def_single_char_fn!(eol);
    list_char_settings_def_single_char_fn!(space);
    list_char_settings_def_single_char_fn!(lead);
    list_char_settings_def_single_char_fn!(trail);
    list_char_settings_def_single_char_fn!(extends);
    list_char_settings_def_single_char_fn!(precedes);
    list_char_settings_def_single_char_fn!(conceal);
    list_char_settings_def_single_char_fn!(nbsp);

    pub const fn tab(self, setting: (char, char, Option<char>)) -> Self {
        let mut s = self;
        s.tab = Some(setting);
        s
    }

    pub const fn tab2(self, char1: char, char2: char) -> Self {
        let mut s = self;
        s.tab = Some((char1, char2, None));
        s
    }

    pub const fn tab3(self, char1: char, char2: char, char3: char) -> Self {
        let mut s = self;
        s.tab = Some((char1, char2, Some(char3)));
        s
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<ListCharsSettings> for Object {
    fn from(value: ListCharsSettings) -> Self {
        let mut settings_string = String::new();

        if let Some(eol) = value.eol {
            settings_string += &format!("eol:{}", eol);
        }

        if let Some(tab) = value.tab {
            if let Some(tab_2) = tab.2 {
                settings_string += &format!("tab:{}{}{}", tab.0, tab.1, tab_2);
            } else {
                settings_string += &format!("tab:{}{}", tab.0, tab.1);
            }
        }

        if let Some(space) = value.space {
            settings_string += &format!("space:{}", space);
        }

        if let Some(lead) = value.lead {
            settings_string += &format!("lead:{}", lead);
        }

        if let Some(trail) = value.trail {
            settings_string += &format!("trail:{}", trail);
        }

        if let Some(extends) = value.extends {
            settings_string += &format!("extends:{}", extends);
        }

        if let Some(precedes) = value.precedes {
            settings_string += &format!("precedes:{}", precedes);
        }

        if let Some(conceal) = value.conceal {
            settings_string += &format!("conceal:{}", conceal);
        }

        if let Some(nbsp) = value.nbsp {
            settings_string += &format!("nbsp:{}", nbsp);
        }

        Self::from(LuaString::new(settings_string).unwrap())
    }
}

impl TryFrom<Object> for ListCharsSettings {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let mut settings = Self::default();

        for setting in value.as_string_unchecked().to_string_lossy().split(',') {
            let mut key_value = setting.split(':');
            let key = key_value.next().unwrap();
            let mut value_chars = key_value.next().unwrap().chars();

            match key {
                "tab" => {
                    settings.tab = Some((
                        value_chars.next().unwrap(),
                        value_chars.next().unwrap(),
                        value_chars.next(),
                    ));
                }
                "eol" => {
                    settings.eol = Some(value_chars.next().unwrap());
                }
                "space" => {
                    settings.space = Some(value_chars.next().unwrap());
                }
                "lead" => {
                    settings.lead = Some(value_chars.next().unwrap());
                }
                "trail" => {
                    settings.trail = Some(value_chars.next().unwrap());
                }
                "extends" => {
                    settings.extends = Some(value_chars.next().unwrap());
                }
                "precedes" => {
                    settings.precedes = Some(value_chars.next().unwrap());
                }
                "conceal" => {
                    settings.conceal = Some(value_chars.next().unwrap());
                }
                "nbsp" => {
                    settings.nbsp = Some(value_chars.next().unwrap());
                }
                _ => (),
            }
        }

        Ok(settings)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number;

impl VimOption for Number {
    type Value = bool;

    const SHORT_NAME: &'static str = "nu";
    const LONG_NAME: &'static str = "number";
}

#[derive(Debug, Clone, Copy)]
pub struct PasteToggle;

impl VimOption for PasteToggle {
    type Value = KeyCode;

    const SHORT_NAME: &'static str = "pt";
    const LONG_NAME: &'static str = "pastetoggle";
}

#[derive(Debug, Clone, Copy)]
pub struct IncCommand;

impl VimOption for IncCommand {
    type Value = IncCommandValue;

    const SHORT_NAME: &'static str = "icm";
    const LONG_NAME: &'static str = "inccommand";
}

#[derive(Debug, Clone, Copy)]
pub enum IncCommandValue {
    NoSplit,
    Split,
}

#[allow(clippy::fallible_impl_from)]
impl From<IncCommandValue> for Object {
    fn from(value: IncCommandValue) -> Self {
        match value {
            IncCommandValue::NoSplit => Self::from(LuaString::new("nosplit").unwrap()),
            IncCommandValue::Split => (Self::from(LuaString::new("split").unwrap())),
        }
    }
}

impl TryFrom<Object> for IncCommandValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_string_unchecked().to_string_lossy() {
            Cow::Borrowed("nosplit") => Ok(Self::NoSplit),
            Cow::Borrowed("split") => Ok(Self::Split),
            s => Err(VimOptionError::UnexpectedOptionValue(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineBreak;

impl VimOption for LineBreak {
    type Value = bool;

    const SHORT_NAME: &'static str = "lbr";
    const LONG_NAME: &'static str = "linebreak";
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollOff;

impl VimOption for ScrollOff {
    type Value = u16;

    const SHORT_NAME: &'static str = "scs";
    const LONG_NAME: &'static str = "smartcase";
}

#[derive(Debug, Clone, Copy)]
pub struct SmartCase;

impl VimOption for SmartCase {
    type Value = bool;

    const SHORT_NAME: &'static str = "scs";
    const LONG_NAME: &'static str = "smartcase";
}

// #[derive(Debug, Clone, Copy)]
// pub enum Background {
//     Light,
//     Dark,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Backspace {
//     Indent,
//     Eol,
//     Start,
//     NoStop,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BackupCopy {
//     Yes,
//     No,
//     Auto,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BackupCopyBreakLink {
//     BreakSymLink,
//     BreakHardLink,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BellOff {
//     All,
//     Backspace,
//     Cursor,
//     Complete,
//     Copy,
//     CtrlG,
//     Error,
//     Esc,
//     Ex,
//     Hangul,
//     InsertMode,
//     Lang,
//     Mess,
//     ShowMatch,
//     Operator,
//     Register,
//     Shell,
//     Spell,
//     WildMode,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BreakIndentOptItem {
//     Min(u16),
//     Shift(u16),
//     Sbr,
//     List(u16),
//     ListMinusOne,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BrowseDir {
//     Last,
//     Buffer,
//     Current,
// }
