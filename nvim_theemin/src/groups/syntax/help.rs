use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(ignore);
    def_group_fn!(hyper_text_jump);
    def_group_fn!(bar);
    def_group_fn!(backtick);
    def_group_fn!(star);
    def_group_fn!(hyper_text_entry);
    def_group_fn!(headline);
    def_group_fn!(header);
    def_group_fn!(section_delim);
    def_group_fn!(vim);
    def_group_fn!(command);
    def_group_fn!(example);
    def_group_fn!(option);
    def_group_fn!(special);
    def_group_fn!(note);
    def_group_fn!(warning);
    def_group_fn!(deprecated);

    def_group_fn!(comment);
    def_group_fn!(constant);
    def_group_fn!(string);
    def_group_fn!(character);
    def_group_fn!(number);
    def_group_fn!(boolean);
    def_group_fn!(float);
    def_group_fn!(identifier);
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
    def_group_fn!(special_char);
    def_group_fn!(tag);
    def_group_fn!(delimiter);
    def_group_fn!(special_comment);
    def_group_fn!(debug);
    def_group_fn!(underlined);
    def_group_fn!(error);
    def_group_fn!(todo);
    def_group_fn!(url);
}

/// `From runtime/syntax/help.vim`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Ignore,
    HyperTextJump,
    Bar,
    Backtick,
    Star,
    HyperTextEntry,
    Headline,
    Header,
    SectionDelim,
    Vim,
    Command,
    Example,
    Option,
    Special,
    Note,
    Warning,
    Deprecated,

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
    SpecialChar,
    Tag,
    Delimiter,
    SpecialComment,
    Debug,
    Underlined,
    Error,
    Todo,
    Url,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Ignore => "helpIgnore",
            Self::HyperTextJump => "helpHyperTextJump",
            Self::Bar => "helpBar",
            Self::Backtick => "helpBacktick",
            Self::Star => "helpStar",
            Self::HyperTextEntry => "helpHyperTextEntry",
            Self::Headline => "helpHeadline",
            Self::Header => "helpHeader",
            Self::SectionDelim => "helpSectionDelim",
            Self::Vim => "helpVim",
            Self::Command => "helpCommand",
            Self::Example => "helpExample",
            Self::Option => "helpOption",
            Self::Special => "helpSpecial",
            Self::Note => "helpNote",
            Self::Warning => "helpWarning",
            Self::Deprecated => "helpDeprecated",
            Self::Comment => "helpComment",
            Self::Constant => "helpConstant",
            Self::String => "helpString",
            Self::Character => "helpCharacter",
            Self::Number => "helpNumber",
            Self::Boolean => "helpBoolean",
            Self::Float => "helpFloat",
            Self::Identifier => "helpIdentifier",
            Self::Function => "helpFunction",
            Self::Statement => "helpStatement",
            Self::Conditional => "helpConditional",
            Self::Repeat => "helpRepeat",
            Self::Label => "helpLabel",
            Self::Operator => "helpOperator",
            Self::Keyword => "helpKeyword",
            Self::Exception => "helpException",
            Self::PreProc => "helpPreProc",
            Self::Include => "helpInclude",
            Self::Define => "helpDefine",
            Self::Macro => "helpMacro",
            Self::PreCondit => "helpPreCondit",
            Self::Type => "helpType",
            Self::StorageClass => "helpStorageClass",
            Self::Structure => "helpStructure",
            Self::Typedef => "helpTypeDef",
            Self::SpecialChar => "helpSpecialChar",
            Self::Tag => "helpTag",
            Self::Delimiter => "helpDelimiter",
            Self::SpecialComment => "helpSpecialComment",
            Self::Debug => "helpDebug",
            Self::Underlined => "helpUnderlined",
            Self::Error => "helpError",
            Self::Todo => "helpTodo",
            Self::Url => "helpUrl",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
