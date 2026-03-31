use core::fmt;

/// Porting the example in `:h augroup`...
///
/// ```
/// use overkill_nvim::autocmd::{augroup, autocmd, Event};
///
/// augroup("uncompress", |g| {
///     g.remove_all_autocmds();
///     g.define_autocmds(vec![Event::BufEnter], vec!["*.gz"], "%!gunzip");
/// })
/// ```
///
pub fn augroup<'a, F>(name: &'a str, autocmds: F)
where
    F: Fn(GroupedAutocmdBuilder<'a>),
{
    nvim_api::autocmd::augroup(name).unwrap();
    autocmds(GroupedAutocmdBuilder::new(name));
    nvim_api::autocmd::augroup("END").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct GroupedAutocmdBuilder<'a> {
    group_name: &'a str,
}

impl<'a> GroupedAutocmdBuilder<'a> {
    pub fn new(group_name: &'a str) -> Self {
        Self { group_name }
    }

    pub fn autocmd(&self, events: Vec<Event>, patterns: Vec<&str>, cmd: &str) {
        let mut au = events
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<_>>()
            .join(",");
        au += " ";
        au += &patterns.join(",");
        au += " ";
        au += cmd;

        nvim_api::autocmd::autocmd(&au).unwrap();
    }

    pub fn remove_all_autocmds(&self) {
        nvim_api::autocmd::remove_autocmd(self.group_name).unwrap();
    }

    pub fn remove_autocmds(&self, events: Vec<Event>, patterns: Vec<&str>, cmd: &str) {
        let au = format!(
            "{} {} {} {}",
            self.group_name,
            events
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<_>>()
                .join(","),
            &patterns.join(","),
            cmd
        );

        nvim_api::autocmd::remove_autocmd(&au).unwrap();
    }
}

pub trait Cmd {
    fn cmd_string(&self, cmd: &str) -> String;
    fn cmd(&self, cmd: &str) {
        nvim_api::autocmd::autocmd(&self.cmd_string(cmd)).unwrap();
    }
}

pub struct Autocmd<'a> {
    events: &'a [Event],
    patterns: &'a [&'a str],
    once: bool,
    nested: bool,
}

impl<'a> Autocmd<'a> {
    pub const fn new(events: &'a [Event], patterns: &'a [&'a str]) -> Self {
        Self {
            events,
            patterns,
            once: false,
            nested: false,
        }
    }

    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }

    pub fn nested(mut self) -> Self {
        self.nested = true;
        self
    }
}

impl<'a> Cmd for Autocmd<'a> {
    fn cmd_string(&self, cmd: &str) -> String {
        let mut start = format!(
            "{} {}",
            self.events
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<_>>()
                .join(","),
            self.patterns.join(","),
        );

        if self.once {
            start += " ++once"
        };

        if self.nested {
            start += " ++nested"
        };

        start += " ";

        start + cmd
    }
}

pub struct BufLocalAutocmd<'a> {
    buffer: Buffer,
    autocmd: Autocmd<'a>,
}

