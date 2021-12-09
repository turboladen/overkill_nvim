#![allow(clippy::module_name_repetitions)]

//! Names for keys in mappings and such.
//!
use nvim_api::sys::api::nvim::NvimString;
use std::{borrow::Borrow, fmt};

/// :help key-codes
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCode {
    /// `<Nul>`, zero
    Nul,

    /// `<Bs>`, backspace
    BS,

    /// `<Tab>`, tab
    Tab,

    /// `<NL>`, linefeed
    NL,

    /// `<FF>`, formfeed
    FF,

    /// `<CR>` carriage return
    CR,

    /// `<Return>`, same as `<CR>`
    Return,

    /// `<Enter>`, same as `<CR>`
    Enter,

    /// `<Esc>`, escape
    Esc,

    /// `<Space>`, space
    Space,

    /// `<lt>`, less-than
    LT,

    /// `<Bslash>`, backslash
    Bslash,

    /// `<Bar>`, vertical bar, `|`
    Bar,

    /// `<Del>`, delete
    Del,

    /// `<CSI>`, command sequence intro, ALT-Esc
    CSI,

    /// `<xCSI>`, CSI when typed in the GUI
    XCSI,

    /// `<EOL>`, end-of-line (can be `<CR>`, `<NL>`, or `<CR>NL>`)
    EOL,

    /// `<Ignore>`, cancel wait-for-character
    Ignore,

    /// `<NOP>`, no-op
    NOP,

    /// `<Up>`, cursor-up
    Up,

    /// `<Down>`, cursor-down
    Down,

    /// `<Left>`, cursor-left
    Left,

    /// `<Right>`, cursor-right
    Right,

    /// `<S-Up>`, shift-cursor-up
    ShiftUp,

    /// `<S-Down>`, shift-cursor-down
    ShiftDown,

    /// `<S-Left>`, shift-cursor-left
    ShiftLeft,

    /// `<S-Right>`, shift-cursor-right
    ShiftRight,

    /// `<C-Left>`, control-cursor-left
    ControlLeft,

    /// `<C-Right>`, control-cursor-right
    ControlRight,

    /// `<F1>`
    F1,

    /// `<F2>`
    F2,

    /// `<F3>`
    F3,

    /// `<F4>`
    F4,

    /// `<F5>`
    F5,

    /// `<F6>`
    F6,

    /// `<F7>`
    F7,

    /// `<F8>`
    F8,

    /// `<F9>`
    F9,

    /// `<F10>`
    F10,

    /// `<F11>`
    F11,

    /// `<F12>`
    F12,

    /// `<S-F1>`
    ShiftF1,

    /// `<S-F2>`
    ShiftF2,

    /// `<S-F3>`
    ShiftF3,

    /// `<S-F4>`
    ShiftF4,

    /// `<S-F5>`
    ShiftF5,

    /// `<S-F6>`
    ShiftF6,

    /// `<S-F7>`
    ShiftF7,

    /// `<S-F8>`
    ShiftF8,

    /// `<S-F9>`
    ShiftF9,

    /// `<S-F10>`
    ShiftF10,

    /// `<S-F11>`
    ShiftF11,

    /// `<S-F12>`
    ShiftF12,

    /// `<Help>`, help key
    Help,

    /// `<Undo>`, undo key
    Undo,

    /// `<Insert>`, insert key
    Insert,

    /// `<Home>`, home
    Home,

    /// `<End>`, end
    End,

    /// `<PageUp>`, page-up
    PageUp,

    /// `<PageDown>`, page-down
    PageDown,

    /// `<kUp>`, keypad cursor-up
    KeypadUp,

    /// `<kDown>`, keypad cursor-down
    KeypadDown,

    /// `<kLeft>`, keypad cursor-left
    KeypadLeft,

    /// `<kRight>`, keypad cursor-right
    KeypadRight,

    /// `<kHome>`, keypad home (upper left)
    KeypadHome,

    /// `<kEnd>`, keypad end (lower left)
    KeypadEnd,

    /// `<kOrigin>`, keypad origin (middle)
    KeypadOrigin,

    /// `<kPageUp>`, keypad page-up (upper right)
    KeypadPageUp,

    /// `<kPageDown>`, keypad page-down (lower right)
    KeypadPageDown,

    /// `<kDel>`, keypad delete
    KeypadDel,

    /// `<kPlus>`, keypad +
    KeypadPlus,

    /// `<kMinus>`, keypad -
    KeypadMinus,

    /// `<kMultiply>`, keypad *
    KeypadMultiply,

    /// `<kDivide>`, keypad /
    KeypadDivide,

    /// `<kPoint>`, keypad .
    KeypadPoint,

    /// `<kComma>`, keypad ,
    KeypadComma,

    /// `<kEqual>`, keypad =
    KeypadEqual,

    /// `<kEnter>`, keypad Enter
    KeypadEnter,

    /// `<k0>`, keypad 0
    Keypad0,

    /// `<k1>`, keypad 1
    Keypad1,

    /// `<k2>`, keypad 2
    Keypad2,

    /// `<k3>`, keypad 3
    Keypad3,

    /// `<k4>`, keypad 4
    Keypad4,

    /// `<k5>`, keypad 5
    Keypad5,

    /// `<k6>`, keypad 6
    Keypad6,

    /// `<k7>`, keypad 7
    Keypad7,

    /// `<k8>`, keypad 8
    Keypad8,

    /// `<k9>`, keypad 9
    Keypad9,

    /// `<S-…>`, shift-key
    Shift(char),

    /// `<C-…>`, control-key
    Control(char),

    /// `<M-…>`, alt-key or meta-key
    Meta(char),

    /// `<A-…>`, same as `<M-…>`
    Alt(char),

    /// `<D-…>`, command-key or "super" key
    Super(char),
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Nul => "<Nul>",
            Self::BS => "<BS>",
            Self::Tab => "<Tab>",
            Self::NL => "<NL>",
            Self::FF => "<FF>",
            Self::CR => "<CR>",
            Self::Return => "<Return>",
            Self::Enter => "<Enter>",
            Self::Esc => "<Esc>",
            Self::Space => "<Space>",
            Self::LT => "<lt>",
            Self::Bslash => "<Bslash>",
            Self::Bar => "<Bar>",
            Self::Del => "<Del>",
            Self::CSI => "<CSI>",
            Self::XCSI => "<xCSI>",
            Self::EOL => "<EOL>",
            Self::Ignore => "<Ignore>",
            Self::NOP => "<NOP>",
            Self::Up => "<Up>",
            Self::Down => "<Down>",
            Self::Right => "<Right>",
            Self::Left => "<Left>",
            Self::ShiftUp => "<S-Up>",
            Self::ShiftDown => "<S-Down>",
            Self::ShiftRight => "<S-Right>",
            Self::ShiftLeft => "<S-Left>",
            Self::ControlLeft => "<C-Left>",
            Self::ControlRight => "<C-Right>",

            Self::F1 => "<F1>",
            Self::F2 => "<F2>",
            Self::F3 => "<F3>",
            Self::F4 => "<F4>",
            Self::F5 => "<F5>",
            Self::F6 => "<F6>",
            Self::F7 => "<F7>",
            Self::F8 => "<F8>",
            Self::F9 => "<F9>",
            Self::F10 => "<F10>",
            Self::F11 => "<F11>",
            Self::F12 => "<F12>",

            Self::ShiftF1 => "<S-F1>",
            Self::ShiftF2 => "<S-F2>",
            Self::ShiftF3 => "<S-F3>",
            Self::ShiftF4 => "<S-F4>",
            Self::ShiftF5 => "<S-F5>",
            Self::ShiftF6 => "<S-F6>",
            Self::ShiftF7 => "<S-F7>",
            Self::ShiftF8 => "<S-F8>",
            Self::ShiftF9 => "<S-F9>",
            Self::ShiftF10 => "<S-F10>",
            Self::ShiftF11 => "<S-F11>",
            Self::ShiftF12 => "<S-F12>",

            Self::Help => "<Help>",
            Self::Undo => "<Undo>",
            Self::Insert => "<Insert>",
            Self::Home => "<Home>",
            Self::End => "<End>",
            Self::PageUp => "<PageUp>",
            Self::PageDown => "<PageDown>",

            Self::KeypadUp => "<kUp>",
            Self::KeypadDown => "<kDown>",
            Self::KeypadLeft => "<kLeft>",
            Self::KeypadRight => "<kRight>",
            Self::KeypadHome => "<kHome>",
            Self::KeypadEnd => "<kEnd>",
            Self::KeypadOrigin => "<kOrigin>",
            Self::KeypadPageUp => "<kPageUp>",
            Self::KeypadPageDown => "<kPageDown>",
            Self::KeypadDel => "<kDel>",
            Self::KeypadPlus => "<kPlus>",
            Self::KeypadMinus => "<kMinus>",
            Self::KeypadMultiply => "<kMultiply>",
            Self::KeypadDivide => "<kDivide>",
            Self::KeypadPoint => "<kPoint>",
            Self::KeypadComma => "<kComma>",
            Self::KeypadEqual => "<kEqual>",
            Self::KeypadEnter => "<kEnter>",

            Self::Keypad0 => "<k0>",
            Self::Keypad1 => "<k1>",
            Self::Keypad2 => "<k2>",
            Self::Keypad3 => "<k3>",
            Self::Keypad4 => "<k4>",
            Self::Keypad5 => "<k5>",
            Self::Keypad6 => "<k6>",
            Self::Keypad7 => "<k7>",
            Self::Keypad8 => "<k8>",
            Self::Keypad9 => "<k9>",
            Self::Shift(char) => return write!(f, "<S-{}>", char),
            Self::Control(char) => return write!(f, "<C-{}>", char),
            Self::Meta(char) => return write!(f, "<M-{}>", char),
            Self::Alt(char) => return write!(f, "<A-{}>", char),
            Self::Super(char) => return write!(f, "<D-{}>", char),
        };

        f.write_str(s)
    }
}

