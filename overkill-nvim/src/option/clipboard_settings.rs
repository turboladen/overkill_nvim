use super::NvimOptionError;
use nvim_api::sys::api::nvim::NvimString;

/// Allows for setting `'clipboard'` in a typed manner.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct ClipboardSettings {
    unnamed: bool,
    unnamed_plus: bool,
}

impl ClipboardSettings {
    def_settings_builder_method!(unnamed);
    def_settings_builder_method!(unnamed_plus);
}

impl From<ClipboardSettings> for NvimString {
    fn from(value: ClipboardSettings) -> Self {
        match (value.unnamed, value.unnamed_plus) {
            (true, true) => Self::new_unchecked("unnamed,unnamedplus"),
            (true, _) => Self::new_unchecked("unnamed"),
            (_, true) => Self::new_unchecked("unnamedplus"),
            (_, _) => Self::default(),
        }
    }
}

impl TryFrom<NvimString> for ClipboardSettings {
    type Error = NvimOptionError;

    fn try_from(value: NvimString) -> Result<Self, Self::Error> {
        let string = value.to_string_lossy();
        let split = string.split(',');
        let mut settings = Self::default();

        for item in split {
            match item {
                "unnamed" => settings.unnamed = true,
                "unnamedplus" => settings.unnamed_plus = true,
                _ => (),
            }
        }

        Ok(settings)
    }
}
