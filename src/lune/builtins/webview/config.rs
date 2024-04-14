use mlua::prelude::*;

pub struct WebviewConfig<'a> {
    pub exit: Option<LuaFunction<'a>>,
    pub url: LuaString<'a>,
}

impl<'lua> FromLua<'lua> for WebviewConfig<'lua> {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::Table(tab) = &value {
            let exit: Option<LuaFunction> = tab.get("exit")?;
            let url: Option<LuaString> = tab.get("url")?;

            if url.is_none() {
                return Err(LuaError::RuntimeError(
                    "Invalid webview config - missing 'url'".to_owned(),
                ));
            }

            Ok(Self {
                exit,
                url: url.unwrap(),
            })
        } else {
            // Anything else is invalid
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "WebviewConfig",
                message: Some(format!(
                    "Invalid webview config - expected table, got {}",
                    value.type_name()
                )),
            })
        }
    }
}