impl TryFrom<NvimString> for KeyCode {
    type Error = InvalidKeyCode;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: NvimString) -> Result<Self, Self::Error> {
        let s = value.to_string_lossy();

        let code = match s.borrow() {
            "<Nul>" => Self::Nul,
            "<BS>" => Self::BS,
            "<Tab>" => Self::Tab,
            "<NL>" => Self::NL,
            "<FF>" => Self::FF,
            "<CR>" => Self::CR,
            "<Return>" => Self::Return,
            "<Enter>" => Self::Enter,
            "<Esc>" => Self::Esc,
            "<Space>" => Self::Space,
            "<lt>" => Self::LT,
            "<Bslash>" => Self::Bslash,
            "<Bar>" => Self::Bar,
            "<Del>" => Self::Del,
            "<CSI>" => Self::CSI,
            "<xCSI>" => Self::XCSI,
            "<EOL>" => Self::EOL,
            "<Ignore>" => Self::Ignore,
            "<NOP>" => Self::NOP,

            "<Up>" => Self::Up,
            "<Down>" => Self::Down,
            "<Right>" => Self::Right,
            "<Left>" => Self::Left,
            "<S-Up>" => Self::ShiftUp,
            "<S-Down>" => Self::ShiftDown,
            "<S-Right>" => Self::ShiftRight,
            "<S-Left>" => Self::ShiftLeft,
            "<C-Left>" => Self::ControlLeft,
            "<C-Right>" => Self::ControlRight,

            "<F1>" => Self::F1,
            "<F2>" => Self::F2,
            "<F3>" => Self::F3,
            "<F4>" => Self::F4,
            "<F5>" => Self::F5,
            "<F6>" => Self::F6,
            "<F7>" => Self::F7,
            "<F8>" => Self::F8,
            "<F9>" => Self::F9,
            "<F10>" => Self::F10,
            "<F11>" => Self::F11,
            "<F12>" => Self::F12,
            "<S-F1>" => Self::ShiftF1,
            "<S-F2>" => Self::ShiftF2,
            "<S-F3>" => Self::ShiftF3,
            "<S-F4>" => Self::ShiftF4,
            "<S-F5>" => Self::ShiftF5,
            "<S-F6>" => Self::ShiftF6,
            "<S-F7>" => Self::ShiftF7,
            "<S-F8>" => Self::ShiftF8,
            "<S-F9>" => Self::ShiftF9,
            "<S-F10>" => Self::ShiftF10,
            "<S-F11>" => Self::ShiftF11,
            "<S-F12>" => Self::ShiftF12,

            "<Help>" => Self::Help,
            "<Undo>" => Self::Undo,
            "<Insert>" => Self::Insert,
            "<Home>" => Self::Home,
            "<End>" => Self::End,
            "<PageUp>" => Self::PageUp,
            "<PageDown>" => Self::PageDown,

            "<kUp>" => Self::KeypadUp,
            "<kDown>" => Self::KeypadDown,
            "<kLeft>" => Self::KeypadLeft,
            "<kRight>" => Self::KeypadRight,
            "<kHome>" => Self::KeypadHome,
            "<kEnd>" => Self::KeypadEnd,
            "<kOrigin>" => Self::KeypadOrigin,
            "<kPageUp>" => Self::KeypadPageUp,
            "<kPageDown>" => Self::KeypadPageDown,
            "<kDel>" => Self::KeypadDel,
            "<kPlus>" => Self::KeypadPlus,
            "<kMinus>" => Self::KeypadMinus,
            "<kMultiply>" => Self::KeypadMultiply,
            "<kDivide>" => Self::KeypadDivide,
            "<kPoint>" => Self::KeypadPoint,
            "<kComma>" => Self::KeypadComma,
            "<kEqual>" => Self::KeypadEqual,
            "<kEnter>" => Self::KeypadEnter,

            "<k0>" => Self::Keypad0,
            "<k1>" => Self::Keypad1,
            "<k2>" => Self::Keypad2,
            "<k3>" => Self::Keypad3,
            "<k4>" => Self::Keypad4,
            "<k5>" => Self::Keypad5,
            "<k6>" => Self::Keypad6,
            "<k7>" => Self::Keypad7,
            "<k8>" => Self::Keypad8,
            "<k9>" => Self::Keypad9,
            _ => {
                debug_assert!(s.starts_with('<'));
                debug_assert!(s.ends_with('>'));

                let mut split = s.trim_start().trim_end().split('-');
                let lhs = split.next();
                let rhs = split.next().and_then(|c| c.chars().next());

                match (lhs, rhs) {
                    (Some("S"), Some(c)) => Self::Shift(c),
                    (Some("C"), Some(c)) => Self::Control(c),
                    (Some("M"), Some(c)) => Self::Meta(c),
                    (Some("A"), Some(c)) => Self::Alt(c),
                    (Some("D"), Some(c)) => Self::Super(c),
                    (Some(_) | None, Some(_) | None) => {
                        return Err(InvalidKeyCode(s.to_string()));
                    }
                }
            }
        };

        Ok(code)
    }
}

