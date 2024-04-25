use crate::lune::builtins::winit::{config::EventLoopMessage, webview::config::LuaWebView};
use mlua::prelude::*;
use std::rc::Rc;
use winit::window::Window;

// LuaWindow
pub struct LuaWindow {
    pub sender: tokio::sync::watch::Sender<EventLoopMessage>,
    pub window: Window,
    pub webview: Option<Rc<LuaWebView>>,
}

impl LuaUserData for LuaWindow {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("close", |_lua: &Lua, this: &mut Self, _: ()| {
            this.window.set_visible(false);
            Ok(())
        });

        methods.add_method(
            "get_webview",
            |lua: &Lua, this: &Self, _: ()| -> LuaResult<LuaValue> {
                if this.webview.is_some() {
                    let clone = this.webview.clone().unwrap();
                    Ok(LuaValue::UserData(lua.create_userdata(clone)?))
                } else {
                    Ok(LuaValue::Nil)
                }
            },
        );

        methods.add_meta_method(
            "__eq",
            |_lua: &Lua, this: &Self, other: LuaUserDataRef<'lua, Self>| {
                let result = this.window.id() == other.window.id();
                Ok(result)
            },
        );
    }
}
