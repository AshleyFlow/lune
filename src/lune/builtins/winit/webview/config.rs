use crate::lune::builtins::serde::encode_decode::{EncodeDecodeConfig, EncodeDecodeFormat};
use mlua::prelude::*;
use wry::WebView;

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
    pub init_script: Option<String>,
    pub url: String,
    pub mimic_input: Option<bool>,
}

impl<'lua> FromLua<'lua> for LuaWebViewConfig {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Some(config) = value.as_table() {
            Ok(Self {
                init_script: config.get("init_script").ok(),
                url: config
                    .get("url")
                    .expect("WebViewConfig is missing 'url' property"),
                mimic_input: config.get("mimic_input").ok(),
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

// LuaWebViewScript
pub struct LuaWebViewScript {
    raw: String,
}

impl LuaWebViewScript {
    pub fn new() -> Self {
        Self { raw: String::new() }
    }

    pub fn read(self) -> Box<str> {
        self.raw.as_str().into()
    }

    pub fn write(&mut self, string: &str) {
        self.raw += string;
        self.raw.push(';');
    }

    pub fn extract_from_option<T>(&mut self, option: Option<T>)
    where
        T: AsRef<str> + std::default::Default,
    {
        let binding = option.unwrap_or_default();
        let string = binding.as_ref();
        self.raw += string;
        self.raw.push(';');
    }
}