/// Error if an unexpected key-code char was received from nvim.
///
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid key-code: '{}'", _0)]
pub struct InvalidKeyCode(String);

#[allow(clippy::fallible_impl_from)]
impl From<KeyCode> for NvimString {
    #[allow(clippy::too_many_lines)]
    fn from(key_code: KeyCode) -> Self {
        let s = match key_code {
            KeyCode::Nul => "<Nul>",
            KeyCode::BS => "<BS>",
            KeyCode::Tab => "<Tab>",
            KeyCode::NL => "<NL>",
            KeyCode::FF => "<FF>",
            KeyCode::CR => "<CR>",
            KeyCode::Return => "<Return>",
            KeyCode::Enter => "<Enter>",
            KeyCode::Esc => "<Esc>",
            KeyCode::Space => "<Space>",
            KeyCode::LT => "<lt>",
            KeyCode::Bslash => "<Bslash>",
            KeyCode::Bar => "<Bar>",
            KeyCode::Del => "<Del>",
            KeyCode::CSI => "<CSI>",
            KeyCode::XCSI => "<xCSI>",
            KeyCode::EOL => "<EOL>",
            KeyCode::Ignore => "<Ignore>",
            KeyCode::NOP => "<NOP>",
            KeyCode::Up => "<Up>",
            KeyCode::Down => "<Down>",
            KeyCode::Right => "<Right>",
            KeyCode::Left => "<Left>",
            KeyCode::ShiftUp => "<S-Up>",
            KeyCode::ShiftDown => "<S-Down>",
            KeyCode::ShiftRight => "<S-Right>",
            KeyCode::ShiftLeft => "<S-Left>",
            KeyCode::ControlLeft => "<C-Left>",
            KeyCode::ControlRight => "<C-Right>",

            KeyCode::F1 => "<F1>",
            KeyCode::F2 => "<F2>",
            KeyCode::F3 => "<F3>",
            KeyCode::F4 => "<F4>",
            KeyCode::F5 => "<F5>",
            KeyCode::F6 => "<F6>",
            KeyCode::F7 => "<F7>",
            KeyCode::F8 => "<F8>",
            KeyCode::F9 => "<F9>",
            KeyCode::F10 => "<F10>",
            KeyCode::F11 => "<F11>",
            KeyCode::F12 => "<F12>",

            KeyCode::ShiftF1 => "<S-F1>",
            KeyCode::ShiftF2 => "<S-F2>",
            KeyCode::ShiftF3 => "<S-F3>",
            KeyCode::ShiftF4 => "<S-F4>",
            KeyCode::ShiftF5 => "<S-F5>",
            KeyCode::ShiftF6 => "<S-F6>",
            KeyCode::ShiftF7 => "<S-F7>",
            KeyCode::ShiftF8 => "<S-F8>",
            KeyCode::ShiftF9 => "<S-F9>",
            KeyCode::ShiftF10 => "<S-F10>",
            KeyCode::ShiftF11 => "<S-F11>",
            KeyCode::ShiftF12 => "<S-F12>",

            KeyCode::Help => "<Help>",
            KeyCode::Undo => "<Undo>",
            KeyCode::Insert => "<Insert>",
            KeyCode::Home => "<Home>",
            KeyCode::End => "<End>",
            KeyCode::PageUp => "<PageUp>",
            KeyCode::PageDown => "<PageDown>",

            KeyCode::KeypadUp => "<kUp>",
            KeyCode::KeypadDown => "<kDown>",
            KeyCode::KeypadLeft => "<kLeft>",
            KeyCode::KeypadRight => "<kRight>",
            KeyCode::KeypadHome => "<kHome>",
            KeyCode::KeypadEnd => "<kEnd>",
            KeyCode::KeypadOrigin => "<kOrigin>",
            KeyCode::KeypadPageUp => "<kPageUp>",
            KeyCode::KeypadPageDown => "<kPageDown>",
            KeyCode::KeypadDel => "<kDel>",
            KeyCode::KeypadPlus => "<kPlus>",
            KeyCode::KeypadMinus => "<kMinus>",
            KeyCode::KeypadMultiply => "<kMultiply>",
            KeyCode::KeypadDivide => "<kDivide>",
            KeyCode::KeypadPoint => "<kPoint>",
            KeyCode::KeypadComma => "<kComma>",
            KeyCode::KeypadEqual => "<kEqual>",
            KeyCode::KeypadEnter => "<kEnter>",

            KeyCode::Keypad0 => "<k0>",
            KeyCode::Keypad1 => "<k1>",
            KeyCode::Keypad2 => "<k2>",
            KeyCode::Keypad3 => "<k3>",
            KeyCode::Keypad4 => "<k4>",
            KeyCode::Keypad5 => "<k5>",
            KeyCode::Keypad6 => "<k6>",
            KeyCode::Keypad7 => "<k7>",
            KeyCode::Keypad8 => "<k8>",
            KeyCode::Keypad9 => "<k9>",
            KeyCode::Shift(char) => {
                return Self::new(format!("<S-{}>", char)).unwrap();
            }
            KeyCode::Control(char) => {
                return Self::new(format!("<C-{}>", char)).unwrap();
            }
            KeyCode::Meta(char) => {
                return Self::new(format!("<M-{}>", char)).unwrap();
            }
            KeyCode::Alt(char) => {
                return Self::new(format!("<A-{}>", char)).unwrap();
            }
            KeyCode::Super(char) => {
                return Self::new(format!("<D-{}>", char)).unwrap();
            }
        };

        Self::new_unchecked(s)
    }
}
