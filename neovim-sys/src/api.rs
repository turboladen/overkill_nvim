//!
//! This module really only exists just to mimic neovim's file structre: `neovim/src/api/`
//!
pub mod buffer;
pub mod vim;


/// Mask for all internal calls
// #define INTERNAL_CALL_MASK (((uint64_t)1) << (sizeof(uint64_t) * 8 - 1))
pub(crate) const INTERNAL_CALL_MASK: u64 = 1_u64 << (std::mem::size_of::<u64>() * 8 - 1);

/// Internal call from lua code
// #define LUA_INTERNAL_CALL (VIML_INTERNAL_CALL + 1)
pub const LUA_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK + 1;
