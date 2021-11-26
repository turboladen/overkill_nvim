use nvim_api_rs::sys::api::vim::{LuaString, Object};

#[derive(Debug, Clone)]
pub struct CharFlags<T>(Vec<T>)
where
    char: From<T>;

impl<T> CharFlags<T>
where
    char: From<T>,
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

impl<T> From<CharFlags<T>> for Object
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

        Self::from(LuaString::new_unchecked(s))
    }
}
