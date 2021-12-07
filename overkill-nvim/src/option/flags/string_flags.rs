use nvim_api::sys::api::nvim::NvimString;

/// Type used for joining multiple values into a comma-joined string.
///
/// ```compile_fail
/// use overkill_nvim::option::{SpellLang, StringFlags, SpellLangValue};
///
/// // Same as `:set seplllang=en_us,fr`:
/// SpellLang::set(StringFlags::new(vec![
///     SpellLangValue::EnUs,   // en_us
///     SpellLangValue::Fr,     // fr
/// ])).ok();
/// ```
///
#[derive(Debug, Clone)]
pub struct StringFlags<T>(Vec<T>)
where
    NvimString: From<T>;

impl<T> StringFlags<T>
where
    NvimString: From<T>,
    T: PartialEq,
{
    /// Basic constructor.
    ///
    #[must_use]
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }

    /// Pushes an element to the flag list.
    ///
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Removes an element from the flag list.
    ///
    pub fn remove(&mut self, value: &T) {
        self.0.retain(|v| v != value);
    }
}

impl<T> From<StringFlags<T>> for NvimString
where
    Self: From<T>,
{
    fn from(value: StringFlags<T>) -> Self {
        let s = value
            .0
            .into_iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join(",");

        Self::new_unchecked(s)
    }
}

// TODO: Do this after manually implementing a few times.
// impl<'a, T> TryFrom<NvimString> for StringFlags<T>
// where
//     NvimString: From<T>,
//     T: TryFrom<&'a str>,
//     T::Error: std::fmt::Debug,
//     NvimOptionError: From<<T as TryFrom<NvimString>>::Error>,
// {
//     type Error = NvimOptionError;

//     fn try_from(string: NvimString) -> Result<Self, Self::Error> {
//         let s = string.to_string_lossy();
//         let chunks = s.split(',');
//         let mut inner = Vec::with_capacity(chunks.size_hint().0);

//         for ref chunk in chunks.into_iter() {
//             inner.push(T::try_from(chunk)?)
//         }

//         Ok(Self(inner))
//     }
// }
