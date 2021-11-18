local ffi = require("ffi")

ffi.cdef [[
    bool test_overkill_set_global_option();
]]

local suffix = ffi.os == "OSX" and ".dylib" or ".so"
local lib = ffi.load("./target/debug/liboverkill_nvim" .. suffix)

describe(
    "api",
    function()
        it(
            "tests nvim_set_global_option()",
            function()
                assert.True(lib.test_overkill_set_global_option())
            end
        )
    end
)
