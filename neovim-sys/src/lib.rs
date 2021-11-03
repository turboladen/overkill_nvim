#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]
#![allow(non_camel_case_types)]

pub mod api;
pub mod lua;
pub mod types;
// pub mod typval;

// use std::os::raw::c_void;

// extern "C" {
//     // https://github.com/neovim/neovim/blob/684299ed4c9c21f0353ceaec2d1679f956617737/src/nvim/eval/funcs.c#L1154
//     pub fn f_complete(
//         argvars: *const typval::TypvalT,
//         rettv: *mut typval::TypvalT,
//         fptr: FunPtr,
//     ) -> c_void;

//     pub fn f_complete_info(
//         argvars: *const typval::TypvalT,
//         rettv: *mut typval::TypvalT,
//         fptr: FunPtr,
//     ) -> c_void;
// }

// pub type FunPtr = extern "C" fn() -> *const c_void;

// #[must_use]
// pub extern "C" fn no_op_fn_ptr() -> *const c_void {
//     std::ptr::null()
// }

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn init_logger() {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_timestamps(true)
        .init()
        .ok();
}
