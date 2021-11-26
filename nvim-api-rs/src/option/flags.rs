mod char_flags;
mod string_flags;

pub use self::{char_flags::CharFlags, string_flags::StringFlags};

use super::VimOption;
use crate::api::Error;
use neovim_sys::api::vim::Object;

/// Trait that allows mimicking vim's operators for options that can contain lists of flags:
///
/// - `set {option}+={value}
///
pub trait AddAssignFlags: VimOption
where
    Object: From<Self::Value>,
    Error: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Item: PartialEq;

    /// Like `set {option}+={value}.
    ///
    fn add_assign(rhs: Self::Item) -> Result<(), Error>;

    /// Like `setglobal {option}+={value}.
    ///
    fn add_assign_global(rhs: Self::Item) -> Result<(), Error>;
}

pub trait SubAssignFlags: VimOption
where
    Object: From<Self::Value>,
    Error: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Item: PartialEq;

    /// Like `set {option}-={value}.
    ///
    fn sub_assign(rhs: &Self::Item) -> Result<(), Error>;

    /// Like `setglobal {option}-={value}.
    ///
    fn sub_assign_global(rhs: &Self::Item) -> Result<(), Error>;
}
