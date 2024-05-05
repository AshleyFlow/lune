use mlua::prelude::*;
use serde::Deserialize;

// LuaDimension
#[derive(Deserialize, Default, Debug)]
pub struct LuaDimension {
    pub x: f64,
    pub y: f64,
}

impl<'lua> FromLua<'lua> for LuaDimension {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Some(t) = value.as_table() {
            let x: f64 = t.get("x").unwrap();
            let y: f64 = t.get("y").unwrap();
            Ok(Self { x, y })
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "Dimension",
                message: None,
            })
        }
    }
}

// LuaRGBA
#[derive(Deserialize, Default, Debug)]
pub struct LuaRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl<'lua> FromLua<'lua> for LuaRGBA {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Some(t) = value.as_table() {
            let r: u8 = t.get("r").unwrap();
            let g: u8 = t.get("g").unwrap();
            let b: u8 = t.get("b").unwrap();
            let a: u8 = t.get("a").unwrap();
            Ok(Self { r, g, b, a })
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "RGBA",
                message: None,
            })
        }
    }
}
