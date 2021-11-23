pub mod buf;
pub mod global;
pub mod global_local;

pub use self::{global::Global, global_local::GlobalLocal};

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

macro_rules! impl_vim_option {
    ($option:ident, $value:ty, $short_name:expr, $long_name:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $option;

        impl VimOption for $option {
            type Value = $value;

            const SHORT_NAME: &'static str = $short_name;
            const LONG_NAME: &'static str = $long_name;
        }
    };
}

impl_vim_option!(AutoIndent, bool, "ai", "autoindent");
impl_vim_option!(BreakIndent, bool, "bri", "breakindent");
impl_vim_option!(CmdHeight, u8, "ch", "cmdheight");
impl_vim_option!(Clipboard, ClipboardSettings, "cb", "clipboard");
impl_vim_option!(ColorColumn, ColorColumnValue, "cc", "colorcolumn");
impl_vim_option!(ConcealLevel, ConcealLevelValue, "cole", "conceallevel");
impl_vim_option!(CursorLine, bool, "cul", "cursorline");
impl_vim_option!(ExpandTab, bool, "et", "expandtab");
impl_vim_option!(FoldEnable, bool, "fen", "foldenable");
impl_vim_option!(Hidden, bool, "hid", "hidden");
impl_vim_option!(IncCommand, IncCommandValue, "icm", "inccommand");
impl_vim_option!(LineBreak, bool, "lbr", "linebreak");
impl_vim_option!(List, bool, "list", "list");
impl_vim_option!(ListChars, ListCharsSettings, "lcs", "listchars");
impl_vim_option!(Number, bool, "nu", "number");
impl_vim_option!(PasteToggle, KeyCode, "pt", "pastetoggle");
impl_vim_option!(ScrollOff, u16, "so", "scrolloff");
impl_vim_option!(ShowTabline, ShowTablineValue, "stal", "showtabline");
impl_vim_option!(SmartCase, bool, "scs", "smartcase");
impl_vim_option!(SmartIndent, bool, "si", "smartindent");
impl_vim_option!(Spell, bool, "spell", "spell");
impl_vim_option!(SplitBelow, bool, "sb", "splitbelow");
impl_vim_option!(SplitRight, bool, "spr", "splitright");
impl_vim_option!(SwapFile, bool, "swf", "swapfile");
impl_vim_option!(SynMaxCol, u32, "smc", "synmaxcol");
impl_vim_option!(TermGuiColors, bool, "tgc", "termguicolors");
impl_vim_option!(UndoFile, bool, "udf", "undofile");
impl_vim_option!(WildMenu, bool, "smnu", "wildmenu");
impl_vim_option!(WriteBackup, bool, "wb", "writebackup");

//-------------------------------------------------------------------------------------------------
// Custom types for options
//-------------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default)]
pub struct ClipboardSettings {
    unnamed: bool,
    unnamed_plus: bool,
}

impl ClipboardSettings {
    pub const fn unnamed(self) -> Self {
        let mut s = self;
        s.unnamed = true;
        s
    }

    pub const fn unnamed_plus(self) -> Self {
        let mut s = self;
        s.unnamed_plus = true;
        s
    }
}

impl From<ClipboardSettings> for Object {
    fn from(value: ClipboardSettings) -> Self {
        match (value.unnamed, value.unnamed_plus) {
            (true, true) => Self::from(LuaString::new_unchecked("unnamed,unnamedplus")),
            (true, _) => Self::from(LuaString::new_unchecked("unnamed")),
            (_, true) => Self::from(LuaString::new_unchecked("unnamedplus")),
            (_, _) => Self::new_nil(),
        }
    }
}

impl TryFrom<Object> for ClipboardSettings {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut settings = Self::default();

        for item in split {
            match item {
                "unnamed" => settings.unnamed = true,
                "unnamedplus" => settings.unnamed_plus = true,
                _ => (),
            }
        }

        Ok(settings)
    }
}

#[derive(Debug, Clone)]
pub struct ColorColumnValue(Vec<ColorColumnItem>);

impl ColorColumnValue {
    pub fn new(inner: Vec<ColorColumnItem>) -> Self {
        Self(inner)
    }
}

impl From<ColorColumnValue> for Object {
    fn from(value: ColorColumnValue) -> Self {
        let s = value
            .0
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(",");

        Self::from(LuaString::new_unchecked(s))
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

        Self::from(LuaString::new_unchecked(settings_string))
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
pub enum IncCommandValue {
    NoSplit,
    Split,
}

impl From<IncCommandValue> for Object {
    fn from(value: IncCommandValue) -> Self {
        match value {
            IncCommandValue::NoSplit => Self::from(LuaString::new_unchecked("nosplit")),
            IncCommandValue::Split => (Self::from(LuaString::new_unchecked("split"))),
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
pub enum ShowTablineValue {
    Never,
    OnlyIfTabPages,
    Always,
}

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
