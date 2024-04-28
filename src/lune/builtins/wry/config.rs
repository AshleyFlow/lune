use crate::lune::util::TableBuilder;
use mlua::prelude::*;
use serde::Deserialize;

// LuaDimension
#[derive(Deserialize, Default, Debug)]
pub struct LuaDimension {
    pub x: f64,
    pub y: f64,
}

// EventLoopMessage
#[derive(Clone, Debug, PartialEq)]
pub enum EventLoopMessage {
    CloseRequested,
    MouseButtton(String, bool),
    KeyCode(String, bool),
    CursorMoved(f64, f64),
    None,
}

impl EventLoopMessage {
    pub fn create_lua_table(lua: &Lua) -> LuaResult<LuaTable> {
        TableBuilder::new(lua)?
            .with_value("CloseRequested", Self::CloseRequested)?
            .with_value("MouseButton", Self::MouseButtton("".into(), false))?
            .with_value("KeyCode", Self::KeyCode("".into(), false))?
            .with_value("CursorMoved", Self::CursorMoved(0.0, 0.0))?
            .with_value("None", Self::None)?
            .build_readonly()
    }
}

impl LuaUserData for EventLoopMessage {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mousebutton", |lua: &Lua, this: &Self| match this {
            EventLoopMessage::MouseButtton(button, _) => Ok(button.clone().into_lua(lua)?),
            _ => Ok(LuaValue::Nil),
        });

        fields.add_field_method_get("keycode", |lua: &Lua, this: &Self| match this {
            EventLoopMessage::KeyCode(keycode, _) => Ok(keycode.clone().into_lua(lua)?),
            _ => Ok(LuaValue::Nil),
        });

        fields.add_field_method_get("pressed", |_lua: &Lua, this: &Self| match this {
            EventLoopMessage::MouseButtton(_, pressed, ..) => Ok(LuaValue::Boolean(*pressed)),
            EventLoopMessage::KeyCode(_, pressed, ..) => Ok(LuaValue::Boolean(*pressed)),
            _ => Ok(LuaValue::Nil),
        });

        fields.add_field_method_get("position", |lua: &Lua, this: &Self| match this {
            EventLoopMessage::CursorMoved(x, y) => Ok(LuaValue::Table(
                TableBuilder::new(lua)?
                    .with_value("x", x.into_lua(lua)?)?
                    .with_value("y", y.into_lua(lua)?)?
                    .build_readonly()?,
            )),
            _ => Ok(LuaValue::Nil),
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            "__eq",
            |_lua, this: &Self, other: LuaUserDataRef<'lua, Self>| {
                Ok(matches!(
                    (this, other.clone()),
                    (Self::CloseRequested, Self::CloseRequested)
                        | (Self::MouseButtton(..), Self::MouseButtton(..))
                        | (Self::KeyCode(..), Self::KeyCode(..))
                        | (Self::CursorMoved(..), Self::CursorMoved(..))
                        | (Self::None, Self::None)
                ))
            },
        );

        methods.add_meta_method(
            "__tostring",
            |_lua: &Lua, this: &Self, _: ()| -> LuaResult<String> {
                Ok(match this {
                    EventLoopMessage::CloseRequested => "CloseRequested".to_string(),
                    EventLoopMessage::MouseButtton(button, pressed) => {
                        format!("MouseButton({}:{})", button, pressed)
                    }
                    EventLoopMessage::KeyCode(keycode, pressed) => {
                        format!("KeyCode({}:{})", keycode, pressed)
                    }
                    EventLoopMessage::CursorMoved(x, y) => {
                        format!("CursorMoved(x: {}, y: {})", x, y)
                    }
                    EventLoopMessage::None => "None".to_string(),
                })
            },
        );
    }
}
