local ffi = require("ffi")

ffi.cdef [[
    // bool nvim_get_current_buf_test();

    bool test_set_get_var();
    bool test_buf_set_get_var();
  ]]

local lib = ffi.load("./target/debug/libneovim.dylib")

-- describe(
--     "some basics",
--     function()
--         it(
--             "some test",
--             function()
--                 assert.True(lib.nvim_get_current_buf_test())
--             end
--         )
--     end
-- )

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
