use mlua::prelude::*;

use super::context::*;

pub(super) fn require<'lua, 'ctx>(
    lua: &'lua Lua,
    ctx: &'ctx RequireContext,
    alias: &str,
    module: &str,
) -> LuaResult<LuaMultiValue<'lua>>
where
    'lua: 'ctx,
{
    ctx.load_library(lua, alias, module)
}
