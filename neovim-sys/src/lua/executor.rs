use crate::api::vim::LuaRef;

extern "C" {
    pub fn api_free_luaref(api_lua_ref: LuaRef);
}
