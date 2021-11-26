use super::VimOptionError;
use neovim_sys::api::vim::{LuaString, Object};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Default)]
pub struct CompleteOptSettings {
    menu: bool,
    menu_one: bool,
    longest: bool,
    preview: bool,
    no_insert: bool,
    no_select: bool,
}

impl CompleteOptSettings {
    def_settings_builder_method!(menu);
    def_settings_builder_method!(menu_one);
    def_settings_builder_method!(longest);
    def_settings_builder_method!(preview);
    def_settings_builder_method!(no_insert);
    def_settings_builder_method!(no_select);
}

impl From<CompleteOptSettings> for Object {
    fn from(value: CompleteOptSettings) -> Self {
        let mut v = Vec::with_capacity(6);

        if value.menu {
            v.push("menu");
        }
        if value.menu_one {
            v.push("menuone");
        }
        if value.longest {
            v.push("longest");
        }
        if value.preview {
            v.push("preview");
        }
        if value.no_insert {
            v.push("noinsert");
        }
        if value.no_select {
            v.push("noselect");
        }
        let s = v.join(",");

        Object::from(LuaString::new_unchecked(s))
    }
}

impl TryFrom<Object> for CompleteOptSettings {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut settings = Self::default();

        for item in split {
            match item {
                "menu" => settings.menu = true,
                "menuone" => settings.menu_one = true,
                "longest" => settings.longest = true,
                "preview" => settings.preview = true,
                "noinsert" => settings.no_insert = true,
                "noselect" => settings.no_select = true,
                _ => (),
            }
        }

        Ok(settings)
    }
}
