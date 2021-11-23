pub mod buf;
pub mod global;

pub use self::global::Global;

use crate::{api::Error, key_code::KeyCode};
use neovim_sys::api::vim::{LuaString, Object};
use std::{borrow::Cow, convert::TryFrom, num::NonZeroI64};

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
    #[error("Unexpected option value: '{:?}'", _0)]
    UnexpectedOptionValue(Object),
}

#[derive(Debug, Clone, Copy)]
pub struct BreakIndent;

impl VimOption for BreakIndent {
    type Value = bool;

    const SHORT_NAME: &'static str = "bri";
    const LONG_NAME: &'static str = "breakindent";
}

#[derive(Debug, Clone, Copy)]
pub struct CmdHeight;

impl VimOption for CmdHeight {
    type Value = u8;

    const SHORT_NAME: &'static str = "ch";
    const LONG_NAME: &'static str = "cmdheight";
}

#[derive(Debug, Clone, Copy)]
pub struct ColorColumn;

impl VimOption for ColorColumn {
    type Value = ColorColumnValue;

    const SHORT_NAME: &'static str = "cc";
    const LONG_NAME: &'static str = "colorcolumn";
}

#[derive(Debug, Clone)]
pub struct ColorColumnValue(Vec<ColorColumnItem>);

impl ColorColumnValue {
    pub fn new(inner: Vec<ColorColumnItem>) -> Self {
        Self(inner)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<ColorColumnValue> for Object {
    fn from(value: ColorColumnValue) -> Self {
        let s = value
            .0
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(",");

        Self::from(LuaString::new(s).unwrap())
    }
}

impl TryFrom<Object> for ColorColumnValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            if let Some(positive) = item.strip_prefix('+') {
                inner.push(ColorColumnItem::Offset(
                    positive.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else if item.starts_with('-') {
                inner.push(ColorColumnItem::Offset(
                    item.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else {
                inner.push(ColorColumnItem::Absolute(item.parse::<u32>().map_err(
                    |_| VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone())),
                )?));
            }
        }

        Ok(Self(inner))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorColumnItem {
    Absolute(u32),
    Offset(NonZeroI64),
}

impl From<ColorColumnItem> for String {
    fn from(value: ColorColumnItem) -> Self {
        match value {
            ColorColumnItem::Absolute(column) => column.to_string(),
            ColorColumnItem::Offset(offset) => {
                let i = offset.get();

                if i.is_positive() {
                    format!("+{}", offset)
                } else {
                    i.to_string()
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConcealLevel;

impl VimOption for ConcealLevel {
    type Value = ConcealLevelValue;

    const SHORT_NAME: &'static str = "cole";
    const LONG_NAME: &'static str = "conceallevel";
}

#[derive(Debug, Clone, Copy)]
pub enum ConcealLevelValue {
    NormalText,
    OneCharReplacement,
    HiddenUnlessCustomReplacement,
    Hidden,
}

impl From<ConcealLevelValue> for Object {
    fn from(value: ConcealLevelValue) -> Self {
        match value {
            ConcealLevelValue::NormalText => Self::from(0),
            ConcealLevelValue::OneCharReplacement => Self::from(1),
            ConcealLevelValue::HiddenUnlessCustomReplacement => Self::from(2),
            ConcealLevelValue::Hidden => Self::from(3),
        }
    }
}

impl TryFrom<Object> for ConcealLevelValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_integer_unchecked() {
            0 => Ok(Self::NormalText),
            1 => Ok(Self::OneCharReplacement),
            2 => Ok(Self::HiddenUnlessCustomReplacement),
            3 => Ok(Self::Hidden),
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
        }
    }
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
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
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
pub struct ShowTabline;

impl VimOption for ShowTabline {
    type Value = ShowTablineValue;

    const SHORT_NAME: &'static str = "stal";
    const LONG_NAME: &'static str = "showtabline";
}

#[derive(Debug, Clone, Copy)]
pub enum ShowTablineValue {
    Never,
    OnlyIfTabPages,
    Always,
}

#[allow(clippy::fallible_impl_from)]
impl From<ShowTablineValue> for Object {
    fn from(value: ShowTablineValue) -> Self {
        match value {
            ShowTablineValue::Never => Self::from(0),
            ShowTablineValue::OnlyIfTabPages => Self::from(1),
            ShowTablineValue::Always => Self::from(2),
        }
    }
}

impl TryFrom<Object> for ShowTablineValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_integer_unchecked() {
            0 => Ok(Self::Never),
            1 => Ok(Self::OnlyIfTabPages),
            2 => Ok(Self::Always),
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SmartCase;

impl VimOption for SmartCase {
    type Value = bool;

    const SHORT_NAME: &'static str = "scs";
    const LONG_NAME: &'static str = "smartcase";
}

#[derive(Debug, Clone, Copy)]
pub struct Spell;

impl VimOption for Spell {
    type Value = bool;

    const SHORT_NAME: &'static str = "spell";
    const LONG_NAME: &'static str = "spell";
}

#[derive(Debug, Clone, Copy)]
pub struct SynMaxCol;

impl VimOption for SynMaxCol {
    type Value = u32;

    const SHORT_NAME: &'static str = "smc";
    const LONG_NAME: &'static str = "synmaxcol";
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
