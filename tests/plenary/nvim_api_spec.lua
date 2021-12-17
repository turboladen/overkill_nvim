local ffi = require("ffi")

ffi.cdef [[
    bool test_nvim_set_var();
    bool test_nvim_set_vvar();
    bool test_nvim_buf_set_var();
    bool test_nvim_get_current_buf();
    bool test_nvim_feedkeys();
    bool test_nvim_get_mode();
    bool test_nvim_set_global_option();
    bool test_set_map();
    bool test_set_buf_map();
  ]]

local suffix = ffi.os == "OSX" and ".dylib" or ".so"
local lib = ffi.load("./target/debug/libnvim_api" .. suffix)

describe(
    "api",
    function()
        it(
            "tests nvim_set_var() and nvim_get_var()",
            function()
                assert.True(lib.test_nvim_set_var())
            end
        )

        it(
            "tests nvim_set_vvar() and nvim_get_vvar()",
            function()
                assert.True(lib.test_nvim_set_vvar())
            end
        )

        it(
            "tests nvim_buf_set_var() and nvim_buf_get_var()",
            function()
                assert.True(lib.test_nvim_buf_set_var())
            end
        )
        it(
            "tests nvim_get_current_buf()",
            function()
                assert.True(lib.test_nvim_get_current_buf())
            end
        )
        it(
            "tests nvim_feedkeys()",
            function()
                assert.True(lib.test_nvim_feedkeys())
            end
        )
        it(
            "tests nvim_get_mode()",
            function()
                assert.True(lib.test_nvim_get_mode())
            end
        )
        it(
            "tests nvim_set_global_option()",
            function()
                assert.True(lib.test_nvim_set_global_option())
            end
        )
        it(
            "tests api::keymap::map()",
            function()
                assert.True(lib.test_set_map())
            end
        )
        it(
            "tests api::keymap::buf_map()",
            function()
                assert.True(lib.test_set_buf_map())
            end
        )
    end
)
