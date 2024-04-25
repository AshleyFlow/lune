use mlua::prelude::*;
use wry::WebView;

// LuaWebView
pub struct LuaWebView {
    pub webview: WebView,
}

impl LuaUserData for LuaWebView {}

// LuaWebViewConfig
pub struct LuaWebViewConfig {
    pub init_script: Result<String, LuaError>,
    pub url: String,
}

impl<'lua> FromLua<'lua> for LuaWebViewConfig {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Some(config) = value.as_table() {
            Ok(Self {
                init_script: config.get("init_script"),
                url: config
                    .get("url")
                    .expect("WebViewConfig is missing 'url' property"),
            })
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "table",
                message: None,
            })
        }
    }
}
