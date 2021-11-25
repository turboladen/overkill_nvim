// impl<O, U> AddAssignFlags for O
// where
//     O: GlobalLocal<Value = StringFlags<U>>,
//     StringFlags<U>: From<Object>,
//     String: From<U>,
//     Error: From<<<O as VimOption>::Value as TryFrom<Object>>::Error>,
//     U: PartialEq,
// {
//     type Item = U;

//     fn add_assign(rhs: Self::Item) -> Result<(), Error> {
//         let mut current = Self::get()?;
//         current.push(rhs);
//         Self::set(current)
//     }
// }

// impl<O, U> SubAssignFlags for O
// where
//     O: GlobalLocal<Value = StringFlags<U>>,
//     StringFlags<U>: From<Object>,
//     String: From<U>,
//     Error: From<<<O as VimOption>::Value as TryFrom<Object>>::Error>,
//     U: PartialEq,
// {
//     type Item = U;

//     fn sub_assign(rhs: &Self::Item) -> Result<(), Error> {
//         let mut current = Self::get()?;
//         current.remove(rhs);
//         Self::set(current)
//     }
// }

// impl<O, U> AddAssignFlags for O
// where
//     O: GlobalLocal<Value = CharFlags<U>>,
//     CharFlags<U>: From<Object>,
//     char: From<U>,
//     Error: From<<<O as VimOption>::Value as TryFrom<Object>>::Error>,
//     U: PartialEq,
// {
//     type Item = U;

//     fn add_assign(rhs: Self::Item) -> Result<(), Error> {
//         let mut current = Self::get()?;
//         current.push(rhs);
//         Self::set(current)
//     }
// }

// impl<O, U> SubAssignFlags for O
// where
//     O: GlobalLocal<Value = CharFlags<U>>,
//     CharFlags<U>: From<Object>,
//     char: From<U>,
//     Error: From<<<O as VimOption>::Value as TryFrom<Object>>::Error>,
//     U: PartialEq,
// {
//     type Item = U;

//     fn sub_assign(rhs: &Self::Item) -> Result<(), Error> {
//         let mut current = Self::get()?;
//         current.remove(rhs);
//         Self::set(current)
//     }
// }
