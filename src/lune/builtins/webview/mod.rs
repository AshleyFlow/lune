mod enums;
mod window;

use crate::lune::util::TableBuilder;
use enums::LuaWindowEvent;
use mlua::prelude::*;
use std::{cell::RefCell, collections::HashMap, time::Duration};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Window, WindowId},
};
use wry::WebView;

use self::window::LuaWindow;

thread_local! {
    pub static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
    pub static WINDOWS: RefCell<HashMap<WindowId, Window>> = RefCell::new(HashMap::new());

    // will panic if this gets accessed in another thread
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::new().build().unwrap());
}

pub struct LuaWindowId(WindowId);
impl LuaUserData for LuaWindowId {}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    let events = TableBuilder::new(lua)?
        .with_value("Nothing", LuaWindowEvent::Nothing)?
        .with_value("Exit", LuaWindowEvent::Exit)?
        .build_readonly()?;

    TableBuilder::new(lua)?
        .with_value("events", events)?
        .with_function("new", LuaWindow::new)?
        .with_async_function("event_loop", window_event_loop)?
        .build_readonly()
}

async fn window_event_loop<'lua>(_lua: &'lua Lua, callback: LuaFunction<'lua>) -> LuaResult<()> {
    loop {
        let mut callback_args: (Option<LuaWindowId>, LuaWindowEvent) =
            (None, LuaWindowEvent::Nothing);
        EVENT_LOOP.with(|event_loop| {
            event_loop
                .borrow_mut()
                .pump_events(Some(Duration::ZERO), |event, elwt| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                        ..
                    } => {
                        let lua_window_id = LuaWindowId(window_id);
                        callback_args = (Some(lua_window_id), LuaWindowEvent::Exit);
                        elwt.exit();
                    }
                    Event::AboutToWait => {}
                    _ => (),
                });
        });

        tokio::time::sleep(Duration::from_millis(16)).await;

        let lua_value = callback.call::<_, LuaValue>(callback_args).unwrap();

        if lua_value.is_boolean() && lua_value.as_boolean().unwrap() {
            break;
        }
    }

    Ok(())
}
