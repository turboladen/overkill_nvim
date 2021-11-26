use super::VimOptionError;
use nvim_api_rs::sys::api::vim::{LuaString, Object};

#[derive(Debug, Clone, Copy, Default)]
pub struct ClipboardSettings {
    unnamed: bool,
    unnamed_plus: bool,
}

impl ClipboardSettings {
    def_settings_builder_method!(unnamed);
    def_settings_builder_method!(unnamed_plus);
}

impl From<ClipboardSettings> for Object {
    fn from(value: ClipboardSettings) -> Self {
        match (value.unnamed, value.unnamed_plus) {
            (true, true) => Self::from(LuaString::new_unchecked("unnamed,unnamedplus")),
            (true, _) => Self::from(LuaString::new_unchecked("unnamed")),
            (_, true) => Self::from(LuaString::new_unchecked("unnamedplus")),
            (_, _) => Self::new_nil(),
        }
    }
}

impl TryFrom<Object> for ClipboardSettings {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
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
