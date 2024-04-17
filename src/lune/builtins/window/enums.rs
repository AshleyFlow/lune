use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaWindowEvent {
    Nothing,
    Redraw,
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
            return Ok(*userdata.borrow::<Self>().unwrap());
        }

        Err(LuaError::RuntimeError(
            "Provided value is not a LuaWindowEvent".into(),
        ))
    }
}
