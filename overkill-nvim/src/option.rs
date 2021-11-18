#![allow(clippy::module_name_repetitions)]

//! Types and functions for getting and setting options.
//!
pub mod flags;

macro_rules! def_settings_builder_method {
    ($meth:ident) => {
        #[doc=concat!("Sets ", stringify!($meth), " to true.")]
        #[must_use]
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
mod sign_column_value;
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
    sign_column_value::SignColumnValue,
    spell_lang_value::SpellLangValue,
};

use crate::key_code::KeyCode;
use nvim_api::api::{nvim, Integer, NvimString, Object};
use std::convert::{Infallible, TryFrom};

/// The trait that all options implement, allowing to define each option's long name (ex.
/// `autoindent`) and short name (ex. `ai`), as well as what type of value they expect. While vim's
/// docs say that options can be a a) `boolean`, b) `integer`, or c) `string`, 1) that's not quite
/// accurate and 2) we can do a bit better with `string`-related values by using Rust types.
///
pub trait NvimOption {
    /// The short name/abbreviation of the option (ex. `"cc"` for colorcolumn).
    ///
    const SHORT_NAME: &'static str;

    /// The long name of the option (ex. `"colorcolumn"` for colorcolumn).
    ///
    const LONG_NAME: &'static str;

    /// Calls `nvim_get_option()`, and handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_object() -> Result<Object, NvimOptionError> {
        Ok(nvim_api::api::nvim::nvim_get_option(Self::SHORT_NAME)?)
    }

    /// Calls `nvim_set_option()`, and handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_object<T>(value: T) -> Result<(), NvimOptionError>
    where
        Object: From<T>,
    {
        Ok(nvim::nvim_set_option(Self::SHORT_NAME, value)?)
    }

    /// Calls `nvim_get_global_option()`, and handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_global_object() -> Result<Object, NvimOptionError> {
        Ok(nvim::nvim_get_global_option(Self::SHORT_NAME)?)
    }

    /// Calls `nvim_set_global_option()`, and handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global_object<T>(value: T) -> Result<(), NvimOptionError>
    where
        Object: From<T>,
    {
        Ok(nvim::nvim_set_global_option(Self::SHORT_NAME, value)?)
    }
}

/// Trait for getting and setting options that have boolean values.
///
pub trait BooleanOption: NvimOption {
    /// Analogous to `:set option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    /// # Panics
    ///
    /// If the returned value isn't a `bool`.
    ///
    fn get() -> Result<bool, NvimOptionError> {
        let object = Self::get_object()?;

        Ok(bool::try_from(object).unwrap())
    }

    /// Analogous to `:set option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: bool) -> Result<(), NvimOptionError> {
        Self::set_object(value)
    }

    /// Analogous to `:setglobal option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    /// # Panics
    ///
    /// If the returned value isn't a `bool`.
    ///
    ///
    fn get_global() -> Result<bool, NvimOptionError> {
        let object = Self::get_global_object()?;

        Ok(bool::try_from(object).unwrap())
    }

    /// Analogous to `:setglobal option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global(value: bool) -> Result<(), NvimOptionError> {
        Self::set_global_object(value)
    }
}

/// Trait for getting and setting options that have number values.
///
pub trait NumberOption: NvimOption
where
    Integer: From<Self::Value>,
    NvimOptionError: From<<<Self as NumberOption>::Value as TryFrom<Integer>>::Error>,
{
    /// The type of value the implementation works with. This allows for defining a type that
    /// represents the set of supported numbers the given option supports. For example,
    /// `'conceallevel'` only supports `0`, `1`, `2`, or `3`, where each of those values has some
    /// meaning. Instead of allowing, say, all `u8` values for this option, we can define an `enum`
    /// with variants for each one of those values, where each of those can be covnerted back into
    /// a number. This ensures--at compile time--the caller never passes in a value that nvim won't
    /// accept.
    ///
    type Value: TryFrom<Integer>;

    /// Analogous to `:set option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call. If the returned value can't be converted to a `Integer`.
    ///
    fn get() -> Result<Self::Value, NvimOptionError> {
        let object = Self::get_object()?;
        let i = Integer::try_from(object)?;

        Self::Value::try_from(i).map_err(NvimOptionError::from)
    }

    /// Analogous to `:set option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Self::Value) -> Result<(), NvimOptionError> {
        Ok(Self::set_object(Integer::from(value))?)
    }

    /// Analogous to `:setglobal option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call. If the returned value can't be converted to a `Integer`.
    ///
    fn get_global() -> Result<Self::Value, NvimOptionError> {
        let object = Self::get_global_object()?;
        let i = Integer::try_from(object)?;

        Self::Value::try_from(i).map_err(NvimOptionError::from)
    }

    /// Analogous to `:setglobal option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global(value: Self::Value) -> Result<(), NvimOptionError> {
        Ok(Self::set_global_object(Integer::from(value))?)
    }
}

