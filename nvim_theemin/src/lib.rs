macro_rules! def_group_fn {
    ($name:ident) => {
        fn $name<'a, T>(&self) -> Option<T>
        where
            crate::HighlightAttributes<'a>: From<T>,
        {
            None
        }
    };
}

pub mod color;
pub mod groups;
pub mod highlight;
pub mod theme;

pub type Result = std::result::Result<(), nvim_oxi::api::Error>;

pub trait AsGroupName {
    fn as_group_name(&self) -> &'static str;
}

pub enum HighlightAttributes<'a> {
    Highlight(highlight::group::Group<'a>),
    Link(highlight::link::Link<'a>),
}

impl<'a> From<highlight::group::Group<'a>> for HighlightAttributes<'a> {
    fn from(value: highlight::group::Group<'a>) -> Self {
        Self::Highlight(value)
    }
}

impl<'a> From<highlight::link::Link<'a>> for HighlightAttributes<'a> {
    fn from(value: highlight::link::Link<'a>) -> Self {
        Self::Link(value)
    }
}

pub trait ToHighlightCommand: AsGroupName {
    fn highlight(&self, attributes: &highlight::group::Group<'_>) -> crate::Result {
        nvim_oxi::api::command(&self.to_highlight_command(attributes))
    }

    fn to_highlight_command(&self, attributes: &crate::highlight::group::Group<'_>) -> String {
        format!("{} {}", self.as_group_name(), attributes.cmd())
    }
}

pub trait ToLinkCommand: AsGroupName + Copy {
    fn highlight_link(&self, attributes: &highlight::link::Link<'_>) -> crate::Result {
        nvim_oxi::api::command(&self.to_link_command(attributes))
    }

    fn to_link_command(&self, attributes: &crate::highlight::link::Link<'_>) -> String {
        attributes.cmd(*self)
    }
}
