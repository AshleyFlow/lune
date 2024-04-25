mod config;

use crate::lune::util::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use std::time::Duration;
use winit::{event_loop::EventLoopBuilder, platform::pump_events::EventLoopExtPumpEvents};

use self::config::{EventLoopHandle, EventLoopMessage};

pub static EVENT_LOOP_SENDER: Lazy<tokio::sync::watch::Sender<EventLoopMessage>> =
    Lazy::new(|| tokio::sync::watch::Sender::new(EventLoopMessage::None));

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    let events = EventLoopMessage::create_lua_table(lua)?;

    TableBuilder::new(lua)?
        .with_value("events", events)?
        .with_async_function("event_loop", winit_event_loop)?
        .with_async_function("run", winit_run)?
        .build_readonly()
}

pub async fn winit_run(lua: &Lua, _: ()) -> LuaResult<()> {
    lua.spawn_local(async {
        let mut event_loop = EventLoopBuilder::new().build().unwrap();

        loop {
            let mut message: EventLoopMessage = EventLoopMessage::None;

            event_loop.pump_events(Some(Duration::ZERO), |event, elwt| {
                if let winit::event::Event::WindowEvent {
                    window_id,
                    event: winit::event::WindowEvent::CloseRequested,
                } = event
                {
                    message = EventLoopMessage::CloseRequested;
                }
            });

            if EVENT_LOOP_SENDER.receiver_count() > 0 {
                EVENT_LOOP_SENDER.send(message).unwrap();
            } else {
                break;
            }

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    Ok(())
}

pub async fn winit_event_loop(lua: &Lua, callback: LuaFunction<'_>) -> LuaResult<()> {
    let loop_function =
        lua.create_async_function(|inner_lua, inner_callback: LuaFunction<'_>| async move {
            let mut listener = EVENT_LOOP_SENDER.subscribe();

            loop {
                let changed = listener.changed().await;

                if changed.is_ok() {
                    let message = *listener.borrow_and_update();
                    let callback_result = inner_callback
                        .call_async::<_, LuaValue>((EventLoopHandle::Break, message))
                        .await?;

                    if let Some(userdata) = callback_result.as_userdata() {
                        if let Ok(handle) = userdata.borrow::<EventLoopHandle>() {
                            match *handle {
                                EventLoopHandle::Break => break,
                            }
                        }
                    }
                }

                tokio::time::sleep(Duration::ZERO).await;
            }

            Ok(())
        })?;

    let loop_thread = lua.create_thread(loop_function)?;
    lua.push_thread_back(loop_thread, callback)?;

    Ok(())
}
