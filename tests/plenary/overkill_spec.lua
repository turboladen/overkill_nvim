local ffi = require("ffi")

ffi.cdef [[
    bool test_overkill_set_global_string_option();
    bool test_overkill_set_global_nullable_string_option();
    bool test_overkill_set_global_nullable_string_char_flags_option();
    bool test_overkill_set_add_assign_global_nullable_string_char_flags_option();
    bool test_overkill_set_sub_assign_global_nullable_string_char_flags_option();
]]

local suffix = ffi.os == "OSX" and ".dylib" or ".so"
local lib = ffi.load("./target/debug/liboverkill_nvim" .. suffix)

describe(
    "api",
    function()
        it(
            "tests nvim_set_global_option() as a string",
            function()
                assert.True(lib.test_overkill_set_global_string_option())
            end
        )
        it(
            "tests nvim_set_global_nullable_string_option() as a nullable string",
            function()
                assert.True(lib.test_overkill_set_global_nullable_string_option())
            end
        )
        it(
            "tests nvim_set_global_nullable_string_option() as a char flags",
            function()
                assert.True(lib.test_overkill_set_global_nullable_string_char_flags_option())
            end
        )
        it(
            "tests nvim_set_global_add_assign_nullable_string_option() as a char flags",
            function()
                assert.True(lib.test_overkill_set_add_assign_global_nullable_string_char_flags_option())
            end
        )
        it(
            "tests nvim_set_global_sub_assign_nullable_string_option() as a char flags",
            function()
                assert.True(lib.test_overkill_set_sub_assign_global_nullable_string_char_flags_option())
            end
        )
    end
)
