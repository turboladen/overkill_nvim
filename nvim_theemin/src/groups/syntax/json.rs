use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(padding);
    def_group_fn!(string);
    def_group_fn!(test);
    def_group_fn!(escape);
    def_group_fn!(number);
    def_group_fn!(braces);
    def_group_fn!(null);
    def_group_fn!(boolean);
    def_group_fn!(keyword);

    def_group_fn!(num_error);
    def_group_fn!(comment_error);
    def_group_fn!(semicolon_error);
    def_group_fn!(trailing_comma_error);
    def_group_fn!(missing_comma_error);
    def_group_fn!(string_sq_error);
    def_group_fn!(no_quotes_error);
    def_group_fn!(triple_quotes_error);
}

/// `From runtime/syntax/json.vim`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Padding,
    String,
    Test,
    Escape,
    Number,
    Braces,
    Null,
    Boolean,
    Keyword,

    NumError,
    CommentError,
    SemicolonError,
    TrailingCommaError,
    MissingCommaError,
    StringSqError,
    NoQuotesError,
    TripleQuotesError,

    Quote,
    Noise,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Padding => "jsonPadding",
            Self::String => "jsonString",
            Self::Test => "jsonTest",
            Self::Escape => "jsonEscape",
            Self::Number => "jsonNumber",
            Self::Braces => "jsonBraces",
            Self::Null => "jsonNull",
            Self::Boolean => "jsonBoolean",
            Self::Keyword => "jsonKeyword",
            Self::NumError => "jsonNumError",
            Self::CommentError => "jsonCommentError",
            Self::SemicolonError => "jsonSemicolonError",
            Self::TrailingCommaError => "jsonTrailingCommaError",
            Self::MissingCommaError => "jsonMissingCommaError",
            Self::StringSqError => "jsonStringSqError",
            Self::NoQuotesError => "jsonNoQuotesError",
            Self::TripleQuotesError => "jsonTripleQuotesError",
            Self::Quote => "jsonQuote",
            Self::Noise => "jsonNoise",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
