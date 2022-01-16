//#[derive(Debug, Clone, Copy)]
//#[allow(clippy::module_name_repetitions)]
//pub struct CurrentMode {
//    mode: Mode,
//    blocking: bool,
//}

//impl CurrentMode {
//    /// The current mode.
//    ///
//    pub const fn mode(self) -> Mode {
//        self.mode
//    }

//    /// `true` if Nvim is waiting for input.
//    ///
//    pub const fn blocking(self) -> bool {
//        self.blocking
//    }
//}

//impl TryFrom<Dictionary> for CurrentMode {
//    type Error = CurrentModeError;

//    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
//        match (dict.get("mode"), dict.get("blocking")) {
//            (Some(mode), Some(blocking)) if mode.is_string() && blocking.is_boolean() => Ok(Self {
//                mode: Mode::from(mode.as_string_unchecked()),
//                blocking: blocking.as_boolean_unchecked(),
//            }),
//            (None, Some(_)) => Err(CurrentModeError::Mode),
//            (Some(_), None) => Err(CurrentModeError::Blocking),
//            _ => Err(CurrentModeError::ModeAndBlocking),
//        }
//    }
//}

///// Error type for instantiating a `CurrentMode` from the `Dictionary` returned by neovim on
///// related calls.
/////
//#[derive(Debug, Clone, Copy, thiserror::Error)]
//pub enum CurrentModeError {
//    #[error("Underlying neovim dictionary did not have the `mode` key/value pair set")]
//    Mode,

//    #[error("Underlying neovim dictionary did not have the `blocking` key/value pair set")]
//    Blocking,

//    #[error(
//        "Underlying neovim dictionary did not have the `mode` or `blocking` key/value pairs set"
//    )]
//    ModeAndBlocking,
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use neovim_api::sys::nvim::{KeyValuePair, Object};

//    #[test]
//    fn try_from_dictionary_test() {
//        let dict = Dictionary::new_from([
//            KeyValuePair::new(
//                NvimString::new_unchecked("mode"),
//                Object::from(NvimString::new_unchecked("n")),
//            ),
//            KeyValuePair::new(NvimString::new_unchecked("blocking"), Object::from(false)),
//        ]);
//        let current_mode = CurrentMode::try_from(dict).unwrap();
//        assert_eq!(current_mode.mode(), Mode::Normal);
//    }
//}
