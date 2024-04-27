mod config;
mod webview;
mod window;

use crate::lune::util::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Weak, time::Duration};
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

use self::{config::EventLoopMessage, window::config::LuaWindow};

pub static EVENT_LOOP_SENDER: Lazy<
    tokio::sync::watch::Sender<(Option<WindowId>, EventLoopMessage)>,
> = Lazy::new(|| {
    let init = (None, EventLoopMessage::None);
    tokio::sync::watch::Sender::new(init)
});

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    let events = EventLoopMessage::create_lua_table(lua)?;

    TableBuilder::new(lua)?
        .with_value("events", events)?
        .with_async_function("event_loop", winit_event_loop)?
        .with_async_function("run", winit_run)?
        .with_function("create_window", winit_create_window)?
        .with_function("create_webview", winit_create_webview)?
        .build_readonly()
}

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<(WindowId, EventLoopMessage)>> = RefCell::new(EventLoopBuilder::with_user_event().build());
}

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

pub async fn winit_run(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.spawn_local(async {
        loop {
            let mut message: (Option<WindowId>, EventLoopMessage) = (None, EventLoopMessage::None);

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
                            message = (Some(window_id), EventLoopMessage::CloseRequested);
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
                                EventLoopMessage::CursorMoved(position.x, position.y),
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
                                    EventLoopMessage::MouseButtton(button, pressed),
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
                                (Some(window_id), EventLoopMessage::KeyCode(keycode, pressed));
                        }
                        _ => {}
                    }
                });
            });

            if !EVENT_LOOP_SENDER.is_closed() {
                EVENT_LOOP_SENDER.send(message).unwrap();
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
                    drop(window);
                    continue;
                }
            }

            window.window.request_redraw();
            drop(window);

            let thread = inner_lua.create_thread(callback).unwrap();
            inner_lua
                .as_ref()
                .push_thread_back(thread, message.1)
                .unwrap();

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    TableBuilder::new(lua)?
        .with_function("stop", move |_lua: &Lua, _: ()| {
            if shutdown_tx.is_closed() {
                return Ok(());
            }

            shutdown_tx.send(true).unwrap();
            Ok(())
        })?
        .build_readonly()
}
