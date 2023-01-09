//! Things from `$VIMRUNTIME/syntax/`
//!

pub mod diff;
pub mod eruby;
pub mod gitcommit;
pub mod help;
pub mod json;
pub mod lua;
// pub mod make;
pub mod markdown;
pub mod netrw;
pub mod ruby;
pub mod rust;
pub mod sh;
pub mod vim;
pub mod yaml;
pub mod zsh;
// pub mod sh_zsh;

use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    fn highlighting(&self) -> crate::Result {
        // TODO: Finish me
        Ok(())
    }

    def_group_fn!(comment);
    def_group_fn!(constant);

    def_group_fn!(string);
    def_group_fn!(character);
    def_group_fn!(number);
    def_group_fn!(boolean);
    def_group_fn!(float);

    def_group_fn!(identifier);
    def_group_fn!(function);

    def_group_fn!(statement);
    def_group_fn!(conditional);
    def_group_fn!(repeat);
    def_group_fn!(label);
    def_group_fn!(operator);
    def_group_fn!(keyword);
    def_group_fn!(exception);

    def_group_fn!(pre_proc);
    def_group_fn!(include);
    def_group_fn!(define);
    def_group_fn!(_macro);
    def_group_fn!(pre_condit);

    def_group_fn!(_type);
    def_group_fn!(storage_class);
    def_group_fn!(structure);
    def_group_fn!(typedef);

    def_group_fn!(special);
    def_group_fn!(special_char);
    def_group_fn!(tag);
    def_group_fn!(delimiter);
    def_group_fn!(debug);

    def_group_fn!(underlined);
    def_group_fn!(ignore);
    def_group_fn!(error);
    def_group_fn!(todo);
}

/// `:help group-name`
///
#[derive(Debug, Clone, Copy)]
pub enum SuggestedGroupName {
    Comment,

    Constant,
    String,
    Character,
    Number,
    Boolean,
    Float,

    Identifier,
    Function,

    Statement,
    Conditional,
    Repeat,
    Label,
    Operator,
    Keyword,
    Exception,

    PreProc,
    Include,
    Define,
    Macro,
    PreCondit,

    Type,
    StorageClass,
    Structure,
    Typedef,

    Special,
    SpecialChar,
    Tag,
    Delimiter,
    SpecialComment,
    Debug,

    Underlined,

    Ignore,

    Error,

    Todo,
}

impl AsGroupName for SuggestedGroupName {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Comment => "Comment",
            Self::Constant => "Constant",
            Self::String => "String",
            Self::Character => "Character",
            Self::Number => "Number",
            Self::Boolean => "Boolean",
            Self::Float => "Float",
            Self::Identifier => "Identifier",
            Self::Function => "Function",
            Self::Statement => "Statement",
            Self::Conditional => "Conditional",
            Self::Repeat => "Repeat",
            Self::Label => "Label",
            Self::Operator => "Operator",
            Self::Keyword => "Keyword",
            Self::Exception => "Exception",
            Self::PreProc => "PreProc",
            Self::Include => "Include",
            Self::Define => "Define",
            Self::Macro => "Macro",
            Self::PreCondit => "PreCondit",
            Self::Type => "Type",
            Self::StorageClass => "StorageClass",
            Self::Structure => "Structure",
            Self::Typedef => "Typedef",
            Self::Special => "Special",
            Self::SpecialChar => "SpecialChar",
            Self::Tag => "Tag",
            Self::Delimiter => "Delimiter",
            Self::SpecialComment => "SpecialComment",
            Self::Debug => "Debug",
            Self::Underlined => "Underlined",
            Self::Ignore => "Ignore",
            Self::Error => "Error",
            Self::Todo => "Todo",
        }
    }
}

impl ToHighlightCommand for SuggestedGroupName {}

impl ToLinkCommand for SuggestedGroupName {}
