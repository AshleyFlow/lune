pub mod config;

use self::config::{LuaWebView, LuaWebViewConfig};
use super::window::config::LuaWindow;
use mlua::prelude::*;
use std::rc::Rc;
use wry::WebViewBuilder;

pub fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let field2 = values.get(1).expect("Parameter 2 is missing");

    let config = LuaWebViewConfig::from_lua(field2.clone(), lua)?;

    if let Some(window) = field1.as_userdata() {
        let mut window = window.borrow_mut::<LuaWindow>()?;
        let mut webview_builder = WebViewBuilder::new(&window.window).with_url(config.url);

        if let Ok(init_script) = config.init_script.clone() {
            webview_builder = webview_builder.with_initialization_script(init_script.as_str());
        }

        let webview = webview_builder.build().unwrap();
        if let Ok(init_script) = config.init_script {
            webview.evaluate_script(init_script.as_str()).unwrap();
        }

        let lua_webview = LuaWebView { webview };
        let rc_lua_webview = Rc::new(lua_webview);

        window.webview = Some(Rc::clone(&rc_lua_webview));

        lua.create_userdata(rc_lua_webview)
    } else {
        Err(LuaError::RuntimeError("".into()))
    }
}
