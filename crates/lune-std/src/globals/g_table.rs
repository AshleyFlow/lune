use crate::context::GlobalsContext;
use mlua::prelude::*;

pub fn create<'lua>(lua: &'lua Lua, _: &'lua GlobalsContext) -> LuaResult<LuaValue<'lua>> {
    lua.create_table()?.into_lua(lua)
}
