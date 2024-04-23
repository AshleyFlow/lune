use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaWindowEvent {
    Nothing,
    Exit,
}

impl LuaUserData for LuaWindowEvent {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__eq", |_, this, other| Ok(*this == other));
    }
}

impl<'lua> FromLua<'lua> for LuaWindowEvent {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let userdata = value.as_userdata();

        if let Some(userdata) = userdata {
            if let Ok(this) = userdata.borrow::<Self>() {
                return Ok(*this);
            } else {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "LuaWindowEvent",
                    message: None,
                });
            }
        }

        Err(LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "userdata",
            message: None,
        })
    }
}
