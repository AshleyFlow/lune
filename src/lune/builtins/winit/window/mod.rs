mod config;

use self::config::LuaWindow;
use super::EVENT_LOOP;
use mlua::prelude::*;
use winit::window::WindowBuilder;

pub fn create(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let window = EVENT_LOOP.with(|event_loop| {
        let event_loop = event_loop.borrow_mut();

        WindowBuilder::new().build(&event_loop).unwrap()
    });

    let lua_window = LuaWindow {
        sender: tokio::sync::watch::Sender::new(super::config::EventLoopMessage::None),
        window,
    };

    lua.create_userdata(lua_window)
}
