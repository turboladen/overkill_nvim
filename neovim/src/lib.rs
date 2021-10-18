pub mod api;
pub mod eval;
pub mod typval;

pub use neovim_sys as sys;

#[no_mangle]
pub extern "C" fn rs_nvim_get_current_buf() -> sys::api::buffer::Buffer {
    self::api::nvim_get_current_buf()
}
