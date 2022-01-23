use nvim_api::NvimString;

use super::{NvimOptionError, StringFlags};
use std::convert::TryFrom;

/// Pulled from [the vim ftp site](http://ftp.vim.org/vim/runtime/spell/).
///
#[derive(Debug, Clone, PartialEq)]
pub enum SpellLangValue {
    /// Afrikaans.
    Af,

    /// Amharic.
    Am,

    /// Bulgarian.
    Bg,

    /// Breton.
    Br,

    /// Catalan, Valencian.
    Ca,

    /// Chinese/Japanese/Korean.
    Cjk,

    /// Czech.
    Cs,

    /// Welsh.
    Cy,

    /// Danish.
    Da,

    /// All German.
    De,

    /// Old German spellings.
    De19,

    /// New German spellings.
    De20,

    /// Austrian German.
    DeAt,

    /// Swiss German.
    DeCh,

    /// German German.
    DeDe,

    /// Modern Greek (1453-).
    El,

    /// All English.
    En,

    /// Australian English.
    EnAu,

    /// Canadian English.
    EnCa,

    /// Great Britain English.
    EnGb,

    /// New Zealand English.
    EnNz,

    /// US English.
    EnUs,

    /// Esperanto.
    Eo,

    /// All Spanish.
    Es,

    /// Spanish Spanish.
    EsEs,

    /// Mexican Spanish.
    EsMx,

    /// Faroese.
    Fo,

    /// French.
    Fr,

    /// Irish.
    Ga,

    /// Scottish Gaelic.
    Gd,

    /// Galician.
    Gl,

    /// Hebrew.
    He,

    /// Croatian.
    Hr,

    /// Hungarian.
    Hu,

    /// Indonesian.
    Id,

    /// Italian.
    It,

    /// Kurdish.
    Ku,

    /// Latin.
    La,

    /// Lithuanian.
    Lt,

    /// Latvian.
    Lv,

    /// Malagasy.
    Mg,

    /// Maori.
    Mi,

    /// Malay.
    Ms,

    /// Norwegian Bokmål.
    Nb,

    /// Dutch, Flemmish.
    Nl,

    /// Norwegian Nynorsk.
    Nn,

    /// Nyanja, Chewa, Chichewa.
    Ny,

    /// Polish.
    Pl,

    /// Portuguese.
    Pt,

    /// Brazilian Portuguese.
    PtBr,

    /// Portuguese Portuguese.
    PtPt,

    /// Romanian, Moldavian, Moldovan.
    Ro,

    /// Russian.
    Ru,

    /// Russian, using "IE" spellings.
    RuRu,

    /// Russian, using "YO" spellings.
    RuYo,

    /// Kinyarwanda.
    Rw,

    /// Slovak.
    Sk,

    /// Slovenian.
    Sl,

    /// Serbian.
    Sr,

    /// Swedish.
    Sv,

    /// Swahili (macrolanguage).
    Sw,

    /// Tetum.
    Tet,

    /// Thai.
    Th,

    /// Tagalog.
    Tl,

    /// Tswana.
    Tn,

    /// Ukrainian.
    Uk,

    /// Yiddish.
    Yi,

    /// Zulu.
    Zu,

    /// Allows for specifying your own dictionary, like "medical", "en-rare", or whatever
    /// floats your boat.
    Custom(String),
}

