use crate::lune::util::TableBuilder;
use mlua::prelude::*;

// EventLoopHandle
pub enum EventLoopHandle {
    Break,
}

impl LuaUserData for EventLoopHandle {}

// EventLoopMessage
#[derive(Clone, Copy, PartialEq)]
pub enum EventLoopMessage {
    CloseRequested,
    None,
}

impl EventLoopMessage {
    pub fn create_lua_table(lua: &Lua) -> LuaResult<LuaTable> {
        TableBuilder::new(lua)?
            .with_value("CloseRequested", Self::CloseRequested)?
            .with_value("None", Self::None)?
            .build_readonly()
    }
}

impl LuaUserData for EventLoopMessage {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method("__eq", |_lua, this: &Self, other: Self| Ok(*this == other));
    }
}

impl<'lua> FromLua<'lua> for EventLoopMessage {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        if let Some(userdata) = value.as_userdata() {
            let this = userdata.borrow::<Self>()?;
            Ok(*this)
        } else {
            Err(LuaError::RuntimeError("".into()))
        }
    }
}
