use crate::lune::builtins::winit::config::EventLoopMessage;
use mlua::prelude::*;
use winit::window::Window;

// LuaWindow
pub struct LuaWindow {
    pub sender: tokio::sync::watch::Sender<EventLoopMessage>,
    pub window: Window,
}

impl LuaUserData for LuaWindow {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            "__eq",
            |_lua: &Lua, this: &Self, other: LuaUserDataRef<'lua, Self>| {
                let result = this.window.id() == other.window.id();
                Ok(result)
            },
        );
    }
}
