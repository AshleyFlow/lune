use crate::lune::{
    builtins::winit::{config::EventLoopMessage, webview::config::LuaWebView},
    util::TableBuilder,
};
use mlua::prelude::*;
use std::rc::Rc;
use tao::window::Window;

// LuaWindow
pub struct LuaWindow {
    pub sender: tokio::sync::watch::Sender<EventLoopMessage>,
    pub window: Window,
    pub webview: Option<Rc<LuaWebView>>,
}

impl LuaUserData for LuaWindow {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("webview", |lua: &Lua, this: &Self| {
            if this.webview.is_some() {
                let clone = this.webview.clone().unwrap();
                Ok(LuaValue::UserData(lua.create_userdata(clone)?))
            } else {
                Ok(LuaValue::Nil)
            }
        });

        fields.add_field_method_get("size", |lua: &Lua, this: &Self| {
            TableBuilder::new(lua)?
                .with_value("x", this.window.inner_size().width)?
                .with_value("y", this.window.inner_size().height)?
                .build_readonly()
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("close", |_lua: &Lua, this: &mut Self, _: ()| {
            this.window.set_visible(false);
            Ok(())
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

// LuaWindowConfig
pub struct LuaWindowConfig {
    pub title: String,
}

impl<'lua> FromLua<'lua> for LuaWindowConfig {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Some(config) = value.as_table() {
            Ok(Self {
                title: config.get("title").unwrap_or("Lune WebView".to_string()),
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
