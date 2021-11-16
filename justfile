default: test-rust test-lua

test-rust:
  cargo test --workspace

test-miri:
  cargo +nightly miri test --workspace

test-lua:
  cargo build --features lua_test
  nvim --headless -c "PlenaryBustedDirectory tests/plenary {minimal_init = 'tests/minimal_init.vim'}"
