use neovim_sys::api::vim::{LuaString, Object};
use std::convert::TryFrom;
use super::{CharFlags, VimOptionError};

impl TryFrom<Object> for CharFlags<ShortMessItem> {
    type Error = VimOptionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let s = object.as_string_unchecked().to_string_lossy();
        let mut inner = Vec::with_capacity(s.len());

        for char in s.chars() {
            inner.push(ShortMessItem::try_from(char)?);
        }

        Ok(Self::new(inner))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShortMessItem {
    /// `f`
    AbbreviateFile,
    /// `i`
    AbbreviateIncompleteLastLine,
    /// `l`
    AbbreviateLinesAndChars,
    /// `m`
    AbbreviateModified,
    /// `n`
    AbbreviateNewFile,
    /// `r`
    AbbreviateReadOnly,
    /// `w`
    AbbreviateWritten,
    /// `x`
    AbbreviateFormat,
    /// `a`
    AllAbbreviations,
    /// `o`
    OverwriteFileWriteMessagesWithFileReadMessage,
    /// `O`
    OverwriteAllMessagesWithFileReadMessage,
    /// `s`
    SuppressSearchMessage,
    /// `t`
    TruncateFileMessage,
    /// `T`
    TruncateOtherMessages,
    /// `W`
    SuppressWrittenMessage,
    /// `A`
    SuppressAttentionMessage,
    /// `I`
    SuppressIntroMessage,
    /// `c`
    SuppressInsCompetionMenuMessages,
    /// `q`
    SuppressMacroNameWhenRecording,
    /// `F`
    SuppressFileInfoWhenEditing,
    /// `S`
    SuppressSearchCountMessage,
}

impl TryFrom<char> for ShortMessItem {
    type Error = VimOptionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let item = match value {
            'f' => Self::AbbreviateFile,
            'i' => Self::AbbreviateIncompleteLastLine,
            'l' => Self::AbbreviateLinesAndChars,
            'm' => Self::AbbreviateModified,
            'n' => Self::AbbreviateNewFile,
            'r' => Self::AbbreviateReadOnly,
            'w' => Self::AbbreviateWritten,
            'x' => Self::AbbreviateFormat,
            'a' => Self::AllAbbreviations,
            'o' => Self::OverwriteFileWriteMessagesWithFileReadMessage,
            'O' => Self::OverwriteAllMessagesWithFileReadMessage,
            's' => Self::SuppressSearchMessage,
            't' => Self::TruncateFileMessage,
            'T' => Self::TruncateOtherMessages,
            'W' => Self::SuppressWrittenMessage,
            'A' => Self::SuppressAttentionMessage,
            'I' => Self::SuppressIntroMessage,
            'c' => Self::SuppressInsCompetionMenuMessages,
            'q' => Self::SuppressMacroNameWhenRecording,
            'F' => Self::SuppressFileInfoWhenEditing,
            'S' => Self::SuppressSearchCountMessage,
            c => {
                return Err(VimOptionError::UnexpectedOptionValue(Object::from(
                    LuaString::new_unchecked(vec![c].into_iter().collect::<String>()),
                )))
            }
        };

        Ok(item)
    }
}

impl From<ShortMessItem> for char {
    fn from(item: ShortMessItem) -> Self {
        match item {
            ShortMessItem::AbbreviateFile => 'f',
            ShortMessItem::AbbreviateIncompleteLastLine => 'i',
            ShortMessItem::AbbreviateLinesAndChars => 'l',
            ShortMessItem::AbbreviateModified => 'm',
            ShortMessItem::AbbreviateNewFile => 'n',
            ShortMessItem::AbbreviateReadOnly => 'r',
            ShortMessItem::AbbreviateWritten => 'w',
            ShortMessItem::AbbreviateFormat => 'x',
            ShortMessItem::AllAbbreviations => 'a',
            ShortMessItem::OverwriteFileWriteMessagesWithFileReadMessage => 'o',
            ShortMessItem::OverwriteAllMessagesWithFileReadMessage => 'O',
            ShortMessItem::SuppressSearchMessage => 's',
            ShortMessItem::TruncateFileMessage => 't',
            ShortMessItem::TruncateOtherMessages => 'T',
            ShortMessItem::SuppressWrittenMessage => 'W',
            ShortMessItem::SuppressAttentionMessage => 'A',
            ShortMessItem::SuppressIntroMessage => 'I',
            ShortMessItem::SuppressInsCompetionMenuMessages => 'c',
            ShortMessItem::SuppressMacroNameWhenRecording => 'q',
            ShortMessItem::SuppressFileInfoWhenEditing => 'F',
            ShortMessItem::SuppressSearchCountMessage => 'S',
        }
    }
}