impl<'a> BufLocalAutocmd<'a> {
    pub fn new(buffer: Buffer, events: &'a [Event], patterns: &'a [&'a str]) -> Self {
        Self {
            buffer,
            autocmd: Autocmd::new(events, patterns),
        }
    }

    pub fn once(mut self) -> Self {
        self.autocmd.once = true;
        self
    }

    pub fn nested(mut self) -> Self {
        self.autocmd.nested = true;
        self
    }
}

impl<'a> Cmd for BufLocalAutocmd<'a> {
    fn cmd_string(&self, cmd: &str) -> String {
        let mut start = format!(
            "{} {}",
            self.autocmd
                .events
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<_>>()
                .join(","),
            self.autocmd.patterns.join(","),
        );

        if self.autocmd.once {
            start += " ++once"
        };

        if self.autocmd.nested {
            start += " ++nested"
        };

        start += " ";
        start += &self.buffer.to_string();

        start + cmd
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Buffer {
    Current,
    Number(u32),
    ABuf,
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Current => f.write_str("<buffer>"),
            Self::Number(n) => write!(f, "<buffer={}>", n),
            Self::ABuf => f.write_str("<buffer=abuf>"),
        }
    }
}

pub fn autocmd(events: Vec<Event>, patterns: Vec<&str>, cmd: &str) {
    let au = format!(
        "{} {} {}",
        events
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<_>>()
            .join(","),
        &patterns.join(","),
        cmd
    );

    nvim_api::autocmd::autocmd(&au).unwrap();
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum Event {
    BufAdd,
    BufDelete,
    BufEnter,
    BufFilePost,
    BufFilePre,
    BufHidden,
    BufLeave,
    BufModifiedSet,
    BufNew,
    BufNewFile,
    BufRead,
    BufReadCmd,
    BufReadPost,
    BufReadPre,
    BufUnload,
    BufWinEnter,
    BufWinLeave,
    BufWipeout,
    BufWrite,
    BufWritePre,
    BufWriteCmd,
    BufWritePost,

    ChanInfo,
    ChanOpen,
    CmdUndefined,

    CmdlineChanged,
    CmdlineEnter,
    CmdlineLeave,

    CmdwinEnter,
    CmdwinLeave,

    ColorScheme,
    ColorSchemePre,

    CompleteChanged,
    CompleteDonePre,
    CompleteDone,

    CursorHold,
    CursorHoldI,
    CursorMoved,
    CursorMovedI,

    DiffUpdated,

    DirChanged,

    FileAppendCmd,
    FileAppendPost,
    FileAppendPre,
    FileChangedRO,

    ExitPre,

    FileChangedShell,
    FileChangedShellPost,
    FileReadCmd,
    FileReadPost,
    FileReadPre,

    FileType,

    FileWriteCmd,
    FileWritePost,
    FileWritePre,

    FocusGained,
    FocusLost,

    FuncUndefined,
    UIEnter,
    UILeave,

    InsertChange,
    InsertCharPre,
    InsertEnter,
    InsertLeavePre,
    InsertLeave,

    MenuPopup,

    OptionSet,

    QuickFixCmdPre,
    QuickFixCmdPost,
    QuitPre,

    RemoteReply,

    SessionLoadPost,
    ShellCmdPost,
    Signal,
    ShellFilterPost,

    SourcePre,
    SourcePost,
    SourceCmd,

    SpellFileMissing,
    StdinReadPost,
    StdinReadPre,

    SwapExists,
    Syntax,

    TabEnter,
    TabLeave,
    TabNew,
    TabNewEntered,
    TabClosed,

    TermOption,
    TermEnter,
    TermLeave,
    TermClose,
    TermResponse,

    TextChanged,
    TextChangedI,
    TextChangedP,
    TextYankPost,

    User,

    VimEnter,
    VimLeave,
    VimLeavePre,
    VimResized,
    VimResume,
    VimSuspend,

    WinClosed,
    WinEnter,
    WinLeave,
    WinNew,
    WinScrolled,
}

impl Event {
    pub fn as_str(&self) -> &'static str {
        match self {
            Event::BufAdd => "BufAdd",
            Event::BufDelete => "BufDelete",
            Event::BufEnter => "BufEnter",
            Event::BufFilePost => "BufFilePost",
            Event::BufFilePre => "BufFilePre",
            Event::BufHidden => "BufHidden",
            Event::BufLeave => "BufLeave",
            Event::BufModifiedSet => "BufModifiedSet",
            Event::BufNew => "BufNew",
            Event::BufNewFile => "BufNewFile",
            Event::BufRead => "BufRead",
            Event::BufReadCmd => "BufReadCmd",
            Event::BufReadPost => "BufReadPost",
            Event::BufReadPre => "BufReadPre",
            Event::BufUnload => "BufUnload",
            Event::BufWinEnter => "BufWinEnter",
            Event::BufWinLeave => "BufWinLeave",
            Event::BufWipeout => "BufWipeout",
            Event::BufWrite => "BufWrite",
            Event::BufWritePre => "BufWritePre",
            Event::BufWriteCmd => "BufWriteCmd",
            Event::BufWritePost => "BufWritePost",
            Event::ChanInfo => "ChanInfo",
            Event::ChanOpen => "ChanOpen",
            Event::CmdUndefined => "CmdUndefined",
            Event::CmdlineChanged => "CmdlineChanged",
            Event::CmdlineEnter => "CmdlineEnter",
            Event::CmdlineLeave => "CmdlineLeave",
            Event::CmdwinEnter => "CmdwinEnter",
            Event::CmdwinLeave => "CmdwinLeave",
            Event::ColorScheme => "ColorScheme",
            Event::ColorSchemePre => "ColorSchemePre",
            Event::CompleteChanged => "CompleteChanged",
            Event::CompleteDonePre => "CompleteDonePre",
            Event::CompleteDone => "CompleteDone",
            Event::CursorHold => "CursorHold",
            Event::CursorHoldI => "CursorHoldI",
            Event::CursorMoved => "CursorMoved",
            Event::CursorMovedI => "CursorMovedI",
            Event::DiffUpdated => "DiffUpdated",
            Event::DirChanged => "DirChanged",
            Event::FileAppendCmd => "FileAppendCmd",
            Event::FileAppendPost => "FileAppendPost",
            Event::FileAppendPre => "FielAppendPre",
            Event::FileChangedRO => "FileChangedRO",
            Event::ExitPre => "ExitPre",
            Event::FileChangedShell => "FileChangedShell",
            Event::FileChangedShellPost => "FileChangedShellPost",
            Event::FileReadCmd => "FileReadCmd",
            Event::FileReadPost => "FileReadPost",
            Event::FileReadPre => "FileReadPre",
            Event::FileType => "FileType",
            Event::FileWriteCmd => "FileWriteCmd",
            Event::FileWritePost => "FileWritePost",
            Event::FileWritePre => "FileWritePre",
            Event::FocusGained => "FocusGained",
            Event::FocusLost => "FocusLost",
            Event::FuncUndefined => "FuncUndefined",
            Event::UIEnter => "UIEnter",
            Event::UILeave => "UILeave",
            Event::InsertChange => "InsertChange",
            Event::InsertCharPre => "InsertCharPre",
            Event::InsertEnter => "InsertEnter",
            Event::InsertLeavePre => "InsertLeavePre",
            Event::InsertLeave => "InsertLeave",
            Event::MenuPopup => "MenuPopup",
            Event::OptionSet => "OptionSet",
            Event::QuickFixCmdPre => "QuickFixCmdPre",
            Event::QuickFixCmdPost => "QuickFixCmdPost",
            Event::QuitPre => "QuitPre",
            Event::RemoteReply => "RemoveReply",
            Event::SessionLoadPost => "SessionLoadPost",
            Event::ShellCmdPost => "ShellCmdPost",
            Event::Signal => "Signal",
            Event::ShellFilterPost => "ShellFilterPost",
            Event::SourcePre => "SourcePre",
            Event::SourcePost => "SourcePost",
            Event::SourceCmd => "SourceCmd",
            Event::SpellFileMissing => "SpellFileMissing",
            Event::StdinReadPost => "StdinReadPost",
            Event::StdinReadPre => "StdinReadPre",
            Event::SwapExists => "SwapExists",
            Event::Syntax => "Syntax",
            Event::TabEnter => "TabEnter",
            Event::TabLeave => "TabLeave",
            Event::TabNew => "TabNew",
            Event::TabNewEntered => "TabNewEntered",
            Event::TabClosed => "TabClosed",
            Event::TermOption => "TermOption",
            Event::TermEnter => "TermEnter",
            Event::TermLeave => "TermLeave",
            Event::TermClose => "TermClose",
            Event::TermResponse => "TermReponse",
            Event::TextChanged => "TextChanged",
            Event::TextChangedI => "TextChangedI",
            Event::TextChangedP => "TextChangedP",
            Event::TextYankPost => "TextYankPost",
            Event::User => "User",
            Event::VimEnter => "VimEnter",
            Event::VimLeave => "VimLeave",
            Event::VimLeavePre => "VimLeavePre",
            Event::VimResized => "VimResized",
            Event::VimResume => "VimResume",
            Event::VimSuspend => "VimSuspend",
            Event::WinClosed => "WinClosed",
            Event::WinEnter => "WinEnter",
            Event::WinLeave => "WinLeave",
            Event::WinNew => "WinNew",
            Event::WinScrolled => "WinScrolled",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        Autocmd::new(&[Event::BufEnter], &["*.rs"]).cmd("echo 'hi'<CR>");
        Autocmd::new(&[Event::BufEnter], &["*.rs"])
            .once()
            .cmd("echo 'hi'<CR>");
        Autocmd::new(&[Event::BufEnter], &["*.rs"])
            .nested()
            .cmd("echo 'hi'<CR>");
        Autocmd::new(&[Event::BufEnter], &["*.rs"])
            .once()
            .nested()
            .cmd("echo 'hi'<CR>");
        BufLocalAutocmd::new(Buffer::Current, &[Event::BufEnter], &["*.rs"])
            .once()
            .nested()
            .cmd("echo 'hi'<CR>");
        BufLocalAutocmd::new(Buffer::Number(24), &[Event::BufEnter], &["*.rs"])
            .cmd("echo 'hi'<CR>");
        BufLocalAutocmd::new(Buffer::ABuf, &[Event::BufEnter], &["*.rs"])
            .cmd("echo 'hi'<CR>");
    }
}
