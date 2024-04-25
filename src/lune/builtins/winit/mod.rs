use std::{cell::RefCell, time::Duration};

use crate::lune::util::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{IntoLuaThread, LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use winit::{event_loop::EventLoopBuilder, platform::pump_events::EventLoopExtPumpEvents};

// thread_local! {
//     pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::new().build().unwrap());
// }

pub static EVENT_LOOP_SENDER: Lazy<tokio::sync::watch::Sender<()>> =
    Lazy::new(|| tokio::sync::watch::Sender::new(()));

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    lua.spawn_local(async {
        let mut event_loop = EventLoopBuilder::new().build().unwrap();

        loop {
            let mut message: () = ();

            event_loop.pump_events(Some(Duration::ZERO), |event, elwt| match event {
                winit::event::Event::WindowEvent { window_id, event } => {
                    message = ();
                }
                _ => {
                    message = ();
                }
            });

            if EVENT_LOOP_SENDER.receiver_count() > 0 {
                EVENT_LOOP_SENDER.send(message).unwrap();
            }

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    TableBuilder::new(lua)?
        .with_async_function("eventLoop", winit_event_loop)?
        .build_readonly()
}

pub async fn winit_event_loop(lua: &Lua, callback: LuaFunction<'_>) -> LuaResult<()> {
    let loop_function =
        lua.create_async_function(|inner_lua, inner_callback: LuaFunction<'_>| async move {
            let mut listener = EVENT_LOOP_SENDER.subscribe();

            loop {
                let changed = listener.changed().await;

                if changed.is_ok() {
                    let message = *listener.borrow_and_update();
                    inner_callback.call::<_, ()>(message)?;
                }

                tokio::time::sleep(Duration::ZERO).await;
            }

            Ok(())
        })?;

    let loop_thread = lua.create_thread(loop_function)?;
    lua.push_thread_back(loop_thread, callback)?;

    Ok(())
}
