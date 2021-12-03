#![allow(clippy::module_name_repetitions)]

//! Types and traits for dealing with options that are set using combinations of flags.
//!
mod char_flags;
mod string_flags;

pub use self::{char_flags::CharFlags, string_flags::StringFlags};

use super::{NvimOption, NvimOptionError};

/// Trait that allows mimicking vim's add-and-assign operator for options that can contain lists of flags:
///
/// - `set {option}+={value}`
///
pub trait AddAssignFlags: NvimOption {
    /// Each flag item is of this type.
    ///
    type Item: PartialEq;

    /// Like `set {option}+={value}`.
    ///
    /// # Errors
    ///
    /// * Errors if either getting or setting the value fails.
    ///
    fn add_assign(rhs: Self::Item) -> Result<(), NvimOptionError>;

    /// Like `setglobal {option}+={value}`.
    ///
    /// # Errors
    ///
    /// * Errors if either getting or setting the value fails.
    ///
    fn add_assign_global(rhs: Self::Item) -> Result<(), NvimOptionError>;
}

/// Trait that allows mimicking vim's subtract-and-assign operator for options that can contain lists of flags:
///
/// - `set {option}-={value}`
///
pub trait SubAssignFlags: NvimOption {
    /// Each flag item is of this type.
    ///
    type Item: PartialEq;

    /// Like `set {option}-={value}`.
    ///
    /// # Errors
    ///
    /// * Errors if either getting or setting the value fails.
    ///
    fn sub_assign(rhs: &Self::Item) -> Result<(), NvimOptionError>;

    /// Like `setglobal {option}-={value}`.
    ///
    /// # Errors
    ///
    /// * Errors if either getting or setting the value fails.
    ///
    fn sub_assign_global(rhs: &Self::Item) -> Result<(), NvimOptionError>;
}