#[allow(clippy::fallible_impl_from)]
impl From<SpellLangValue> for NvimString {
    fn from(spelllang_value: SpellLangValue) -> Self {
        let s = match spelllang_value {
            SpellLangValue::Af => "af",
            SpellLangValue::Am => "am",
            SpellLangValue::Bg => "bg",
            SpellLangValue::Br => "br",
            SpellLangValue::Ca => "ca",
            SpellLangValue::Cjk => "cjk",
            SpellLangValue::Cs => "cs",
            SpellLangValue::Cy => "cy",
            SpellLangValue::Da => "da",
            SpellLangValue::De => "de",
            SpellLangValue::De19 => "de_19",
            SpellLangValue::De20 => "de_20",
            SpellLangValue::DeAt => "de_at",
            SpellLangValue::DeCh => "de_ch",
            SpellLangValue::DeDe => "de_de",
            SpellLangValue::El => "el",
            SpellLangValue::En => "en",
            SpellLangValue::EnAu => "en_au",
            SpellLangValue::EnCa => "en_ca",
            SpellLangValue::EnGb => "en_gb",
            SpellLangValue::EnNz => "en_nz",
            SpellLangValue::EnUs => "en_us",
            SpellLangValue::Eo => "eo",
            SpellLangValue::Es => "es",
            SpellLangValue::EsEs => "es_es",
            SpellLangValue::EsMx => "es_mx",
            SpellLangValue::Fo => "fo",
            SpellLangValue::Fr => "fr",
            SpellLangValue::Ga => "ga",
            SpellLangValue::Gd => "gd",
            SpellLangValue::Gl => "gl",
            SpellLangValue::He => "he",
            SpellLangValue::Hr => "hr",
            SpellLangValue::Hu => "hu",
            SpellLangValue::Id => "id",
            SpellLangValue::It => "it",
            SpellLangValue::Ku => "ku",
            SpellLangValue::La => "la",
            SpellLangValue::Lt => "lt",
            SpellLangValue::Lv => "lv",
            SpellLangValue::Mg => "mg",
            SpellLangValue::Mi => "mi",
            SpellLangValue::Ms => "ms",
            SpellLangValue::Nb => "nb",
            SpellLangValue::Nl => "nl",
            SpellLangValue::Nn => "nn",
            SpellLangValue::Ny => "ny",
            SpellLangValue::Pl => "pl",
            SpellLangValue::Pt => "pt",
            SpellLangValue::PtBr => "pt_br",
            SpellLangValue::PtPt => "pt_pt",
            SpellLangValue::Ro => "ro",
            SpellLangValue::Ru => "ru",
            SpellLangValue::RuRu => "ru_ru",
            SpellLangValue::RuYo => "ru_yo",
            SpellLangValue::Rw => "rw",
            SpellLangValue::Sk => "sk",
            SpellLangValue::Sl => "sl",
            SpellLangValue::Sr => "sr",
            SpellLangValue::Sv => "sv",
            SpellLangValue::Sw => "sw",
            SpellLangValue::Tet => "tet",
            SpellLangValue::Th => "th",
            SpellLangValue::Tl => "tl",
            SpellLangValue::Tn => "tn",
            SpellLangValue::Uk => "uk",
            SpellLangValue::Yi => "yi",
            SpellLangValue::Zu => "zu",
            SpellLangValue::Custom(locale) => return Self::new(locale).unwrap(),
        };

        Self::new_unchecked(s)
    }
}

impl<'a> From<&'a str> for SpellLangValue {
    fn from(s: &str) -> Self {
        match s {
            "af" => Self::Af,
            "am" => Self::Am,
            "bg" => Self::Bg,
            "br" => Self::Br,
            "ca" => Self::Ca,
            "cs" => Self::Cs,
            "cy" => Self::Cy,
            "da" => Self::Da,
            "de" => Self::De,
            "de_19" => Self::De19,
            "de_20" => Self::De20,
            "de_at" => Self::DeAt,
            "de_ch" => Self::DeCh,
            "de_de" => Self::DeDe,
            "el" => Self::El,
            "en" => Self::En,
            "en_au" => Self::EnAu,
            "en_ca" => Self::EnCa,
            "en_gb" => Self::EnGb,
            "en_nz" => Self::EnNz,
            "en_us" => Self::EnUs,
            "eo" => Self::Eo,
            "es" => Self::Es,
            "es_es" => Self::EsEs,
            "es_mx" => Self::EsMx,
            "fo" => Self::Fo,
            "fr" => Self::Fr,
            "ga" => Self::Ga,
            "gd" => Self::Gd,
            "gl" => Self::Gl,
            "he" => Self::He,
            "hr" => Self::Hr,
            "hu" => Self::Hu,
            "id" => Self::Id,
            "it" => Self::It,
            "ku" => Self::Ku,
            "la" => Self::La,
            "lt" => Self::Lt,
            "lv" => Self::Lv,
            "mg" => Self::Mg,
            "mi" => Self::Mi,
            "ms" => Self::Ms,
            "nb" => Self::Nb,
            "nl" => Self::Nl,
            "nn" => Self::Nn,
            "ny" => Self::Ny,
            "pl" => Self::Pl,
            "pt" => Self::Pt,
            "pt_br" => Self::PtBr,
            "pt_pt" => Self::PtPt,
            "ro" => Self::Ro,
            "ru" => Self::Ru,
            "ru_ru" => Self::RuRu,
            "ru_yo" => Self::RuYo,
            "rw" => Self::Rw,
            "sk" => Self::Sk,
            "sl" => Self::Sl,
            "sr" => Self::Sr,
            "sv" => Self::Sv,
            "sw" => Self::Sw,
            "tet" => Self::Tet,
            "th" => Self::Th,
            "tl" => Self::Tl,
            "tn" => Self::Tn,
            "uk" => Self::Uk,
            "yi" => Self::Yi,
            "zu" => Self::Zu,
            locale => Self::Custom(locale.to_string()),
        }
    }
}

impl TryFrom<NvimString> for StringFlags<SpellLangValue> {
    type Error = NvimOptionError;

    fn try_from(string: NvimString) -> Result<Self, Self::Error> {
        let string = string.to_string_lossy();
        let split = string.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            inner.push(SpellLangValue::from(item));
        }

        Ok(Self::new(inner))
    }
}
