local ffi = require("ffi")

ffi.cdef [[
    bool test_nvim_set_var();
    bool test_nvim_set_vvar();
    bool test_nvim_buf_set_var();
    bool test_nvim_get_current_buf();
    bool test_nvim_feedkeys();
  ]]

local lib = ffi.load("./target/debug/libneovim.dylib")

describe(
    "nvim_set_var() and nvim_get_var()",
    function()
        it(
            "tests all the things in rust",
            function()
                assert.True(lib.test_nvim_set_var())
            end
        )
    end
)

describe(
    "nvim_set_vvar() and nvim_get_vvar()",
    function()
        it(
            "tests all the things in rust",
            function()
                assert.True(lib.test_nvim_set_vvar())
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
                assert.True(lib.test_nvim_buf_set_var())
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
                assert.True(lib.test_nvim_get_current_buf())
            end
        )
    end
)

describe(
    "nvim_feedkeys()",
    function()
        it(
            "some test",
            function()
                assert.True(lib.test_nvim_feedkeys())
            end
        )
    end
)
