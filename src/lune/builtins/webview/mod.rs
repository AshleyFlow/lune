mod builder;
mod config;
mod logic;

use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use std::{rc::Weak, sync::Arc};

use self::{builder::BuilderConfig, config::WebviewConfig};
use crate::lune::util::TableBuilder;

struct LuaWebview {}

pub const CLOSED_WINDOW_MSG: &str = "^ClosedWindow";

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_async_function("build", build)?
        .build_readonly()
}

async fn build<'lua>(lua: &'lua Lua, config: WebviewConfig<'lua>) -> LuaResult<LuaTable<'lua>> {
    let lua_strong = {
        lua.app_data_ref::<Weak<Lua>>()
            .expect("Missing weak lua ref")
            .upgrade()
            .expect("Lua was dropped unexpectedly")
    };

    let url: String = config.url.to_string_lossy().into();
    let exit_key: Option<LuaRegistryKey> = config
        .exit
        .map(|callback| lua.create_registry_value(callback).unwrap());

    /*

    If a channel's sender or receiver starts with a '_'-
    -it indicates that it's going to be used in the second thread

    */
    let (send_msg, mut _receive_msg) = tokio::sync::watch::channel::<String>("".to_owned());
    let (_send_msg, receive_msg) = tokio::sync::watch::channel::<String>("".to_owned());

    if lua.app_data_ref::<LuaWebview>().is_some() {
        return Err(LuaError::RuntimeError(
            "You are not allowed to make more than one webview window at a time for now."
                .to_owned(),
        ));
    }

    lua.set_app_data(LuaWebview {});

    tokio::spawn(async move {
        builder::start(
            _send_msg,
            &mut _receive_msg,
            BuilderConfig { url: url.clone() },
        )
        .unwrap();
    });

    lua.spawn_local(async move {
        let mut receive_msg_outer = receive_msg.clone();
        loop {
            let changed = receive_msg_outer.changed();

            if changed.await.is_ok()
                && receive_msg_outer.borrow_and_update().clone() == CLOSED_WINDOW_MSG
            {
                if let Some(exit_key) = &exit_key {
                    if let Ok(callback) = lua_strong.registry_value::<LuaFunction>(exit_key) {
                        callback.call::<_, ()>(()).expect("Failed to call 'exit'");
                    }
                }

                break;
            }
        }
    });

    let send_msg = Arc::new(send_msg);
    let send_msg1 = send_msg.clone();
    let send_msg2 = send_msg.clone();
    let send_msg3 = send_msg.clone();
    let send_msg4 = send_msg.clone();

    TableBuilder::new(lua)?
        .with_function("exit", move |lua, _: ()| {
            send_msg1
                .clone()
                .send("^CloseWindow".to_owned())
                .expect("Failed to send message to channel");

            lua.remove_app_data::<LuaWebview>().unwrap();

            Ok(())
        })?
        .with_function("open_devtools", move |_lua, _: ()| {
            send_msg2
                .clone()
                .send("^OpenDevtools".to_owned())
                .expect("Failed to send message to channel");
            Ok(())
        })?
        .with_function("close_devtools", move |_lua, _: ()| {
            send_msg3
                .clone()
                .send("^CloseDevtools".to_owned())
                .expect("Failed to send message to channel");
            Ok(())
        })?
        .with_function("load_url", move |_lua, url: LuaString| {
            send_msg4
                .clone()
                .send("^LoadUrl:".to_owned() + url.to_string_lossy().to_string().as_str())
                .expect("Failed to send message to channel");
            Ok(())
        })?
        .build_readonly()
}
