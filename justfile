default: test-rust test-lua

test-rust:
  cargo test --workspace

test-miri:
  cargo +nightly miri test --workspace

test-lua:
  cargo build --features lua_test --package nvim-api --package overkill-nvim
  nvim --headless -n -c "PlenaryBustedDirectory tests/plenary {minimal_init = 'tests/minimal_init.vim'}"

fix-clippies:
  cargo clippy --all-features --tests --fix

fix-formatting:
  cargo +nightly fmt

fix-all-lints: fix-clippies fix-formatting

# vim:ft=just
