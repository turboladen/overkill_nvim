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
impl_vim_option!(ColorColumn, Flags<ColorColumnValue>, "cc", "colorcolumn");
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
impl_vim_option!(ShiftWidth, u8, "sw", "shiftwidth");
impl_vim_option!(SmartCase, bool, "scs", "smartcase");
impl_vim_option!(SmartIndent, bool, "si", "smartindent");
impl_vim_option!(SoftTabStop, u8, "sts", "softtabstop");
impl_vim_option!(Spell, bool, "spell", "spell");
impl_vim_option!(SpellLang, Flags<SpellLangValue>, "spl", "spelllang");
impl_vim_option!(SplitBelow, bool, "sb", "splitbelow");
impl_vim_option!(SplitRight, bool, "spr", "splitright");
impl_vim_option!(SwapFile, bool, "swf", "swapfile");
impl_vim_option!(SynMaxCol, u32, "smc", "synmaxcol");
impl_vim_option!(TabStop, u8, "ts", "tabstop");
impl_vim_option!(TermGuiColors, bool, "tgc", "termguicolors");
impl_vim_option!(UndoFile, bool, "udf", "undofile");
impl_vim_option!(WildMenu, bool, "smnu", "wildmenu");
impl_vim_option!(WriteBackup, bool, "wb", "writebackup");

//-------------------------------------------------------------------------------------------------
// Custom types for options
//-------------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Flags<T>(Vec<T>)
where
    String: From<T>;

impl<T> Flags<T>
where
    String: From<T>,
{
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }
}

impl<T> From<Flags<T>> for Object
where
    String: From<T>,
{
    fn from(value: Flags<T>) -> Self {
        let s = value
            .0
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(",");

        Self::from(LuaString::new_unchecked(s))
    }
}

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

