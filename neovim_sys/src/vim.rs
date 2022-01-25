//! Types and functions found in `nvim/vim.h`
//!
use std::str::FromStr;

/// Values for states--used for mappings (`0x01` - `0x20`) and other states.
///
/// See `nvim/vim.h`.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// Normal mode; command expected. `:nmap`
    // 1
    Normal = 0x01,

    /// :xmap
    // 2
    Visual = 0x02,

    /// Normal mode, operator is pending. `:omap`
    // 4
    OpPending = 0x04,

    /// Editing command line. `:cmap`
    // 8
    CmdLine = 0x08,

    /// :imap
    // 16
    Insert = 0x10,

    /// :map!
    // 24
    InsertCmdLine = 0x10 + 0x08,

    /// :lmap
    // 32
    LangMap = 0x20,

    // Non-mapping states below here...
    //
    /// Replace mode
    // 80
    Replace = 0x40 + 0x10, // ReplaceFlag + Insert

    /// Lang-replace
    // 96
    LReplace = 0x40 + 0x20, // ReplaceFlag + LangMap

    /// Virtual-replace
    // 304
    VReplace = 0x80 + 0x40 + 0x10, // VReplaceFlag + ReplaceFlag + Insert

    /// Abbreviation instead of mapping
    // 1280
    Abbrev = 0x500,

    /// Executing an external command.
    // 1536
    ExternCmd = 0x600,

    /// Only for mappings; `smap`.
    // 4096
    SelectMode = 0x1000,

    /// :vmap
    // 4098
    VisualSelectMode = 0x02 + 0x1000, // Visual + SelectMode

    /// :map
    // 4103
    NormalVisualSelectOpPending = 0x01 + 0x02 + 0x1000 + 0x04, // Visual + SelectMode

    /// :tmap
    // 8192
    TermFocus = 0x2000,
}

/// Error for if we get a mode string that we don't yet handle properly. When this library is
/// mature, this could/should probably go away, but for now it will serve as a flag to implement
/// things that haven't yet been done.
///
#[derive(Debug, Clone, thiserror::Error)]
#[error("Unknown mode: {mode}")]
pub struct UnexpectedState {
    mode: String,
}

impl FromStr for State {
    type Err = UnexpectedState;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | " " => Ok(Self::NormalVisualSelectOpPending),
            "n" => Ok(Self::Normal),
            "x" => Ok(Self::Visual),
            "o" => Ok(Self::OpPending),
            "c" => Ok(Self::CmdLine),
            "i" => Ok(Self::Insert),
            "!" => Ok(Self::InsertCmdLine),
            "l" => Ok(Self::LangMap),

            "R" => Ok(Self::Replace),
            "v" => Ok(Self::VisualSelectMode),
            "s" => Ok(Self::SelectMode),
            "gR" => Ok(Self::VReplace),
            "t" => Ok(Self::TermFocus),
            m => Err(UnexpectedState {
                mode: m.to_string(),
            }),
        }
    }
}
