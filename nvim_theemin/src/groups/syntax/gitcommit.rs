use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(summary);
    def_group_fn!(comment);

    def_group_fn!(untracked);
    def_group_fn!(discarded);
    def_group_fn!(selected);
    def_group_fn!(unmerged);

    def_group_fn!(on_branch);
    def_group_fn!(branch);
    def_group_fn!(no_branch);

    def_group_fn!(discarded_type);
    def_group_fn!(selected_type);
    def_group_fn!(unmerged_type);
    def_group_fn!(_type);

    def_group_fn!(no_changes);
    def_group_fn!(header);

    def_group_fn!(untracked_file);
    def_group_fn!(discarded_file);
    def_group_fn!(selected_file);
    def_group_fn!(unmerged_file);
    def_group_fn!(file);

    def_group_fn!(discarded_arrow);
    def_group_fn!(selected_arrow);
    def_group_fn!(unmerged_arrow);
    def_group_fn!(arrow);
    def_group_fn!(blank);
}

/// `From runtime/syntax/gitcommit.vim`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Summary,
    Comment,

    Untracked,
    Discarded,
    Selected,
    Unmerged,

    OnBranch,
    Branch,
    NoBranch,

    DiscardedType,
    SelectedType,
    UnmergedType,
    Type,

    NoChanges,
    Header,

    UntrackedFile,
    DiscardedFile,
    SelectedFile,
    UnmergedFile,
    File,

    DiscardedArrow,
    SelectedArrow,
    UnmergedArrow,
    Arrow,

    Blank,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Summary => "gitcommitSummary",
            Self::Comment => "gitcommitComment",
            Self::Untracked => "gitcommitUntracked",
            Self::Discarded => "gitcommitDiscarded",
            Self::Selected => "gitcommitSelected",
            Self::Unmerged => "gitcommitUnmerged",
            Self::OnBranch => "gitcommitOnBranch",
            Self::Branch => "gitcommitBranch",
            Self::NoBranch => "gitcommitNoBranch",
            Self::DiscardedType => "gitcommitDiscardedType",
            Self::SelectedType => "gitcommitSelectedType",
            Self::UnmergedType => "gitcommitUnmergedType",
            Self::Type => "gitcommitType",
            Self::NoChanges => "gitcommitNoChanges",
            Self::Header => "gitcommitHeader",
            Self::UntrackedFile => "gitcommitUntrackedFile",
            Self::DiscardedFile => "gitcommitDiscardedFile",
            Self::SelectedFile => "gitcommitSelectedFile",
            Self::UnmergedFile => "gitcommitUnmergedFile",
            Self::File => "gitcommitFile",
            Self::DiscardedArrow => "gitcommitDiscardedArrow",
            Self::SelectedArrow => "gitcommitSelectedArrow",
            Self::UnmergedArrow => "gitcommitUnmergedArrow",
            Self::Arrow => "gitcommitArrow",
            Self::Blank => "gitcommitBlank",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
