mod builder;
mod config;
mod logic;

use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use std::{ops::Deref, rc::Weak};

use self::{
    builder::BuilderConfig,
    config::{LuaWebview, WebviewConfig},
    logic::config::{WebviewCommand, WebviewEvent},
};
use crate::lune::util::TableBuilder;

struct LuaWebviewState {}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_async_function("build", build)?
        .build_readonly()
}

async fn build<'lua>(lua: &'lua Lua, config: WebviewConfig<'lua>) -> LuaResult<LuaWebview> {
    let lua_strong = {
        lua.app_data_ref::<Weak<Lua>>()
            .expect("Missing weak lua ref")
            .upgrade()
            .expect("Lua was dropped unexpectedly")
    };

    let url: Option<String> = config
        .url
        .map(|lua_url| lua_url.to_string_lossy().to_string());

    let exit_key: Option<LuaRegistryKey> = config
        .on_exit
        .map(|callback| lua.create_registry_value(callback).unwrap());

    /*

    If a channel's sender or receiver starts with a '_'-
    -it indicates that it's going to be used in the second thread

    */
    let (send_msg, mut _receive_msg) = tokio::sync::broadcast::channel::<WebviewCommand>(3);
    let (_send_msg, mut receive_msg_outer) =
        tokio::sync::watch::channel::<WebviewEvent>(WebviewEvent::Init);
    let receive_msg_inner = receive_msg_outer.clone();

    if lua.app_data_ref::<LuaWebview>().is_some() {
        return Err(LuaError::RuntimeError(
            "You are not allowed to make more than one webview window at a time for now."
                .to_owned(),
        ));
    }

    lua.set_app_data(LuaWebviewState {});

    builder::start(_send_msg, _receive_msg, BuilderConfig { url }).unwrap();

    lua.spawn_local(async move {
        loop {
            let changed = receive_msg_outer.changed();

            if changed.await.is_ok() {
                if let WebviewEvent::ClosedWindow = receive_msg_outer.borrow_and_update().deref() {
                    if let Some(exit_key) = &exit_key {
                        if let Ok(callback) = lua_strong.registry_value::<LuaFunction>(exit_key) {
                            callback.call::<_, ()>(()).expect("Failed to call 'onExit'");
                        }
                    }

                    break;
                }
            }
        }
    });

    Ok(LuaWebview {
        send_message: send_msg,
        receive_message: receive_msg_inner,
    })
}
