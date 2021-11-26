use super::VimOptionError;
use nvim_api_rs::sys::api::vim::{LuaString, Object};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCharsSettings {
    eol: Option<char>,
    tab: Option<(char, char, Option<char>)>,
    space: Option<char>,
    lead: Option<char>,
    trail: Option<char>,
    extends: Option<char>,
    precedes: Option<char>,
    conceal: Option<char>,
    nbsp: Option<char>,
}

macro_rules! list_char_settings_def_single_char_fn {
    ($fn_name:ident) => {
        pub const fn $fn_name(self, setting: char) -> Self {
            let mut s = self;
            s.$fn_name = Some(setting);
            s
        }
    };
}

impl ListCharsSettings {
    list_char_settings_def_single_char_fn!(eol);
    list_char_settings_def_single_char_fn!(space);
    list_char_settings_def_single_char_fn!(lead);
    list_char_settings_def_single_char_fn!(trail);
    list_char_settings_def_single_char_fn!(extends);
    list_char_settings_def_single_char_fn!(precedes);
    list_char_settings_def_single_char_fn!(conceal);
    list_char_settings_def_single_char_fn!(nbsp);

    pub const fn tab(self, setting: (char, char, Option<char>)) -> Self {
        let mut s = self;
        s.tab = Some(setting);
        s
    }

    pub const fn tab2(self, char1: char, char2: char) -> Self {
        let mut s = self;
        s.tab = Some((char1, char2, None));
        s
    }

    pub const fn tab3(self, char1: char, char2: char, char3: char) -> Self {
        let mut s = self;
        s.tab = Some((char1, char2, Some(char3)));
        s
    }
}

impl From<ListCharsSettings> for Object {
    fn from(value: ListCharsSettings) -> Self {
        let mut settings_string = String::new();

        if let Some(eol) = value.eol {
            settings_string += &format!("eol:{}", eol);
        }

        if let Some(tab) = value.tab {
            if let Some(tab_2) = tab.2 {
                settings_string += &format!("tab:{}{}{}", tab.0, tab.1, tab_2);
            } else {
                settings_string += &format!("tab:{}{}", tab.0, tab.1);
            }
        }

        if let Some(space) = value.space {
            settings_string += &format!("space:{}", space);
        }

        if let Some(lead) = value.lead {
            settings_string += &format!("lead:{}", lead);
        }

        if let Some(trail) = value.trail {
            settings_string += &format!("trail:{}", trail);
        }

        if let Some(extends) = value.extends {
            settings_string += &format!("extends:{}", extends);
        }

        if let Some(precedes) = value.precedes {
            settings_string += &format!("precedes:{}", precedes);
        }

        if let Some(conceal) = value.conceal {
            settings_string += &format!("conceal:{}", conceal);
        }

        if let Some(nbsp) = value.nbsp {
            settings_string += &format!("nbsp:{}", nbsp);
        }

        Self::from(LuaString::new_unchecked(settings_string))
    }
}

impl TryFrom<Object> for ListCharsSettings {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let mut settings = Self::default();

        for setting in value.as_string_unchecked().to_string_lossy().split(',') {
            let mut key_value = setting.split(':');
            let key = key_value.next().unwrap();
            let mut value_chars = key_value.next().unwrap().chars();

            match key {
                "tab" => {
                    settings.tab = Some((
                        value_chars.next().unwrap(),
                        value_chars.next().unwrap(),
                        value_chars.next(),
                    ));
                }
                "eol" => {
                    settings.eol = Some(value_chars.next().unwrap());
                }
                "space" => {
                    settings.space = Some(value_chars.next().unwrap());
                }
                "lead" => {
                    settings.lead = Some(value_chars.next().unwrap());
                }
                "trail" => {
                    settings.trail = Some(value_chars.next().unwrap());
                }
                "extends" => {
                    settings.extends = Some(value_chars.next().unwrap());
                }
                "precedes" => {
                    settings.precedes = Some(value_chars.next().unwrap());
                }
                "conceal" => {
                    settings.conceal = Some(value_chars.next().unwrap());
                }
                "nbsp" => {
                    settings.nbsp = Some(value_chars.next().unwrap());
                }
                _ => (),
            }
        }

        Ok(settings)
    }
}
