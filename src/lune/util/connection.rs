use super::TableBuilder;
use mlua::prelude::*;

const CONNECTION_IMPL_LUA: &str = r#"
return freeze(setmetatable({
    disconnect = function(...)
		return connection:disconnect(...)
	end,

    -- for backward compatibility
	stop = function(...)
		return connection:disconnect(...)
	end,
    is_running = function()
		return connection.connected
	end,
}, {
	__index = function(self, key)
		if key == "connected" then
			return connection.connected
		end
	end,
}))
"#;

pub struct LuaConnection {
    shutdown_tx: Option<tokio::sync::watch::Sender<bool>>,
}

impl LuaUserData for LuaConnection {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("connected", |_lua, this| Ok(this.shutdown_tx.is_some()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("disconnect", |_lua, this, _: ()| {
            if this.shutdown_tx.is_none() {
                return Ok(());
            }

            this.shutdown_tx
                .clone()
                .unwrap()
                .send(true)
                .into_lua_err()?;
            this.shutdown_tx = None;

            Ok(())
        });
    }
}

pub fn create_connection_handler(
    lua: &Lua,
    shutdown_tx: tokio::sync::watch::Sender<bool>,
) -> LuaResult<LuaTable> {
    let setmetatable = lua.globals().get::<_, LuaFunction>("setmetatable")?;
    let table_freeze = lua
        .globals()
        .get::<_, LuaTable>("table")?
        .get::<_, LuaFunction>("freeze")?;

    let env = TableBuilder::new(lua)?
        .with_value(
            "connection",
            LuaConnection {
                shutdown_tx: Some(shutdown_tx),
            },
        )?
        .with_value("setmetatable", setmetatable)?
        .with_value("freeze", table_freeze)?
        .build_readonly()?;

    lua.load(CONNECTION_IMPL_LUA)
        .set_name("connection")
        .set_environment(env)
        .eval()
}
