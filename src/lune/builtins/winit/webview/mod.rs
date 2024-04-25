pub mod config;

use self::config::LuaWebView;
use super::window::config::LuaWindow;
use mlua::prelude::*;
use std::rc::Rc;
use wry::WebViewBuilder;

pub fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");

    if let Some(window) = field1.as_userdata() {
        let mut window = window.borrow_mut::<LuaWindow>()?;
        let webview = WebViewBuilder::new(&window.window)
            .with_url("https://roblox.com/")
            .build()
            .unwrap();

        let lua_webview = LuaWebView { webview };
        let rc_lua_webview = Rc::new(lua_webview);

        window.webview = Some(Rc::clone(&rc_lua_webview));

        lua.create_userdata(rc_lua_webview)
    } else {
        Err(LuaError::RuntimeError("".into()))
    }
}
