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
#[derive(Debug, Default, Clone)]
pub struct EventLoopMessage {
    event_type: String,
    mousebutton: Option<String>,
    keycode: Option<String>,
    pressed: Option<bool>,
    position: Option<(f64, f64)>,
}

impl EventLoopMessage {
    pub fn close_requested() -> Self {
        Self {
            event_type: "CloseRequested".into(),
            ..Default::default()
        }
    }

    pub fn mouse_button(mousebutton: String, pressed: bool) -> Self {
        Self {
            event_type: "MouseButton".into(),
            mousebutton: Some(mousebutton),
            pressed: Some(pressed),
            ..Default::default()
        }
    }

    pub fn keycode(keycode: String, pressed: bool) -> Self {
        Self {
            event_type: "KeyCode".into(),
            keycode: Some(keycode),
            pressed: Some(pressed),
            ..Default::default()
        }
    }

    pub fn cursor_moved(x: f64, y: f64) -> Self {
        Self {
            event_type: "CursorMoved".into(),
            position: Some((x, y)),
            ..Default::default()
        }
    }

    pub fn none() -> Self {
        Self {
            event_type: "None".into(),
            ..Default::default()
        }
    }
}

impl LuaUserData for EventLoopMessage {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |_lua: &Lua, this: &Self, _: ()| {
            Ok(this.event_type.clone())
        });
    }

    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("event_type", |_lua: &Lua, this: &Self| {
            Ok(this.event_type.clone())
        });

        fields.add_field_method_get("mousebutton", |lua: &Lua, this: &Self| {
            if let Some(mousebutton) = &this.mousebutton {
                mousebutton.clone().into_lua(lua)
            } else {
                Ok(LuaValue::Nil)
            }
        });

        fields.add_field_method_get("keycode", |lua: &Lua, this: &Self| {
            if let Some(keycode) = &this.keycode {
                keycode.clone().into_lua(lua)
            } else {
                Ok(LuaValue::Nil)
            }
        });

        fields.add_field_method_get("pressed", |lua: &Lua, this: &Self| {
            if let Some(pressed) = &this.pressed {
                Ok(pressed.into_lua(lua)?)
            } else {
                Ok(LuaValue::Nil)
            }
        });

        fields.add_field_method_get("position", |lua: &Lua, this: &Self| {
            if let Some(position) = &this.position {
                TableBuilder::new(lua)?
                    .with_value("x", position.0.into_lua(lua)?)?
                    .with_value("y", position.1.into_lua(lua)?)?
                    .build_readonly()?
                    .into_lua(lua)
            } else {
                Ok(LuaValue::Nil)
            }
        });
    }
}
