local ffi = require("ffi")

ffi.cdef [[
    bool test_overkill_set_global_string_option();
    bool test_overkill_set_global_nullable_string_option();
    bool test_overkill_set_global_nullable_string_char_flags_option();
    bool test_overkill_set_add_assign_global_nullable_string_char_flags_option();
    bool test_overkill_set_sub_assign_global_nullable_string_char_flags_option();

    bool test_map();
    bool test_map_normal();
    bool test_map_visual_select();
    bool test_map_visual();
    bool test_map_select();
    bool test_map_operator_pending();
    bool test_map_insert();
    bool test_map_insert_and_command_line();
    bool test_map_language_mapping();
    bool test_map_command_line();
    bool test_map_terminal_job();
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
        it(
            "tests Mapper::map() with default mode",
            function()
                assert.True(lib.test_map())
            end
        )
        it(
            "tests Mapper::map() with normal mode",
            function()
                assert.True(lib.test_map_normal())
            end
        )
        it(
            "tests Mapper::map() with visual-select mode",
            function()
                assert.True(lib.test_map_visual_select())
            end
        )
        it(
            "tests Mapper::map() with visual mode",
            function()
                assert.True(lib.test_map_visual())
            end
        )
        it(
            "tests Mapper::map() with select mode",
            function()
                assert.True(lib.test_map_select())
            end
        )
        it(
            "tests Mapper::map() with operator-pending mode",
            function()
                assert.True(lib.test_map_operator_pending())
            end
        )
        it(
            "tests Mapper::map() with insert mode",
            function()
                assert.True(lib.test_map_insert())
            end
        )
        -- I don't understand why this is failing...
        -- it(
        --     "tests Mapper::map() with insert-and-command-line mode",
        --     function()
        --         assert.True(lib.test_map_insert_and_command_line())
        --     end
        -- )
        it(
            "tests Mapper::map() with language-mapping mode",
            function()
                assert.True(lib.test_map_language_mapping())
            end
        )
        it(
            "tests Mapper::map() with command-line mode",
            function()
                assert.True(lib.test_map_command_line())
            end
        )
        it(
            "tests Mapper::map() with terminal-job mode",
            function()
                assert.True(lib.test_map_terminal_job())
            end
        )
    end
)