/// Trait for getting and setting options that have string values.
///
pub trait StringOption: NvimOption
where
    NvimString: From<Self::Value>,
    NvimOptionError: From<<<Self as StringOption>::Value as TryFrom<NvimString>>::Error>,
{
    /// The type of value the implementation works with. This allows for defining a type that
    /// represents the set of supported strings the given option supports. For example,
    /// `'completeopt'` is a comma-separated string, made up of any combination of values `menu`,
    /// `menuone`, `longest`, `preview`, `noinsert`, and `noselect`. Instead of allowing the user
    /// to pass in this comma-seaprated string, we define the struct `CompleteOptSettings`, which
    /// allows you to toggle each one of those sub-options/flags, but can then be converted into a
    /// string and used accordingly--no typos, no duplicates.
    ///
    type Value: TryFrom<NvimString>;

    /// Analogous to `:set option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call. If the returned value can't be converted to a
    /// `NvimString`.
    ///
    fn get() -> Result<Self::Value, NvimOptionError> {
        let object = Self::get_object()?;
        let s = object.into_string_unchecked();

        Self::Value::try_from(s).map_err(NvimOptionError::from)
    }

    /// Analogous to `:set option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Self::Value) -> Result<(), NvimOptionError> {
        Ok(Self::set_object(NvimString::from(value))?)
    }

    /// Analogous to `:setglobal option?`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call. If the returned value can't be converted to a
    /// `NvimString`.
    ///
    fn get_global() -> Result<Self::Value, NvimOptionError> {
        let object = Self::get_global_object()?;
        let s = object.into_string_unchecked();

        Self::Value::try_from(s).map_err(NvimOptionError::from)
    }

    /// Analogous to `:setglobal option=value`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global(value: Self::Value) -> Result<(), NvimOptionError> {
        Ok(Self::set_global_object(NvimString::from(value))?)
    }
}

/// Error that happens when getting/setting options.
///
#[derive(Debug, Clone, thiserror::Error)]
pub enum NvimOptionError {
    /// A type of value for the option was expected, but another was returned.
    /// Really, this should only happen if there's a bug in `overkill`.
    ///
    #[error("Unexpected option value: '{:?}'", _0)]
    UnexpectedOptionValue(Object),

