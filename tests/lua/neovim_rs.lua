local ffi = require("ffi")

ffi.cdef [[
    int rs_nvim_get_current_buf();
  ]]

return {
    lib = ffi.load("./target/debug/libneovim.dylib")
}
