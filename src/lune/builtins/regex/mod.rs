mod config;

use crate::lune::util::TableBuilder;
use mlua::prelude::*;

use self::config::LuaRegex;

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", |_lua, pattern: LuaString| {
            Ok(LuaRegex {
                pattern: pattern.to_string_lossy().to_string(),
            })
        })?
        .build_readonly()
}
