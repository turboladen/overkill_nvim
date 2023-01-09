use crate::{AsGroupName, HighlightAttributes, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    fn highlight(&self) -> crate::Result {
        if let Some(attribs) = self.color_column() {
            match attribs {
                HighlightAttributes::Highlight(ref hl) => {
                    BuiltinGroupName::ColorColumn.highlight(hl)?
                }
                HighlightAttributes::Link(ref link) => {
                    BuiltinGroupName::ColorColumn.highlight_link(link)?
                }
            }
        }

        Ok(())
    }

    def_group_fn!(color_column);
    def_group_fn!(conceal);

    def_group_fn!(cur_search);
    def_group_fn!(cursor);
    def_group_fn!(l_cursor);
    def_group_fn!(cursor_im);
    def_group_fn!(cursor_column);
    def_group_fn!(cursor_line);

    def_group_fn!(directory);
    def_group_fn!(diff_add);
    def_group_fn!(diff_change);
    def_group_fn!(diff_delete);
    def_group_fn!(diff_text);

    def_group_fn!(end_of_buffer);
    def_group_fn!(term_cursor);
    def_group_fn!(term_cursor_nc);
    def_group_fn!(error_msg);
    def_group_fn!(win_separator);
    def_group_fn!(folded);
    def_group_fn!(fold_column);
    def_group_fn!(sign_column);
    def_group_fn!(inc_search);
    def_group_fn!(substitute);

    def_group_fn!(line_nr);
    def_group_fn!(line_nr_above);
    def_group_fn!(line_nr_below);
    def_group_fn!(cursor_line_nr);
    def_group_fn!(cursor_line_sign);
    def_group_fn!(cursor_line_fold);

    def_group_fn!(match_paren);
    def_group_fn!(mode_msg);
    def_group_fn!(msg_area);
    def_group_fn!(msg_separator);
    def_group_fn!(more_msg);

    def_group_fn!(non_text);
    def_group_fn!(normal);
    def_group_fn!(normal_float);
    def_group_fn!(normal_nc);

    def_group_fn!(pmenu);
    def_group_fn!(pmenu_sel);
    def_group_fn!(pmenu_s_bar);
    def_group_fn!(pmenu_thumb);

    def_group_fn!(question);
    def_group_fn!(quick_fix_line);

    def_group_fn!(search);
    def_group_fn!(special_key);
    def_group_fn!(spell_bad);
    def_group_fn!(spell_cap);
    def_group_fn!(spell_local);
    def_group_fn!(spell_rare);

    def_group_fn!(status_line);
    def_group_fn!(status_line_nc);
    def_group_fn!(tab_line);
    def_group_fn!(tab_line_fill);
    def_group_fn!(tab_line_sel);

    def_group_fn!(title);
    def_group_fn!(visual);
    def_group_fn!(visual_nos);
    def_group_fn!(warning_msg);
    def_group_fn!(whitespace);
    def_group_fn!(wild_menu);
    def_group_fn!(win_bar);
    def_group_fn!(win_bar_nc);

    def_group_fn!(user1);
    def_group_fn!(user2);
    def_group_fn!(user3);
    def_group_fn!(user4);
    def_group_fn!(user5);
    def_group_fn!(user6);
    def_group_fn!(user7);
    def_group_fn!(user8);
    def_group_fn!(user9);

    def_group_fn!(menu);
    def_group_fn!(scrollbar);
    def_group_fn!(tooltip);
}

#[derive(Clone, Copy, Debug)]
pub enum BuiltinGroupName {
    ColorColumn,
    Conceal,
    CurSearch,
    Cursor,
    LCursor,
    CursorIM,
    CursorColumn,
    CursorLine,
    Directory,
    DiffAdd,
    DiffChange,
    DiffDelete,
    DiffText,
    EndOfBuffer,
    TermCursor,
    TermCursorNC,
    ErrorMsg,
    WinSeparator,
    Folded,
    FoldColumn,
    SignColumn,
    IncSearch,
    Substitute,
    LineNr,
    LineNrAbove,
    LineNrBelow,
    CursorLineNr,
    CursorLineSign,
    CursorLineFold,
    MatchParen,
    ModeMsg,
    MsgArea,
    MsgSeparator,
    MoreMsg,
    NonText,
    Normal,
    NormalFloat,
    NormalNC,
    Pmenu,
    PmenuSel,
    PmenuSBar,
    PmenuThumb,
    Question,
    QuickFixLine,
    Search,
    SpecialKey,
    SpellBad,
    SpellCap,
    SpellLocal,
    SpellRare,
    StatusLine,
    StatusLineNC,
    TabLine,
    TabLineFill,
    TabLineSel,
    Title,
    Visual,
    VisualNOS,
    WarningMsg,
    Whitespace,
    WildMenu,
    WinBar,
    WinBarNC,

