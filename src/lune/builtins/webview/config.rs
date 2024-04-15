use std::{ops::Deref, rc::Weak};

use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;

use super::logic::config::{WebviewCommand, WebviewEvent};

pub struct LuaWebview {
    pub send_message: tokio::sync::broadcast::Sender<WebviewCommand>,
    pub receive_message: tokio::sync::watch::Receiver<WebviewEvent>,
}

impl LuaUserData for LuaWebview {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("exit", |_lua, webview, _: ()| {
            if webview
                .send_message
                .send(WebviewCommand::CloseWindow)
                .is_err()
            {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("openDevtools", |_lua, webview, _: ()| {
            if webview
                .send_message
                .send(WebviewCommand::OpenDevtools)
                .is_err()
            {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("closeDevtools", |_lua, webview, _: ()| {
            if webview
                .send_message
                .send(WebviewCommand::CloseDevtools)
                .is_err()
            {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method(
            "executeJavascript",
            |lua, webview, (script, callback): (LuaString, Option<LuaFunction>)| {
                if webview
                    .send_message
                    .send(WebviewCommand::ExecuteJavascript(
                        script.to_string_lossy().to_string(),
                    ))
                    .is_err()
                {
                    return Err(LuaError::RuntimeError(
                        "Failed to send javascript code to webview".into(),
                    ));
                };

                if let Some(callback) = callback {
                    let lua_strong = {
                        lua.app_data_ref::<Weak<Lua>>()
                            .expect("Missing weak lua ref")
                            .upgrade()
                            .expect("Lua was dropped unexpectedly")
                    };

                    let exit_key: LuaRegistryKey = lua.create_registry_value(callback).unwrap();
                    let mut receive_msg_outer = webview.receive_message.clone();

                    lua.spawn_local(async move {
                        loop {
                            let changed = receive_msg_outer.changed();

                            if changed.await.is_ok() {
                                if let WebviewEvent::ExecutedJavascript(unknown) =
                                    receive_msg_outer.borrow_and_update().deref()
                                {
                                    if let Ok(callback) =
                                        lua_strong.registry_value::<LuaFunction>(&exit_key)
                                    {
                                        let lua_string = unknown.clone().into_lua(&lua_strong);

                                        callback
                                            .call::<_, LuaValue>(lua_string.unwrap())
                                            .expect("Failed to call exectureJavascript 'callback'");
                                    }

                                    break;
                                }
                            }
                        }
                    });
                }

                Ok(())
            },
        );

        methods.add_method("loadUrl", |_lua, webview, url: LuaString| {
            if webview
                .send_message
                .send(WebviewCommand::LoadUrl(url.to_string_lossy().to_string()))
                .is_err()
            {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("onExit", |lua, webview, callback: LuaFunction| {
            let lua_strong = {
                lua.app_data_ref::<Weak<Lua>>()
                    .expect("Missing weak lua ref")
                    .upgrade()
                    .expect("Lua was dropped unexpectedly")
            };

            let exit_key: LuaRegistryKey = lua.create_registry_value(callback).unwrap();
            let mut receive_msg_outer = webview.receive_message.clone();

            lua.spawn_local(async move {
                loop {
                    let changed = receive_msg_outer.changed();

                    if changed.await.is_ok() {
                        if let WebviewEvent::ClosedWindow =
                            receive_msg_outer.borrow_and_update().deref()
                        {
                            if let Ok(callback) =
                                lua_strong.registry_value::<LuaFunction>(&exit_key)
                            {
                                callback.call::<_, ()>(()).expect("Failed to call 'onExit'");
                            }

                            break;
                        }
                    }
                }
            });

            Ok(())
        });
    }
}

pub struct WebviewConfig<'a> {
    pub on_exit: Option<LuaFunction<'a>>,
    pub url: Option<LuaString<'a>>,
}

impl<'lua> FromLua<'lua> for WebviewConfig<'lua> {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::Table(tab) = &value {
            let on_exit: Option<LuaFunction> = tab.get("onExit")?;
            let url: Option<LuaString> = tab.get("url")?;

            Ok(Self { on_exit, url })
        } else {
            // Anything else is invalid
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "WebviewConfig",
                message: Some(format!(
                    "Invalid webview config - expected table, got {}",
                    value.type_name()
                )),
            })
        }
    }
}
