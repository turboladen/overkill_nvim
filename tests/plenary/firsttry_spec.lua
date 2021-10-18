describe(
    "some basics",
    function()
        it(
            "some test",
            function()
                assert.equals(require("neovim_rs").lib.rs_nvim_get_current_buf(), 1)
            end
        )
    end
)