    User1,
    User2,
    User3,
    User4,
    User5,
    User6,
    User7,
    User8,
    User9,

    Menu,
    Scrollbar,
    Tooltip,
}

impl AsGroupName for BuiltinGroupName {
    fn as_group_name(&self) -> &'static str {
        match *self {
            Self::ColorColumn => "ColorColumn",
            Self::Conceal => "Conceal",
            Self::CurSearch => "CurSearch",
            Self::Cursor => "Cursor",
            Self::LCursor => "lCursor",
            Self::CursorIM => "CursorIM",
            Self::CursorColumn => "CursorColumn",
            Self::CursorLine => "CursorLine",

            Self::Directory => "Directory",
            Self::DiffAdd => "DiffAdd",
            Self::DiffChange => "DiffChange",
            Self::DiffDelete => "DiffDelete",
            Self::DiffText => "DiffText",
            Self::EndOfBuffer => "EndOfBuffer",
            Self::TermCursor => "TermCursor",
            Self::TermCursorNC => "TermCursorNC",
            Self::ErrorMsg => "ErrorMsg",
            Self::WinSeparator => "WinSeparator",
            Self::Folded => "Folded",
            Self::FoldColumn => "FoldColumn",
            Self::SignColumn => "SignColumn",
            Self::IncSearch => "IncSearch",
            Self::Substitute => "Substitute",
            Self::LineNr => "LineNr",
            Self::LineNrAbove => "LineNrAbove",
            Self::LineNrBelow => "LineNrBelow",
            Self::CursorLineNr => "CursorLineNr",
            Self::CursorLineSign => "CursorLineSign",
            Self::CursorLineFold => "CursorLineFold",

            Self::MatchParen => "MatchParen",
            Self::ModeMsg => "ModeMsg",
            Self::MsgArea => "MsgArea",
            Self::MsgSeparator => "MsgSeparator",
            Self::MoreMsg => "MoreMsg",

            Self::NonText => "NonText",
            Self::Normal => "Normal",
            Self::NormalFloat => "NormalFloat",
            Self::NormalNC => "NormalNC",

            Self::Pmenu => "Pmenu",
            Self::PmenuSel => "PmenuSel",
            Self::PmenuSBar => "PmenuSBar",
            Self::PmenuThumb => "PmenuThumb",

            Self::Question => "Question",
            Self::QuickFixLine => "QuickFixLine",

            Self::Search => "Search",
            Self::SpecialKey => "SpecialKey",
            Self::SpellBad => "SpellBad",
            Self::SpellCap => "SpellCap",
            Self::SpellLocal => "SpellLocal",
            Self::SpellRare => "SpellRare",

            Self::StatusLine => "StatusLine",
            Self::StatusLineNC => "StatusLineNC",
            Self::TabLine => "TabLine",
            Self::TabLineFill => "TabLineFill",
            Self::TabLineSel => "TabLineSel",

            Self::Title => "Title",
            Self::Visual => "Visual",
            Self::VisualNOS => "VisualNOS",
            Self::WarningMsg => "WarningMsg",
            Self::Whitespace => "Whitespace",
            Self::WildMenu => "WildMenu",
            Self::WinBar => "WinBar",
            Self::WinBarNC => "WinBarNC",

            Self::User1 => "User1",
            Self::User2 => "User2",
            Self::User3 => "User3",
            Self::User4 => "User4",
            Self::User5 => "User5",
            Self::User6 => "User6",
            Self::User7 => "User7",
            Self::User8 => "User8",
            Self::User9 => "User9",

            Self::Menu => "Menu",
            Self::Scrollbar => "Scrollbar",
            Self::Tooltip => "Tooltip",
        }
    }
}

impl ToHighlightCommand for BuiltinGroupName {}

impl ToLinkCommand for BuiltinGroupName {}
