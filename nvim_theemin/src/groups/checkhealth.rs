use crate::AsGroupName;

pub trait Highlighting {
    def_group_fn!(error);
    def_group_fn!(warning);
    def_group_fn!(success);
    def_group_fn!(help);
    // def_group_fn!(health_bar);
}

pub enum Group {
    Error,
    Warning,
    Success,
    Help,
    // HealthBar,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match self {
            Self::Error => "healthError",
            Self::Warning => "healthWarning",
            Self::Success => "healthSuccess",
            Self::Help => "healthHelp",
            // Self::HealthBar => "healthBar",
        }
    }
}
