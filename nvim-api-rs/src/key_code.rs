use neovim_sys::api::vim::{LuaString, Object};
use std::{borrow::Borrow, fmt};

/// :help key-codes
///
#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Nul,
    BS,
    Tab,
    NL,
    FF,
    CR,
    Return,
    Enter,
    Esc,
    Space,
    LT,
    Bslash,
    Bar,
    Del,
    CSI,
    XCSI,
    EOL,
    Ignore,
    NOP,
    Up,
    Down,
    Right,
    Left,
    ShiftUp,
    ShiftDown,
    ShiftRight,
    ShiftLeft,
    ControlLeft,
    ControlRight,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ShiftF1,
    ShiftF2,
    ShiftF3,
    ShiftF4,
    ShiftF5,
    ShiftF6,
    ShiftF7,
    ShiftF8,
    ShiftF9,
    ShiftF10,
    ShiftF11,
    ShiftF12,
    Help,
    Undo,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    KeypadUp,
    KeypadDown,
    KeypadLeft,
    KeypadRight,
    KeypadHome,
    KeypadEnd,
    KeypadOrigin,
    KeypadPageUp,
    KeypadPageDown,
    KeypadDel,
    KeypadPlus,
    KeypadMinus,
    KeypadMultiply,
    KeypadDivide,
    KeypadPoint,
    KeypadComma,
    KeypadEqual,
    KeypadEnter,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Shift(char),
    Control(char),
    Meta(char),
    Alt(char),
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

impl TryFrom<Object> for KeyCode {
    type Error = InvalidKeyCode;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let s = value.as_string_unchecked().to_string_lossy();

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
                    (Some(l), Some(c)) => {
                        return Err(InvalidKeyCode(s.to_string()));
                    }
                    (Some(l), None) => {
                        return Err(InvalidKeyCode(s.to_string()));
                    }
                    (None, Some(c)) => {
                        return Err(InvalidKeyCode(s.to_string()));
                    }
                    (None, None) => {
                        return Err(InvalidKeyCode(s.to_string()));
                    }
                }
            }
        };

        Ok(code)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid key-code: '{}'", _0)]
pub struct InvalidKeyCode(String);

#[allow(clippy::fallible_impl_from)]
impl From<KeyCode> for Object {
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
                return Self::from(LuaString::new(format!("<S-{}>", char)).unwrap());
            }
            KeyCode::Control(char) => {
                return Self::from(LuaString::new(format!("<C-{}>", char)).unwrap());
            }
            KeyCode::Meta(char) => {
                return Self::from(LuaString::new(format!("<M-{}>", char)).unwrap());
            }
            KeyCode::Alt(char) => {
                return Self::from(LuaString::new(format!("<A-{}>", char)).unwrap());
            }
            KeyCode::Super(char) => {
                return Self::from(LuaString::new(format!("<D-{}>", char)).unwrap());
            }
        };

        Self::from(LuaString::new(s).unwrap())
    }
}