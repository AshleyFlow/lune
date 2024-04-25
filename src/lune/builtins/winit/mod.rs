use std::{borrow::BorrowMut, cell::RefCell, time::Duration};

use crate::lune::util::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use once_cell::sync::Lazy;
use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::pump_events::EventLoopExtPumpEvents,
};

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::new().build().unwrap());
}

pub static EVENT_LOOP_SENDER: Lazy<tokio::sync::watch::Sender<()>> =
    Lazy::new(|| tokio::sync::watch::Sender::new(()));

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    lua.spawn_local(async {
        loop {
            EVENT_LOOP.with(|event_loop| {
                let mut message: () = ();

                event_loop.borrow_mut().pump_events(
                    Some(Duration::ZERO),
                    |event, elwt| match event {
                        winit::event::Event::WindowEvent { window_id, event } => {
                            message = ();
                        }
                        _ => {
                            message = ();
                        }
                    },
                );

                if EVENT_LOOP_SENDER.receiver_count() > 0 {
                    EVENT_LOOP_SENDER.send(message).unwrap();
                }
            });

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    TableBuilder::new(lua)?
        .with_function("eventLoop", winit_event_loop)?
        .build_readonly()
}

pub fn winit_event_loop(lua: &Lua, callback: LuaFunction<'_>) -> LuaResult<()> {
    lua.spawn_local(async {
        let mut listener = EVENT_LOOP_SENDER.subscribe();

        loop {
            let changed = listener.changed().await;

            if changed.is_ok() {
                println!("Received new signal");
            }

            tokio::time::sleep(Duration::ZERO).await;
        }
    });

    Ok(())
}
