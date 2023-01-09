use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(match_paren_cur);
    def_group_fn!(match_word);
    def_group_fn!(match_word_cur);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    MatchParenCur,
    MatchWord,
    MatchBackground,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::MatchParenCur => "MatchParenCur",
            Self::MatchWord => "MatchWord",
            Self::MatchBackground => "MatchBackground",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
