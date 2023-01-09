use crate::{AsGroupName, ToHighlightCommand, ToLinkCommand};

/// https://neovim.io/doc/user/diagnostic.html#diagnostic-highlights
///
pub trait Highlighting {
    def_group_fn!(error);
    def_group_fn!(warn);
    def_group_fn!(info);
    def_group_fn!(hint);

    def_group_fn!(virtual_text_error);
    def_group_fn!(virtual_text_warn);
    def_group_fn!(virtual_text_info);
    def_group_fn!(virtual_text_hint);

    def_group_fn!(underline_error);
    def_group_fn!(underline_warn);
    def_group_fn!(underline_info);
    def_group_fn!(underline_hint);

    def_group_fn!(floating_error);
    def_group_fn!(floating_warn);
    def_group_fn!(floating_info);
    def_group_fn!(floating_hint);

    def_group_fn!(sign_error);
    def_group_fn!(sign_warn);
    def_group_fn!(sign_info);
    def_group_fn!(sign_hint);
}

#[derive(Clone, Copy, Debug)]
pub enum Group {
    Error,
    Warn,
    Info,
    Hint,

    VirtualTextError,
    VirtualTextWarn,
    VirtualTextInfo,
    VirtualTextHint,

    UnderlineError,
    UnderlineWarn,
    UnderlineInfo,
    UnderlineHint,

    FloatingError,
    FloatingWarn,
    FloatingInfo,
    FloatingHint,

    SignError,
    SignWarn,
    SignInfo,
    SignHint,
}

impl AsGroupName for Group {
    fn as_group_name(&self) -> &'static str {
        match *self {
            Self::Error => "DiagnosticError",
            Self::Warn => "DiagnosticWarn",
            Self::Info => "DiagnosticInfo",
            Self::Hint => "DiagnosticHint",
            Self::VirtualTextError => "DiagnosticVirtualTextError",
            Self::VirtualTextWarn => "DiagnosticVirtualTextWarn",
            Self::VirtualTextInfo => "DiagnosticVirtualTextInfo",
            Self::VirtualTextHint => "DiagnosticVirtualTextHint",
            Self::UnderlineError => "DiagnosticUnderlineError",
            Self::UnderlineWarn => "DiagnosticUnderlineWarn",
            Self::UnderlineInfo => "DiagnosticUnderlineInfo",
            Self::UnderlineHint => "DiagnosticUnderlineHint",
            Self::FloatingError => "DiagnosticFloatingError",
            Self::FloatingWarn => "DiagnosticFloatingWarn",
            Self::FloatingInfo => "DiagnosticFloatingInfo",
            Self::FloatingHint => "DiagnosticFloatingHint",
            Self::SignError => "DiagnosticSignError",
            Self::SignWarn => "DiagnosticSignWarn",
            Self::SignInfo => "DiagnosticSignInfo",
            Self::SignHint => "DiagnosticSignHint",
        }
    }
}

impl ToHighlightCommand for Group {}
impl ToLinkCommand for Group {}
