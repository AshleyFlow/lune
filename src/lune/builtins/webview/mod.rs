mod enums;
mod window;

use crate::lune::util::TableBuilder;
use enums::LuaWindowEvent;
use mlua::prelude::*;
use std::{cell::RefCell, collections::HashMap};
use winit::window::{Window, WindowId};
use wry::WebView;

use self::window::LuaWindow;

thread_local! {
    pub static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
    pub static WINDOWS: RefCell<HashMap<WindowId, Window>> = RefCell::new(HashMap::new());
}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    let events = TableBuilder::new(lua)?
        .with_value("Nothing", LuaWindowEvent::Nothing)?
        .with_value("Redraw", LuaWindowEvent::Redraw)?
        .with_value("Exit", LuaWindowEvent::Exit)?
        .build_readonly()?;

    TableBuilder::new(lua)?
        .with_value("events", events)?
        .with_function("new", LuaWindow::new)?
        .build_readonly()
}
