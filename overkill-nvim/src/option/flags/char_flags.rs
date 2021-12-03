use nvim_api::sys::api::nvim::{NvimString};

/// A collection of flags where each flag is a single character.
///
/// ```compile_fail
/// use overkill_nvim::option::{ShortMess, CharFlags, ShortMessItem};
///
/// // Same as `:set shortmess=flW`:
/// ShortMess::set(CharFlags::new(vec![
///     ShortMessItem::AbbreviateFile,          // f
///     ShortMessItem::AbbreviateLinesAndChars, // l
///     ShortMessItem::SuppressWrittenMessage,  // W
/// ])).ok();
/// ```
///
#[derive(Debug, Clone)]
pub struct CharFlags<T>(Vec<T>)
where
    char: From<T>;

impl<T> CharFlags<T>
where
    char: From<T>,
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
        self.0.retain(|v| v == value);
    }
}

impl<T> From<CharFlags<T>> for NvimString
where
    char: From<T>,
{
    fn from(value: CharFlags<T>) -> Self {
        let mut iter = value.0.into_iter().map(char::from);

        let s = match iter.next() {
            None => String::new(),
            Some(first_char) => {
                let (lower, _) = iter.size_hint();
                let mut result = String::with_capacity(lower);
                result.push(first_char);
                iter.for_each(|c| {
                    result.push(c);
                });
                result
            }
        };

        Self::new_unchecked(s)
    }
}