impl TryFrom<Object> for Flags<ColorColumnValue> {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            if let Some(positive) = item.strip_prefix('+') {
                inner.push(ColorColumnValue::Offset(
                    positive.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else if item.starts_with('-') {
                inner.push(ColorColumnValue::Offset(
                    item.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else {
                inner.push(ColorColumnValue::Absolute(item.parse::<u32>().map_err(
                    |_| VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone())),
                )?));
            }
        }

        Ok(Self(inner))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorColumnValue {
    Absolute(u32),
    Offset(NonZeroI64),
}

impl From<ColorColumnValue> for String {
    fn from(value: ColorColumnValue) -> Self {
        match value {
            ColorColumnValue::Absolute(column) => column.to_string(),
            ColorColumnValue::Offset(offset) => {
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

impl TryFrom<Object> for Flags<SpellLangValue> {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            inner.push(SpellLangValue::from(item));
        }

        Ok(Self(inner))
    }
}

/// Pulled from http://ftp.vim.org/vim/runtime/spell/.
///
#[derive(Debug, Clone)]
pub enum SpellLangValue {
    Af,
    Am,
    Bg,
    Br,
    Ca,
    Cjk,
    Cs,
    Cy,
    Da,
    De,
    De19,
    De20,
    DeAt,
    DeCh,
    DeDe,
    El,
    En,
    EnAu,
    EnCa,
    EnGb,
    EnNz,
    EnUs,
    Eo,
    Es,
    EsEs,
    EsMx,
    Fo,
    Fr,
    Ga,
    Gd,
    Gl,
    He,
    Hr,
    Hu,
    Id,
    It,
    Ku,
    La,
    Lt,
    Lv,
    Mg,
    Mi,
    Ms,
    Nb,
    Nl,
    Nn,
    Ny,
    Pl,
    Pt,
    PtBr,
    PtPt,
    Ro,
    Ru,
    RuRu,
    RuYo,
    Rw,
    Sk,
    Sl,
    Sr,
    Sv,
    Sw,
    Tet,
    Th,
    Tl,
    Tn,
    Uk,
    Yi,
    Zu,
    Custom(String),
}

impl<'a> From<&'a str> for SpellLangValue {
    fn from(s: &str) -> Self {
        match s {
            "af" => Self::Af,
            "am" => Self::Am,
            "bg" => Self::Bg,
            "br" => Self::Br,
            "ca" => Self::Ca,
            "cs" => Self::Cs,
            "cy" => Self::Cy,
            "da" => Self::Da,
            "de" => Self::De,
            "de_19" => Self::De19,
            "de_20" => Self::De20,
            "de_at" => Self::DeAt,
            "de_ch" => Self::DeCh,
            "de_de" => Self::DeDe,
            "el" => Self::El,
            "en" => Self::En,
            "en_au" => Self::EnAu,
            "en_ca" => Self::EnCa,
            "en_gb" => Self::EnGb,
            "en_nz" => Self::EnNz,
            "en_us" => Self::EnUs,
            "eo" => Self::Eo,
            "es" => Self::Es,
            "es_es" => Self::EsEs,
            "es_mx" => Self::EsMx,
            "fo" => Self::Fo,
            "fr" => Self::Fr,
            "ga" => Self::Ga,
            "gd" => Self::Gd,
            "gl" => Self::Gl,
            "he" => Self::He,
            "hr" => Self::Hr,
            "hu" => Self::Hu,
            "id" => Self::Id,
            "it" => Self::It,
            "ku" => Self::Ku,
            "la" => Self::La,
            "lt" => Self::Lt,
            "lv" => Self::Lv,
            "mg" => Self::Mg,
            "mi" => Self::Mi,
            "ms" => Self::Ms,
            "nb" => Self::Nb,
            "nl" => Self::Nl,
            "nn" => Self::Nn,
            "ny" => Self::Ny,
            "pl" => Self::Pl,
            "pt" => Self::Pt,
            "pt_br" => Self::PtBr,
            "pt_pt" => Self::PtPt,
            "ro" => Self::Ro,
            "ru" => Self::Ru,
            "ru_ru" => Self::RuRu,
            "ru_yo" => Self::RuYo,
            "rw" => Self::Rw,
            "sk" => Self::Sk,
            "sl" => Self::Sl,
            "sr" => Self::Sr,
            "sv" => Self::Sv,
            "sw" => Self::Sw,
            "tet" => Self::Tet,
            "th" => Self::Th,
            "tl" => Self::Tl,
            "tn" => Self::Tn,
            "uk" => Self::Uk,
            "yi" => Self::Yi,
            "zu" => Self::Zu,
            locale => Self::Custom(locale.to_string()),
        }
    }
}

impl From<SpellLangValue> for String {
    fn from(spelllang_value: SpellLangValue) -> Self {
        let s = match spelllang_value {
            SpellLangValue::Af => "af",
            SpellLangValue::Am => "am",
            SpellLangValue::Bg => "bg",
            SpellLangValue::Br => "br",
            SpellLangValue::Ca => "ca",
            SpellLangValue::Cjk => "cjk",
            SpellLangValue::Cs => "cs",
            SpellLangValue::Cy => "cy",
            SpellLangValue::Da => "da",
            SpellLangValue::De => "de",
            SpellLangValue::De19 => "de_19",
            SpellLangValue::De20 => "de_20",
            SpellLangValue::DeAt => "de_at",
            SpellLangValue::DeCh => "de_ch",
            SpellLangValue::DeDe => "de_de",
            SpellLangValue::El => "el",
            SpellLangValue::En => "en",
            SpellLangValue::EnAu => "en_au",
            SpellLangValue::EnCa => "en_ca",
            SpellLangValue::EnGb => "en_gb",
            SpellLangValue::EnNz => "en_nz",
            SpellLangValue::EnUs => "en_us",
            SpellLangValue::Eo => "eo",
            SpellLangValue::Es => "es",
            SpellLangValue::EsEs => "es_es",
            SpellLangValue::EsMx => "es_mx",
            SpellLangValue::Fo => "fo",
            SpellLangValue::Fr => "fr",
            SpellLangValue::Ga => "ga",
            SpellLangValue::Gd => "gd",
            SpellLangValue::Gl => "gl",
            SpellLangValue::He => "he",
            SpellLangValue::Hr => "hr",
            SpellLangValue::Hu => "hu",
            SpellLangValue::Id => "id",
            SpellLangValue::It => "it",
            SpellLangValue::Ku => "ku",
            SpellLangValue::La => "la",
            SpellLangValue::Lt => "lt",
            SpellLangValue::Lv => "lv",
            SpellLangValue::Mg => "mg",
            SpellLangValue::Mi => "mi",
            SpellLangValue::Ms => "ms",
            SpellLangValue::Nb => "nb",
            SpellLangValue::Nl => "nl",
            SpellLangValue::Nn => "nn",
            SpellLangValue::Ny => "ny",
            SpellLangValue::Pl => "pl",
            SpellLangValue::Pt => "pt",
            SpellLangValue::PtBr => "pt_br",
            SpellLangValue::PtPt => "pt_pt",
            SpellLangValue::Ro => "ro",
            SpellLangValue::Ru => "ru",
            SpellLangValue::RuRu => "ru_ru",
            SpellLangValue::RuYo => "ru_yo",
            SpellLangValue::Rw => "rw",
            SpellLangValue::Sk => "sk",
            SpellLangValue::Sl => "sl",
            SpellLangValue::Sr => "sr",
            SpellLangValue::Sv => "sv",
            SpellLangValue::Sw => "sw",
            SpellLangValue::Tet => "tet",
            SpellLangValue::Th => "th",
            SpellLangValue::Tl => "tl",
            SpellLangValue::Tn => "tn",
            SpellLangValue::Uk => "uk",
            SpellLangValue::Yi => "yi",
            SpellLangValue::Zu => "zu",
            SpellLangValue::Custom(locale) => return locale,
        };

        Self::from(s)
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
