//! Types defined in `globals.h`.
//!

/// Special values for `current_SID`.
///
#[derive(Debug, Clone, Copy)]
pub enum Sid {
    /// When using a modeline.
    ///
    Modeline = -1,

    /// For `--cmd` argument.
    ///
    CmdArg = -2,

    /// For `-c` argument.
    ///
    CArg = -3,

    /// For sourcing an environment variable.
    ///
    Env = -4,

    /// Option was reset because of an error.
    ///
    Error = -5,

    /// Don't set the scriptID.
    ///
    None = -6,

    /// Changing window size.
    ///
    WinLayout = -7,

    /// For lua scripts/chunks.
    ///
    Lua = -8,

    /// For API clients.
    ///
    ApiClient = -9,

    /// For sourcing a string.
    ///
    Str = -10,
}
