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

        methods.add_method("has_webview", |_lua: &Lua, this: &Self, _: ()| {
            Ok(this.webview.is_some())
        });

        methods.add_method("get_webview", |_lua: &Lua, this: &Self, _: ()| {
            if this.webview.is_some() {
                Ok(this.webview.clone().unwrap())
            } else {
                Err(LuaError::RuntimeError("Window does not have a WebView, check with window:has_webview() before using window:get_webview()".into()))
            }
        });

        methods.add_meta_method(
            "__eq",
            |_lua: &Lua, this: &Self, other: LuaUserDataRef<'lua, Self>| {
                let result = this.window.id() == other.window.id();
                Ok(result)
            },
        );
    }
}
