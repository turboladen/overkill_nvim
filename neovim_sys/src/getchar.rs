//! Types and functions related to those in nvim/getchar.c.
//!
use crate::{buffer_defs::buf_T, types::CharU};
use std::os::raw::c_int;

/// Flag, specifically for `do_map()` and `buf_do_map()`.
///
#[derive(Debug, Clone, Copy)]
pub enum MapType {
    /// :map
    Map = 0,

    /// :unmap
    Unmap = 1,

    /// :noremap
    NoRemap = 2,
}

/// The mode for which the key-mapping should be defined.
///
/// See `nvim/vim.h`.
///
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Normal mode; command expected. `:nmap`
    Normal = 0x01,

    /// :xmap
    Visual = 0x02,

    /// Normal mode, operator is pending. `:omap`
    OpPending = 0x04,

    /// Editing command line. `:cmap`
    CmdLine = 0x08,

    /// :imap
    Insert = 0x10,

    /// :lmap
    LangMap = 0x20,

    /// Replace mode
    Replace = 0x40 + 0x10, // ReplaceFlag + Insert

    /// Virtual-replace
    VReplace = 0x80 + 0x40 + 0x10, // VReplaceFlag + ReplaceFlag + Insert

    /// Lang-replace
    LReplace = 0x40 + 0x20, // ReplaceFlag + LangMap

    /// :vmap
    VisualSelectMode = 0x02 + 0x1000, // Visual + SelectMode

    /// Abbreviation instead of mapping
    Abbrev = 0x500,

    /// Executing an external command.
    ExternCmd = 0x600,

    /// Only for mappings; `smap`.
    SelectMode = 0x1000,

    /// :tmap
    TermFocus = 0x2000,
}

/// Some vim map-related calls require the mapping arguments (that were provided via a string) be
/// parsed into a struct--this is that struct. See `str_to_mapargs()`.
///
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct MapArguments {
    _inner: [u8; 0],
}

impl MapArguments {
    /// An empty struct that must be passed to a nvim function to be initialized.
    ///
    #[must_use]
    pub const fn new() -> Self {
        Self { _inner: [] }
    }
}

impl Default for MapArguments {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" {
    /// `map_type` is one of `MapType`.
    /// `arg` is the argument to the mapping (this C-string will be modified).
    /// `mode` is `Mode`.
    /// `is_abbrev` should be `true` if setting an abbreviation. (:h abbreviations).
    /// Return values can be:
    /// - 0: success
    /// - 1: invalid arguments
    /// - 2: no matches
    /// - 5: entry not unique
    ///
    pub fn do_map(map_type: c_int, arg: *mut CharU, mode: c_int, is_abbrev: bool) -> c_int;

    /// * `map_type` (see `do_map()`).
    /// * `mapargs` is the map options, parsed into a `MapArguments` using `str_to_mapargs()`.
    /// * `mode` (see `do_map()`).
    /// * `is_abbrev` (see `do_map()`).
    /// * `buf` is a handle to the buffer, obtained from passing the buffer number to
    ///   `find_buffer_by_handle()`.
    ///
    pub fn buf_do_map(
        map_type: c_int,
        mapargs: *const MapArguments,
        mode: c_int,
        is_abbrev: bool,
        buf: *const buf_T,
    ) -> c_int;

    /// `args` is the c-string of args in the map (the rhs).
    /// `is_unmap` parses `args` as if you're unmapping; ex. a `" "` is treated as `<space>`; a
    /// `"<space>"` is treated literally as the string `"<space>"` (it's not parsed to mean the space
    /// char).
    /// `mapargs` is an out param, populated with the parsed `args`.
    ///
    /// Returns:
    /// - 0: success
    /// - 1: invalid arguments
    pub fn str_to_mapargs(args: *const CharU, is_unmap: bool, mapargs: *mut MapArguments) -> c_int;
}