    /// For options that use key-codes, this can occur if a keycode is returned from nvim that
    /// isn't mapped to a type in `overkill`.
    ///
    #[error(transparent)]
    KeyCode(#[from] crate::key_code::InvalidKeyCode),

    /// Can occur if one type of `Object` was expected, but another was returned from nvim.
    ///
    #[error(transparent)]
    ObjectError(#[from] nvim_api::sys::api::nvim::object::Error),

    /// Can occur if an error occurred in nvim when getting/setting the option.
    ///
    #[error(transparent)]
    ApiError(#[from] nvim_api::api::Error),

    /// Shouldn't but theoretically could occur if using an integer type that's smaller than an
    /// `Integer`, and nvim returns a value outside of the capacity of that type. For example,
    /// `'conceallevel'` can only be `0`, `1`, `2`, or `3`, and so should thus be fine to be
    /// represented by a `u8` value; the call to nvim will always return a `Integer` (`i64`)
    /// though, so while getting `'conceallevel'` shouldn't ever return anything > 3, we return
    /// this error just in case there's a bug in this code.
    ///
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),

    /// Only necessary for some implementations where type converting is always guaranteed (i.e.
    /// implements `From`), but the trait defines a `TryFrom` bound _and_ whose `Error` type must
    /// be convertible to `NvimOptionError`.
    ///
    #[error(transparent)]
    _Infallible(#[from] Infallible),
}

macro_rules! impl_vim_option {
    (@base $option:ident, $short_name:expr, $long_name:expr) => {
        #[derive(Debug, Clone, Copy)]
        #[doc=concat!("Struct for representing the '", $long_name, "' option.")]
        pub struct $option;

        impl NvimOption for $option {
            const SHORT_NAME: &'static str = $short_name;
            const LONG_NAME: &'static str = $long_name;
        }
    };

    ($option:ident, string: $value:ty, $short_name:expr, $long_name:expr) => {
        impl_vim_option!(@base $option, $short_name, $long_name);

        impl StringOption for $option {
            type Value = $value;
        }
    };

    ($option:ident, num: $value:ty, $short_name:expr, $long_name:expr) => {
        impl_vim_option!(@base $option, $short_name, $long_name);

        impl NumberOption for $option {
            type Value = $value;
        }
    };

    ($option:ident, bool, $short_name:expr, $long_name:expr) => {
        impl_vim_option!(@base $option, $short_name, $long_name);

        impl BooleanOption for $option {}
    };
}

impl_vim_option!(AutoIndent, bool, "ai", "autoindent");
impl_vim_option!(BreakIndent, bool, "bri", "breakindent");
impl_vim_option!(CmdHeight, num: u8, "ch", "cmdheight");
impl_vim_option!(Clipboard, string: ClipboardSettings, "cb", "clipboard");
impl_vim_option!(
    ColorColumn,
    string: StringFlags<ColorColumnValue>,
    "cc",
    "colorcolumn"
);
impl_vim_option!(
    CompleteOpt,
    string: CompleteOptSettings,
    "cot",
    "completeopt"
);
impl_vim_option!(ConcealLevel, num: ConcealLevelValue, "cole", "conceallevel");
impl_vim_option!(CursorLine, bool, "cul", "cursorline");
impl_vim_option!(ExpandTab, bool, "et", "expandtab");
impl_vim_option!(FoldEnable, bool, "fen", "foldenable");
impl_vim_option!(GrepPrg, string: NvimString, "gp", "grepprg");
impl_vim_option!(Hidden, bool, "hid", "hidden");
impl_vim_option!(History, num: u32, "hi", "history");
impl_vim_option!(IncCommand, string: IncCommandValue, "icm", "inccommand");
impl_vim_option!(LineBreak, bool, "lbr", "linebreak");
impl_vim_option!(List, bool, "list", "list");
impl_vim_option!(ListChars, string: ListCharsSettings, "lcs", "listchars");
impl_vim_option!(Number, bool, "nu", "number");
impl_vim_option!(PasteToggle, string: KeyCode, "pt", "pastetoggle");
impl_vim_option!(ScrollOff, num: u16, "so", "scrolloff");
impl_vim_option!(
    ShortMess,
    string: CharFlags<ShortMessItem>,
    "shm",
    "shortmess"
);
impl_vim_option!(ShowTabline, num: ShowTablineValue, "stal", "showtabline");
impl_vim_option!(ShiftWidth, num: u8, "sw", "shiftwidth");
impl_vim_option!(SignColumn, string: SignColumnValue, "scl", "signcolumn");
impl_vim_option!(SmartCase, bool, "scs", "smartcase");
impl_vim_option!(SmartIndent, bool, "si", "smartindent");
impl_vim_option!(SoftTabStop, num: u8, "sts", "softtabstop");
impl_vim_option!(Spell, bool, "spell", "spell");
impl_vim_option!(
    SpellLang,
    string: StringFlags<SpellLangValue>,
    "spl",
    "spelllang"
);
impl_vim_option!(SplitBelow, bool, "sb", "splitbelow");
impl_vim_option!(SplitRight, bool, "spr", "splitright");
impl_vim_option!(SwapFile, bool, "swf", "swapfile");
impl_vim_option!(SynMaxCol, num: u32, "smc", "synmaxcol");
impl_vim_option!(TabStop, num: u8, "ts", "tabstop");
impl_vim_option!(TermGuiColors, bool, "tgc", "termguicolors");
impl_vim_option!(UndoFile, bool, "udf", "undofile");
impl_vim_option!(UpdateTime, num: u32, "ut", "updatetime");
impl_vim_option!(WildMenu, bool, "smnu", "wildmenu");
impl_vim_option!(WriteBackup, bool, "wb", "writebackup");

impl flags::AddAssignFlags for ShortMess {
    type Item = ShortMessItem;

    fn add_assign(rhs: Self::Item) -> Result<(), NvimOptionError> {
        let mut current = <Self as StringOption>::get()?;
        current.push(rhs);
        <Self as StringOption>::set(current)
    }

    fn add_assign_global(rhs: Self::Item) -> Result<(), NvimOptionError> {
        let mut current = <Self as StringOption>::get_global()?;
        current.push(rhs);
        <Self as StringOption>::set_global(current)
    }
}

impl flags::SubAssignFlags for ShortMess {
    type Item = ShortMessItem;

    fn sub_assign(rhs: &Self::Item) -> Result<(), NvimOptionError> {
        let mut current = <Self as StringOption>::get()?;
        current.remove(rhs);
        <Self as StringOption>::set(current)
    }

    fn sub_assign_global(rhs: &Self::Item) -> Result<(), NvimOptionError> {
        let mut current = <Self as StringOption>::get_global()?;
        current.remove(rhs);
        <Self as StringOption>::set_global(current)
    }
}
