use std::cell::RefCell;
use winit::event_loop::{EventLoop, EventLoopBuilder};
use crate::lune::util::TableBuilder;
use mlua::prelude::*;

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::new().build().unwrap());
}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?.build_readonly()
}
