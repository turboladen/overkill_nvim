use neovim_sys::api::vim::{LuaString, Object};

/// Type used for joining multiple values into a comma-joined string. (ex. colorcolumn=80,120)
///
#[derive(Debug, Clone)]
pub struct StringFlags<T>(Vec<T>)
where
    String: From<T>;

impl<T> StringFlags<T>
where
    String: From<T>,
    T: PartialEq,
{
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn remove(&mut self, value: &T) {
        self.0.retain(|v| v == value)
    }
}

impl<T> From<StringFlags<T>> for Object
where
    String: From<T>,
{
    fn from(value: StringFlags<T>) -> Self {
        let s = value
            .0
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
            .join(",");

        Self::from(LuaString::new_unchecked(s))
    }
}
