use super::TableBuilder;
use mlua::prelude::*;

pub fn create_connection_handler(
    lua: &Lua,
    shutdown_tx: tokio::sync::watch::Sender<bool>,
) -> LuaResult<LuaTable> {
    let shutdown_tx_clone = shutdown_tx.clone();

    TableBuilder::new(lua)?
        .with_function("stop", move |_lua: &Lua, _: ()| {
            if shutdown_tx.is_closed() {
                return Ok(());
            }

            shutdown_tx.send(true).into_lua_err()?;
            Ok(())
        })?
        .with_function("is_running", move |_lua: &Lua, _: ()| {
            Ok(!*shutdown_tx_clone.borrow())
        })?
        .build_readonly()
}
