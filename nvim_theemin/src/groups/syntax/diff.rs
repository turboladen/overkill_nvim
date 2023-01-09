use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(old_file);
    def_group_fn!(new_file);
    def_group_fn!(index_line);
    def_group_fn!(file);
    def_group_fn!(only);
    def_group_fn!(identical);
    def_group_fn!(differ);
    def_group_fn!(b_differ);
    def_group_fn!(is_a);
    def_group_fn!(no_eol);
    def_group_fn!(common);
    def_group_fn!(removed);
    def_group_fn!(changed);
    def_group_fn!(added);
    def_group_fn!(line);
    def_group_fn!(subname);
    def_group_fn!(comment);
}

/// `From runtime/syntax/diff.vim`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    OldFile,
    NewFile,
    IndexLine,
    Only,
    Identical,
    Differ,
    BDiffer,
    IsA,
    NoEol,
    Common,
    Removed,
    Changed,
    Added,
    Line,
    Subname,
    Comment,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::OldFile => "diffOldFile",
            Self::NewFile => "diffNewFile",
            Self::IndexLine => "diffIndexLine",
            Self::Only => "diffOnly",
            Self::Identical => "diffIdentical",
            Self::Differ => "diffDiffer",
            Self::BDiffer => "diffBDiffer",
            Self::IsA => "diffIsA",
            Self::NoEol => "diffNoEOL",
            Self::Common => "diffCommon",
            Self::Removed => "diffRemoved",
            Self::Changed => "diffChanged",
            Self::Added => "diffAdded",
            Self::Line => "diffLine",
            Self::Subname => "diffSubname",
            Self::Comment => "diffComment",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
