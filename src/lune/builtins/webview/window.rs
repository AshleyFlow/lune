use std::{rc::Weak, time::Duration};

use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

use super::enums::LuaWindowEvent;

pub struct LuaWindow {}

pub struct LuaWindowState {
    pub webview: WebView,
    pub window: Window,
}

impl LuaWindow {
    pub fn new(lua: &Lua, config: LuaTable) -> LuaResult<LuaWindow> {
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

        lua.set_app_data(event_loop);
        lua.set_app_data(LuaWindowState {
            webview: webview.build().unwrap(),
            window,
        });

        Ok(LuaWindow {})
    }
}

fn lua_window_process_events<'lua>(lua: &'lua Lua, callback: LuaFunction<'lua>) -> LuaResult<()> {
    let func = lua.create_async_function(|lua: &Lua, callback: LuaFunction| async move {
        let mut event_loop = lua.app_data_mut::<EventLoop<()>>().unwrap();
        // let (send, mut receive) = channel::<LuaWindowEvent>(LuaWindowEvent::Nothing);

        loop {
            let mut lua_window_event: Option<LuaWindowEvent> = None;
            event_loop.pump_events(Some(Duration::ZERO), |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    lua_window_event = Some(LuaWindowEvent::Exit);
                    callback.call::<_, ()>(LuaWindowEvent::Exit).unwrap();
                    elwt.exit();
                }

                Event::AboutToWait => {}
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    lua_window_event = Some(LuaWindowEvent::Redraw);

                    lua.app_data_mut::<LuaWindowState>()
                        .unwrap()
                        .window
                        .request_redraw();
                }
                _ => (),
            });

            let lua_value = callback
                .call::<_, LuaValue>(lua_window_event.unwrap_or(LuaWindowEvent::Nothing))
                .unwrap();

            if lua_value.is_boolean() && lua_value.as_boolean().unwrap() {
                break;
            }

            tokio::time::sleep(Duration::from_millis(16)).await;
        }

        Ok(())
    })?;

    let thread = lua.create_thread(func)?;
    lua.push_thread_back(&thread, callback)?;

    Ok(())
}

async fn lua_window_run_script<'lua>(
    lua: &Lua,
    (script, callback): (LuaValue<'lua>, Option<LuaFunction<'lua>>),
) -> LuaResult<()> {
    if let Some(script) = script.as_str() {
        let (send, mut receive) = tokio::sync::watch::channel("null".to_string());

        let result = lua
            .app_data_mut::<LuaWindowState>()
            .unwrap()
            .webview
            .evaluate_script_with_callback(script, move |unknown| {
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
                        let inner_handler = inner_lua.registry_value::<LuaFunction>(&key).unwrap();
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

        Ok(())
    } else {
        Err(LuaError::FromLuaConversionError {
            from: script.type_name(),
            to: "string",
            message: None,
        })
    }
}

fn lua_window_set_visible(lua: &Lua, visible: bool) -> LuaResult<()> {
    lua.app_data_mut::<LuaWindowState>()
        .unwrap()
        .window
        .set_visible(visible);

    Ok(())
}

impl LuaUserData for LuaWindow {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("process_events", lua_window_process_events);
        methods.add_async_function("run_script", lua_window_run_script);
        methods.add_function("set_visible", lua_window_set_visible);
    }
}
