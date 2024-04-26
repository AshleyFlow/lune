use mlua::prelude::*;
use wry::WebView;

use crate::lune::builtins::serde::encode_decode::{EncodeDecodeConfig, EncodeDecodeFormat};

// LuaWebView
pub struct LuaWebView {
    pub webview: WebView,
}

impl LuaUserData for LuaWebView {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "evaluate",
            |lua: &Lua, this: &Self, script: String| async move {
                let (result_tx, mut result_rx) = tokio::sync::watch::channel("null".to_string());

                this.webview
                    .evaluate_script_with_callback(script.as_str(), move |res| {
                        result_tx.send(res.clone()).unwrap();
                    })
                    .unwrap();

                if result_rx.changed().await.is_ok() {
                    let borrowed = result_rx.borrow_and_update();
                    let config = EncodeDecodeConfig::from(EncodeDecodeFormat::Json);
                    config.deserialize_from_string(lua, borrowed.as_str().into())
                } else {
                    Ok(LuaValue::Nil)
                }
            },
        )
    }
}

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
