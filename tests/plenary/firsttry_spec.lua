local ffi = require("ffi")

ffi.cdef [[
    bool nvim_get_current_buf_test();

    bool nvim_get_var_test_string(const char *var, const char *expected);
    bool test_set_get_var();
  ]]

local lib = ffi.load("./target/debug/libneovim.dylib")

describe(
    "some basics",
    function()
        it(
            "some test",
            function()
                assert.True(lib.nvim_get_current_buf_test())
            end
        )
    end
)

describe(
    "nvim_get_var()",
    function()
        it(
            "tests all the things in rust",
            function()
                assert.True(lib.test_set_get_var())
            end
        )

        -- it(
        --     "can fetch an string",
        --     function()
        --         local var = "nvim_get_var_test_string"
        --         local expected = "i like turtles"
        --         vim.api.nvim_set_var(var, expected)
        --         assert.True(lib.nvim_get_var_test_string(var, expected))
        --     end
        -- )

        -- it(
        --     "can fetch an array",
        --     function()
        --         local var = "nvim_get_var_test_array"
        --         local expected = {55, 42}
        --         vim.api.nvim_set_var(var, expected)
        --         assert.True(lib.nvim_get_var_test_array(var, expected))
        --     end
        -- )
    end
)
