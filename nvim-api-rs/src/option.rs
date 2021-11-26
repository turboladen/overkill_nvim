pub mod flags;

macro_rules! def_settings_builder_method {
    ($meth:ident) => {
        pub const fn $meth(self) -> Self {
            let mut s = self;
            s.$meth = true;
            s
        }
    };
}

mod clipboard_settings;
mod color_column_value;
mod complete_opt_settings;
mod conceal_level_value;
mod inc_command_value;
mod list_char_settings;
mod short_mess_item;
mod show_tabline_value;
mod spell_lang_value;

pub use self::{
    clipboard_settings::ClipboardSettings,
    color_column_value::ColorColumnValue,
    complete_opt_settings::CompleteOptSettings,
    conceal_level_value::ConcealLevelValue,
    flags::{CharFlags, StringFlags},
    inc_command_value::IncCommandValue,
    list_char_settings::ListCharsSettings,
    short_mess_item::ShortMessItem,
    show_tabline_value::ShowTablineValue,
    spell_lang_value::SpellLangValue,
};

use crate::key_code::KeyCode;
use neovim_sys::api::vim::Object;
use std::convert::TryFrom;

/// The trait that all options implement, allowing to define each option's long name (ex.
/// `autoindent`) and short name (ex. `ai`), as well as what type of value they expect. While vim's
/// docs say that options can be a a) `boolean`, b) `integer`, or c) `string`, 1) that's not quite
/// accurate and 2) we can do a bit better with `string`-related values by using Rust types.
///
pub trait VimOption
where
    Object: TryFrom<Self::Value>,
    // VimOptionError: From<<Self::Value as TryFrom<Object>>::Error>,
{
    type Value: TryFrom<Object>;

    const SHORT_NAME: &'static str;
    const LONG_NAME: &'static str;

    /// Calls `nvim_get_global_local_option, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get() -> Result<Self::Value, VimOptionError> {
        crate::api::vim::nvim_get_global_local_option(Self::SHORT_NAME)
            .map_err(|e| VimOptionError::from(e))
            .and_then(|object| Self::Value::try_from(object).map_err(VimOptionError::from))
    }

    /// Calls `nvim_get_global_local_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Self::Value) -> Result<(), VimOptionError> {
        crate::api::vim::nvim_set_global_local_option(Self::SHORT_NAME, Object::try_from(value)?)
    }

    /// Calls `nvim_get_global_option()`, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_global() -> Result<Self::Value, VimOptionError> {
        crate::api::vim::nvim_get_global_option(Self::SHORT_NAME)
            .and_then(|object| Self::Value::try_from(object).map_err(VimOptionError::from))
    }

    /// Calls `nvim_set_global_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global(value: Self::Value) -> Result<(), VimOptionError> {
        crate::api::vim::nvim_set_global_option(Self::SHORT_NAME, Object::from(value))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum VimOptionError {
    #[error("Unexpected option value: '{:?}'", _0)]
    UnexpectedOptionValue(Object),

    #[error(transparent)]
    KeyCode(#[from] crate::key_code::InvalidKeyCode),

    // #[error(transparent)]
    // ApiError(#[from] crate::api::Error),
    #[error(transparent)]
    ObjectError(#[from] neovim_sys::api::vim::object::Error),
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
impl_vim_option!(
    ColorColumn,
    StringFlags<ColorColumnValue>,
    "cc",
    "colorcolumn"
);
impl_vim_option!(CompleteOpt, CompleteOptSettings, "cot", "completeopt");
impl_vim_option!(ConcealLevel, ConcealLevelValue, "cole", "conceallevel");
impl_vim_option!(CursorLine, bool, "cul", "cursorline");
impl_vim_option!(ExpandTab, bool, "et", "expandtab");
impl_vim_option!(FoldEnable, bool, "fen", "foldenable");
impl_vim_option!(GrepPrg, String, "gp", "grepprg");
impl_vim_option!(Hidden, bool, "hid", "hidden");
impl_vim_option!(History, u32, "hi", "history");
impl_vim_option!(IncCommand, IncCommandValue, "icm", "inccommand");
impl_vim_option!(LineBreak, bool, "lbr", "linebreak");
impl_vim_option!(List, bool, "list", "list");
impl_vim_option!(ListChars, ListCharsSettings, "lcs", "listchars");
impl_vim_option!(Number, bool, "nu", "number");
impl_vim_option!(PasteToggle, KeyCode, "pt", "pastetoggle");
impl_vim_option!(ScrollOff, u16, "so", "scrolloff");
impl_vim_option!(ShortMess, CharFlags<ShortMessItem>, "shm", "shortmess");
impl_vim_option!(ShowTabline, ShowTablineValue, "stal", "showtabline");
impl_vim_option!(ShiftWidth, u8, "sw", "shiftwidth");
impl_vim_option!(SmartCase, bool, "scs", "smartcase");
impl_vim_option!(SmartIndent, bool, "si", "smartindent");
impl_vim_option!(SoftTabStop, u8, "sts", "softtabstop");
impl_vim_option!(Spell, bool, "spell", "spell");
impl_vim_option!(SpellLang, StringFlags<SpellLangValue>, "spl", "spelllang");
impl_vim_option!(SplitBelow, bool, "sb", "splitbelow");
impl_vim_option!(SplitRight, bool, "spr", "splitright");
impl_vim_option!(SwapFile, bool, "swf", "swapfile");
impl_vim_option!(SynMaxCol, u32, "smc", "synmaxcol");
impl_vim_option!(TabStop, u8, "ts", "tabstop");
impl_vim_option!(TermGuiColors, bool, "tgc", "termguicolors");
impl_vim_option!(UndoFile, bool, "udf", "undofile");
impl_vim_option!(UpdateTime, u32, "ut", "updatetime");
impl_vim_option!(WildMenu, bool, "smnu", "wildmenu");
impl_vim_option!(WriteBackup, bool, "wb", "writebackup");

impl flags::AddAssignFlags for ShortMess {
    type Item = ShortMessItem;

    fn add_assign(rhs: Self::Item) -> Result<(), VimOptionError> {
        let mut current = Self::get()?;
        current.push(rhs);
        Self::set(current)
    }

    fn add_assign_global(rhs: Self::Item) -> Result<(), VimOptionError> {
        let mut current = Self::get_global()?;
        current.push(rhs);
        Self::set_global(current)
    }
}

impl flags::SubAssignFlags for ShortMess {
    type Item = ShortMessItem;

    fn sub_assign(rhs: &Self::Item) -> Result<(), VimOptionError> {
        let mut current = Self::get()?;
        current.remove(rhs);
        Self::set(current)
    }

    fn sub_assign_global(rhs: &Self::Item) -> Result<(), VimOptionError> {
        let mut current = Self::get_global()?;
        current.remove(rhs);
        Self::set_global(current)
    }
}
