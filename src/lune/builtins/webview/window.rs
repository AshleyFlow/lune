use std::{rc::Weak, time::Duration};

use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
    window::{WindowBuilder, WindowId},
};
use wry::WebViewBuilder;

use super::{enums::LuaWindowEvent, EVENT_LOOP, WEBVIEWS, WINDOWS};

pub struct LuaWindow {
    pub window_id: WindowId,
}

impl LuaWindow {
    pub fn new(_lua: &Lua, config: LuaTable) -> LuaResult<LuaWindow> {
        let reload_script: Result<String, LuaError> = config.get("reload_script");
        let title: String = config.get("title").unwrap_or("Lune".into());
        let html: Result<String, LuaError> = config.get("html");
        let url: Result<String, LuaError> = config.get("url");

        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        let mut webview = WebViewBuilder::new(&window);

        webview = if let Ok(reload_script) = reload_script {
            webview.with_initialization_script(reload_script.as_str())
        } else {
            webview
        };

        webview = if let Ok(url) = url {
            webview.with_url(url)
        } else {
            webview
        };

        webview = if let Ok(html) = html {
            webview.with_html(html)
        } else {
            webview
        };

        let id = window.id();
        WEBVIEWS.with_borrow_mut(|map| {
            let webview = webview.build().unwrap();
            map.insert(id, webview)
        });
        WINDOWS.with_borrow_mut(|map| map.insert(id, window));

        Ok(LuaWindow { window_id: id })
    }
}

async fn lua_window_process_events<'lua>(
    _lua: &'lua Lua,
    this: &LuaWindow,
    callback: LuaFunction<'lua>,
) -> LuaResult<()> {
    loop {
        let mut lua_window_event: Option<LuaWindowEvent> = None;
        EVENT_LOOP.with(|event_loop| {
            event_loop
                .borrow_mut()
                .pump_events(Some(Duration::ZERO), |event, elwt| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                        ..
                    } => {
                        if window_id == this.window_id {
                            lua_window_event = Some(LuaWindowEvent::Exit);
                            callback.call::<_, ()>(LuaWindowEvent::Exit).unwrap();
                            elwt.exit();
                        }
                    }
                    Event::AboutToWait => {}
                    _ => (),
                });
        });

        tokio::time::sleep(Duration::from_millis(16)).await;

        let lua_value = callback
            .call::<_, LuaValue>(lua_window_event.unwrap_or(LuaWindowEvent::Nothing))
            .unwrap();

        if lua_value.is_boolean() && lua_value.as_boolean().unwrap() {
            break;
        }
    }

    Ok(())
}

async fn lua_window_run_script<'lua>(
    lua: &Lua,
    this: &LuaWindow,
    (script, callback): (LuaValue<'lua>, Option<LuaFunction<'lua>>),
) -> LuaResult<()> {
    if let Some(script) = script.as_str() {
        let (send, mut receive) = tokio::sync::watch::channel("null".to_string());

        WEBVIEWS.with(|webviews| {
            let webviews = webviews.borrow();

            if let Some(webview) = webviews.get(&this.window_id) {
                let result = webview.evaluate_script_with_callback(script, move |unknown| {
                    if send.receiver_count() == 0 {
                        return;
                    }

                    if let Err(error) = send.send(unknown) {
                        println!("{}", error);
                    };
                });

                if result.is_err() {
                    return Err(LuaError::RuntimeError("Failed to evaluate script".into()));
                }

                if let Some(callback) = callback {
                    let inner_lua = lua
                        .app_data_ref::<Weak<Lua>>()
                        .expect("Missing weak lua ref")
                        .upgrade()
                        .expect("Lua was dropped unexpectedly");
                    let key = lua.create_registry_value(callback)?;

                    lua.spawn_local(async move {
                        let mut receive_inner = receive.clone();
                        loop {
                            let changed = receive_inner.changed().await;

                            if changed.is_ok() {
                                let inner_handler =
                                    inner_lua.registry_value::<LuaFunction>(&key).unwrap();
                                let unknown = receive.borrow_and_update();

                                inner_handler
                                    .call_async::<_, LuaValue>(unknown.clone().into_lua(&inner_lua))
                                    .await
                                    .unwrap();

                                break;
                            }
                        }
                    })
                }
            }

            Ok(())
        })
    } else {
        Err(LuaError::FromLuaConversionError {
            from: script.type_name(),
            to: "string",
            message: None,
        })
    }
}

fn lua_window_set_visible(_lua: &Lua, this: &LuaWindow, visible: bool) -> LuaResult<()> {
    WINDOWS.with(|windows| {
        let windows = windows.borrow();

        if let Some(window) = windows.get(&this.window_id) {
            window.set_visible(visible);
        }
    });

    Ok(())
}

impl LuaUserData for LuaWindow {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("process_events", lua_window_process_events);
        methods.add_async_method("run_script", lua_window_run_script);
        methods.add_method("set_visible", lua_window_set_visible);
    }
}
