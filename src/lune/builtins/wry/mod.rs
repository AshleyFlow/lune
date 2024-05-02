mod config;
mod webview;
mod window;

use self::{config::EventLoopMessage, window::config::LuaWindow};
use crate::lune::util::{connection::create_connection_handler, TableBuilder};
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    rc::Weak,
    sync::{Arc, Mutex},
    time::Duration,
};
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

pub static EVENT_LOOP_SENDER: Lazy<
    tokio::sync::watch::Sender<(Option<WindowId>, EventLoopMessage)>,
> = Lazy::new(|| {
    let init = (None, EventLoopMessage::none());
    tokio::sync::watch::Sender::new(init)
});

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_async_function("event_loop", winit_event_loop)?
        .with_async_function("run", winit_run)?
        .with_function("create_window", winit_create_window)?
        .with_function("create_webview", winit_create_webview)?
        .build_readonly()
}

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<(WindowId, EventLoopMessage)>> = RefCell::new(EventLoopBuilder::with_user_event().build());
}

pub static EVENT_LOOP_STARTED: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

pub fn winit_create_window<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    window::create(lua, values)
}

pub fn winit_create_webview<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    webview::create(lua, values)
}

pub fn error_if_before_run(name: &str) -> Option<LuaError> {
    let event_loop_started = EVENT_LOOP_STARTED.lock().unwrap();

    if !(*event_loop_started) {
        Some(LuaError::RuntimeError(format!(
            "please make sure to call {} after calling wry.run()",
            name
        )))
    } else {
        None
    }
}

pub async fn winit_run(lua: &Lua, _: ()) -> LuaResult<()> {
    let mut event_loop_started = EVENT_LOOP_STARTED.lock().unwrap();

    if *event_loop_started {
        return Err(LuaError::RuntimeError(
            "wry.run() got called more than once\nthe first call to event_loop automatically calls this function.".into(),
        ));
    }

    *event_loop_started = true;

    lua.spawn_local(async {
        loop {
            let mut message: (Option<WindowId>, EventLoopMessage) =
                (None, EventLoopMessage::none());

            EVENT_LOOP.with(|event_loop| {
                let mut event_loop = event_loop.borrow_mut();

                event_loop.run_return(|event, _elwt, flow| {
                    *flow = tao::event_loop::ControlFlow::Exit;

                    match event {
                        tao::event::Event::UserEvent((window_id, msg)) => {
                            message = (Some(window_id), msg);
                        }
                        tao::event::Event::WindowEvent {
                            window_id,
                            event: tao::event::WindowEvent::CloseRequested,
                            ..
                        } => {
                            message = (Some(window_id), EventLoopMessage::close_requested());
                        }
                        tao::event::Event::WindowEvent {
                            window_id,
                            event:
                                tao::event::WindowEvent::CursorMoved {
                                    device_id: _,
                                    position,
                                    ..
                                },
                            ..
                        } => {
                            message = (
                                Some(window_id),
                                EventLoopMessage::cursor_moved(position.x, position.y),
                            );
                        }
                        tao::event::Event::WindowEvent {
                            window_id,
                            event:
                                tao::event::WindowEvent::MouseInput {
                                    device_id: _,
                                    state,
                                    button,
                                    ..
                                },
                            ..
                        } => {
                            let button = match button {
                                tao::event::MouseButton::Left => Some("left".to_string()),
                                tao::event::MouseButton::Right => Some("right".to_string()),
                                tao::event::MouseButton::Middle => Some("middle".to_string()),
                                _ => None,
                            };

                            if let Some(button) = button {
                                let pressed = match state {
                                    tao::event::ElementState::Pressed => true,
                                    tao::event::ElementState::Released => false,
                                    _ => false,
                                };

                                message = (
                                    Some(window_id),
                                    EventLoopMessage::mouse_button(button, pressed),
                                );
                            }
                        }
                        tao::event::Event::WindowEvent {
                            window_id,
                            event:
                                tao::event::WindowEvent::KeyboardInput {
                                    device_id: _,
                                    event,
                                    is_synthetic: _,
                                    ..
                                },
                            ..
                        } => {
                            let keycode: String = format!("{}", event.physical_key);

                            let pressed = if event.repeat {
                                true
                            } else {
                                match event.state {
                                    tao::event::ElementState::Pressed => true,
                                    tao::event::ElementState::Released => false,
                                    _ => false,
                                }
                            };

                            message =
                                (Some(window_id), EventLoopMessage::keycode(keycode, pressed));
                        }
                        _ => {}
                    }
                });
            });

            if !EVENT_LOOP_SENDER.is_closed() {
                EVENT_LOOP_SENDER.send(message).into_lua_err().unwrap();
            } else {
                break;
            }

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    Ok(())
}

pub async fn winit_event_loop<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaTable<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let field2 = values.get(1).expect("Parameter 2 is missing");

    let (window_key, callback_key) = {
        let window_key = lua.create_registry_value(field1)?;
        let callback_key = lua.create_registry_value(field2)?;
        (window_key, callback_key)
    };

    let inner_lua = lua
        .app_data_ref::<Weak<Lua>>()
        .expect("Missing weak lua ref")
        .upgrade()
        .expect("Lua was dropped unexpectedly");

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);

    lua.spawn_local(async move {
        let mut listener = EVENT_LOOP_SENDER.subscribe();

        let inner_field1 = inner_lua.registry_value::<LuaValue>(&window_key).unwrap();
        let inner_field2 = inner_lua.registry_value::<LuaValue>(&callback_key).unwrap();

        loop {
            tokio::select! {
                _ = listener.changed() => {

                },
                res = shutdown_rx.changed() => {
                    if res.is_ok() {
                        break;
                    }
                }
            }

            {
                let (window, callback) = {
                    let window = inner_field1
                        .as_userdata()
                        .unwrap()
                        .borrow::<LuaWindow>()
                        .unwrap();

                    let callback = inner_field2.as_function().unwrap();

                    (window, callback.clone())
                };

                let message = listener.borrow_and_update().clone();

                if let Some(window_id) = message.0 {
                    if window.window.id() != window_id {
                        continue;
                    }
                }

                window.window.request_redraw();

                let thread = inner_lua.create_thread(callback).unwrap();
                inner_lua
                    .as_ref()
                    .push_thread_back(thread, message.1)
                    .into_lua_err()
                    .unwrap();
            }

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    let run = {
        let event_loop_started = EVENT_LOOP_STARTED.lock().unwrap();
        !(*event_loop_started)
    };

    if run {
        winit_run(lua, ()).await?;
    }

    create_connection_handler(lua, shutdown_tx)
}
