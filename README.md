# overkill-nvim &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![overkill-nvim: rustc 1.56+]][Rust 1.56]

[Build Status]: https://img.shields.io/github/workflow/status/turboladen/neovim-rust/Testing/develop
[actions]: https://github.com/turboladen/neovim-rust/actions?query=branch%3Adevelop
[Latest Version]: https://img.shields.io/crates/v/overkill-nvim.svg
[crates.io]: https://crates.io/crates/overkill-nvim
[overkill-nvim: rustc 1.56+]: https://img.shields.io/badge/overkill_nvim-rustc_1.56+-lightgray.svg
[Rust 1.56]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

**overkill-nvim is a framework for building neovim plugins in Rust (*not* using msgpack!).**

---

There are three crates that make up `overkill-nvim`:

- [neovim-sys](neovim-sys) defines primitive types (ex. `Object`, `Dictionary`, etc) and FFI
  function signatures available in neovim that can be called.
- [nvim-api](nvim-api) wraps `neovim-sys`, providing a Rust API similar to neovim's Lua API.
- [overkill-nvim](overkill-nvim) uses `nvim-api` to provide a (opinionated) framework for writing
  plugins using richer types.

## Quick Comparison

You can use `nvim-api` in a similar fashion to neovim's Lua API. For example, to set the
`signcolumn` option to `"yes:4"`, you could do:

```rust
use nvim_api::nvim;

#[no_mangle]
pub extern "C" fn set_my_options() {
    nvim::nvim_set_option("signcolumn", "yes:4").unwrap();
}
```

(You then need to make this callable in nvim; lua-jit or neovim's `libcall` work. More on this
later.)

...but what if you fat-finger the option name or value? Well, in (neo)vim, you'd get an error
message, telling you about your whoopsie, but since we're in Rust-land, you wouldn't get that error
until you loaded your shared library into neovim and called `set_my_options()`. Sure would be nicer
to know at compile time, yeah?

```rust
use overkill_nvim::option::{StringOption, SignColumn, SignColumnValue};

#[no_mangle]
pub extern "C" fn set_my_options() {
    SignColumn::set(SignColumnValue::YesWithMax(4)).unwrap();
}
```

(Again, this needs to be made callable in nvim)

And there you go--all type-checked at compile time.

Check out [https://github.com/turboladen/init.rs](https://github.com/turboladen/init.rs) for more.

## But why??

Well, for `neovim-sys` and `nvim-api`, the goals are simply to provide a Rust interface to neovim
that's _not_ over msgpack. Nothing against msgpack, but doesn't the idea of calling neovim directly
instead of through that extra layer sound a bit nicer?

As for `overkill-nvim`, well, it's probably overkill, but I really like to have types to tell me
what I can and can't do--`overkill-nvim` give me a typed interface over neovim functionality that
guide me into doing the right thing.

## And how??

Inspiration came from [this post on the neovim
discourse](https://neovim.discourse.group/t/calling-neovim-internal-functions-with-luajit-ffi-and-rust/165/9). The Rust crates are built without the neovim symbols actually available at compile time. You build a shared library using `nvim-api` or `overkill-nvim`, then either:

- Use neovim's `libcall` or `libcallnr` to load the library and call a single function from it,
  or...
- Use lua-jit's `ffi` module for a) defining your FFI symbols, and b) loading the library.
  The specs in [tests/plenary](tests/plenary) do this to be able to test these crates in neovim.

It's quite possible to have your Rust library make all the neovim calls that you need want, and
would thus leave you only needing to define a simple function to call from neovim that kicks off all
of your calls via Rust.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in overkill-nvim by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
