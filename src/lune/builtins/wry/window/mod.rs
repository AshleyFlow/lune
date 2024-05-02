pub mod config;

use self::config::{LuaWindow, LuaWindowConfig};
use super::EVENT_LOOP;
use mlua::prelude::*;
use tao::{
    dpi::{PhysicalSize, Size},
    window::WindowBuilder,
};

pub fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let config = LuaWindowConfig::from_lua(field1.clone(), lua)?;

    let window = EVENT_LOOP.with(|event_loop| {
        let event_loop = event_loop.borrow_mut();
        let mut window = WindowBuilder::new().with_title(config.title);

        window = if let Some(size) = config.size {
            window.with_inner_size(Size::Physical(PhysicalSize::new(size.0, size.1)))
        } else {
            window
        };

        window.build(&event_loop).into_lua_err()
    });

    let lua_window = LuaWindow {
        sender: tokio::sync::watch::Sender::new(super::config::EventLoopMessage::none()),
        window: window?,
        webview: None,
    };

    lua.create_userdata(lua_window)
}
