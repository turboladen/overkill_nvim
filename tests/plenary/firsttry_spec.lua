local ffi = require("ffi")

ffi.cdef [[
    bool test_set_get_var();
    bool test_buf_set_get_var();
    bool nvim_get_current_buf_test();
  ]]

local lib = ffi.load("./target/debug/libneovim.dylib")

describe(
    "nvim_set_var() and nvim_get_var()",
    function()
        it(
            "tests all the things in rust",
            function()
                assert.True(lib.test_set_get_var())
            end
        )
    end
)

describe(
    "nvim_buf_set_var() and nvim_buf_get_var()",
    function()
        it(
            "tests all the things in rust",
            function()
                assert.True(lib.test_buf_set_get_var())
            end
        )
    end
)

describe(
    "nvim_get_current_buf()",
    function()
        it(
            "some test",
            function()
                assert.True(lib.nvim_get_current_buf_test())
            end
        )
    end
)
