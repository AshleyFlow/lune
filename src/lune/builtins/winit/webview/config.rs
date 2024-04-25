use mlua::prelude::*;
use wry::WebView;

// LuaWebView
pub struct LuaWebView {
    pub webview: WebView,
}

impl LuaUserData for LuaWebView {}
