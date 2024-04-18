mod enums;

use crate::lune::util::TableBuilder;
use enums::LuaWindowEvent;
use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use std::{rc::Weak, sync::mpsc::channel, time::Duration};
use tokio::time;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    let events = TableBuilder::new(lua)?
        .with_value("Nothing", LuaWindowEvent::Nothing)?
        .with_value("Redraw", LuaWindowEvent::Redraw)?
        .with_value("Exit", LuaWindowEvent::Exit)?
        .build_readonly()?;

    TableBuilder::new(lua)?
        .with_value("events", events)?
        .with_function("new", LuaWindow::new)?
        .build_readonly()
}

pub struct LuaWindow {
    event_loop: EventLoop<()>,
    webview: WebView,
    window: Window,
}

impl LuaWindow {
    pub fn new(_lua: &Lua, config: LuaTable) -> LuaResult<LuaWindow> {
        let title: String = config.get("title").unwrap_or("Lune".into());
        let html: Result<String, LuaError> = config.get("html");
        let url: Result<String, LuaError> = config.get("url");

        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        let mut webview = WebViewBuilder::new(&window);

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

        Ok(LuaWindow {
            event_loop,
            webview: webview.build().unwrap(),
            window,
        })
    }
}

async fn lua_window_process_events(
    _lua: &Lua,
    this: &mut LuaWindow,
    _: (),
) -> LuaResult<LuaWindowEvent> {
    let (send, receive) = channel::<LuaWindowEvent>();
    let event_loop = &mut this.event_loop;

    event_loop.pump_events(Some(Duration::ZERO), |event, _elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            send.send(LuaWindowEvent::Exit).unwrap();
        }

        Event::AboutToWait => this.window.request_redraw(),
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            send.send(LuaWindowEvent::Redraw).unwrap();
        }
        _ => (),
    });

    time::sleep(Duration::ZERO).await;

    let event = receive.try_recv();
    Ok(event.unwrap_or(LuaWindowEvent::Nothing))
}

async fn lua_window_run_script<'lua>(
    lua: &Lua,
    this: &LuaWindow,
    (script, callback): (LuaValue<'lua>, Option<LuaFunction<'lua>>),
) -> LuaResult<()> {
    if let Some(script) = script.as_str() {
        let (send, mut receive) = tokio::sync::watch::channel("null".to_string());

        let result = this
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

fn lua_window_set_visible(_lua: &Lua, this: &LuaWindow, visible: bool) -> LuaResult<()> {
    this.window.set_visible(visible);

    Ok(())
}

impl LuaUserData for LuaWindow {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("process_events", lua_window_process_events);
        methods.add_async_method("run_script", lua_window_run_script);
        methods.add_method("set_visible", lua_window_set_visible);
    }
}
