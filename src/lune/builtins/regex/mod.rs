mod config;

use crate::lune::util::TableBuilder;
use mlua::prelude::*;

use self::config::LuaRegex;

const REGEX_IMPL_LUA: &str = r#"
return freeze({
    match = function(...)
		return regex:match(...)
	end
})
"#;

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, pattern: LuaString| {
            let lib = LuaRegex {
                pattern: pattern.to_string_lossy().to_string(),
            };

            let table_freeze = lua
                .globals()
                .get::<_, LuaTable>("table")?
                .get::<_, LuaFunction>("freeze")?;

            let env = TableBuilder::new(lua)?
                .with_value("regex", lib)?
                .with_value("freeze", table_freeze)?
                .build_readonly()?;

            lua.load(REGEX_IMPL_LUA)
                .set_name("regex")
                .set_environment(env)
                .eval::<LuaTable>()
        })?
        .build_readonly()
}
