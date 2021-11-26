mod char_flags;
mod string_flags;

pub use self::{char_flags::CharFlags, string_flags::StringFlags};

use super::{VimOption, VimOptionError};
use nvim_api_rs::sys::api::vim::Object;

/// Trait that allows mimicking vim's operators for options that can contain lists of flags:
///
/// - `set {option}+={value}
///
pub trait AddAssignFlags: VimOption
where
    Object: From<Self::Value>,
    VimOptionError: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Item: PartialEq;

    /// Like `set {option}+={value}.
    ///
    fn add_assign(rhs: Self::Item) -> Result<(), VimOptionError>;

    /// Like `setglobal {option}+={value}.
    ///
    fn add_assign_global(rhs: Self::Item) -> Result<(), VimOptionError>;
}

pub trait SubAssignFlags: VimOption
where
    Object: From<Self::Value>,
    VimOptionError: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Item: PartialEq;

    /// Like `set {option}-={value}.
    ///
    fn sub_assign(rhs: &Self::Item) -> Result<(), VimOptionError>;

    /// Like `setglobal {option}-={value}.
    ///
    fn sub_assign_global(rhs: &Self::Item) -> Result<(), VimOptionError>;
}
