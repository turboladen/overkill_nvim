use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(statement);
    def_group_fn!(repeat);
    def_group_fn!(_for);
    def_group_fn!(string);
    def_group_fn!(string2);
    def_group_fn!(number);
    def_group_fn!(operator);
    def_group_fn!(in);
    def_group_fn!(constant);
    def_group_fn!(cond);
    def_group_fn!(else);
    def_group_fn!(function);
    def_group_fn!(comment);
    def_group_fn!(todo);
    def_group_fn!(table);
    def_group_fn!(error);
    def_group_fn!(paren_error);
    def_group_fn!(brace_error);
    def_group_fn!(special);
    def_group_fn!(func);
    def_group_fn!(label);
}

/// `From runtime/syntax/lua.vim`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Statement,
    Repeat,
    For,
    String,
    String2,
    Number,
    Operator,
    In,
    Constant,
    Cond,
    Else,
    Function,
    Comment,
    Todo,
    Table,
    Error,
    ParenError,
    BraceError,
    Special,
    Func,
    Label,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
}
    }
}
