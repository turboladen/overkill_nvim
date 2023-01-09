use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

pub trait Highlighting {
    def_group_fn!(toml_comment);
    def_group_fn!(toml_todo);

    def_group_fn!(toml_table_array);
    def_group_fn!(toml_table);

    def_group_fn!(toml_dot_in_key);
    def_group_fn!(toml_key_sq);
    def_group_fn!(toml_key_dq);
    def_group_fn!(toml_key);

    def_group_fn!(toml_date);
    def_group_fn!(toml_boolean);
    def_group_fn!(toml_float);
    def_group_fn!(toml_integer);
    def_group_fn!(toml_string);
    def_group_fn!(toml_line_escape);
    def_group_fn!(toml_escape);

    // def_group_fn!(toml_ts_property);
    // def_group_fn!(toml_ts_string);

    // def_group_fn!(property_toml);
    // def_group_fn!(string_toml);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    TomlComment,
    TomlTodo,
    TomlTableArray,
    TomlTable,
    TomlDotInKey,
    TomlKeySq,
    TomlKeyDq,
    TomlKey,
    TomlDate,
    TomlBoolean,
    TomlFloat,
    TomlInteger,
    TomlString,
    TomlLineEscape,
    TomlEscape,
    // TomlTsProperty, => "tomlTSProperty"
    // TomlTsString, => "tomlTSString"
    // PropertyToml, => "@property.toml"
    // StringToml, => "@string.toml"
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::TomlTable => "tomlTable",
            Self::TomlKey => "tomlKey",
            Self::TomlString => "tomlString",
            Self::TomlDate => "tomlDate",
            Self::TomlBoolean => "tomlBoolean",
            Self::TomlTableArray => "tomlTableArray",
            Self::TomlComment => "tomlComment",
            Self::TomlTodo => "tomlTodo",
            Self::TomlDotInKey => "tomlDotInKey",
            Self::TomlKeySq => "tomlKeySq",
            Self::TomlKeyDq => "tomlKeyDq",
            Self::TomlFloat => "tomlFloat",
            Self::TomlInteger => "tomlInteger",
            Self::TomlLineEscape => "tomlLineEscape",
            Self::TomlEscape => "tomlEscape",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
