test-all:
  cargo test
  cargo build --features lua_test
  nvim --headless -c "PlenaryBustedDirectory tests/plenary {minimal_init = 'tests/minimal_init.vim'}"
