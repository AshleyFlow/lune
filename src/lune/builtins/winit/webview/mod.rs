pub mod config;
pub mod input;

use self::{
    config::{LuaWebView, LuaWebViewConfig, LuaWebViewScript},
    input::{config::LuaWebViewMessage, JAVASCRIPT_API},
};
use super::{config::EventLoopMessage, window::config::LuaWindow, EVENT_LOOP};
use mlua::prelude::*;
use std::rc::Rc;
use wry::WebViewBuilder;

pub fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let field2 = values.get(1).expect("Parameter 2 is missing");

    let config = LuaWebViewConfig::from_lua(field2.clone(), lua)?;

    if let Some(window) = field1.as_userdata() {
        let mut window = window.borrow_mut::<LuaWindow>()?;
        let mut webview_builder = WebViewBuilder::new(&window.window).with_url(config.url);

        let mut init_script = LuaWebViewScript::new();
        init_script.write(JAVASCRIPT_API);
        init_script.extract_from_option(config.init_script);

        let window_id = window.window.id();

        webview_builder = EVENT_LOOP.with(|event_loop| {
            let event_loop_proxy = event_loop.borrow().create_proxy();

            webview_builder.with_ipc_handler(move |request| {
                let body = request.body().as_str();
                let message: Result<LuaWebViewMessage, serde_json::Error> =
                    serde_json::from_str(body);

                if let Ok(message) = message {
                    if let Some(position) = message.position {
                        let msg = (
                            window_id,
                            EventLoopMessage::CursorMoved(position.x, position.y),
                        );

                        event_loop_proxy.send_event(msg).unwrap();
                    } else if let Some(mousebutton) = message.mousebutton {
                        let presed = message.pressed.unwrap();

                        let msg = (
                            window_id,
                            EventLoopMessage::MouseButtton(mousebutton, presed),
                        );

                        event_loop_proxy.send_event(msg).unwrap();
                    } else if let Some(keycode) = message.keycode {
                        let presed = message.pressed.unwrap();

                        let msg = (window_id, EventLoopMessage::KeyCode(keycode, presed));

                        event_loop_proxy.send_event(msg).unwrap();
                    }
                } else {
                    println!("custom user message.");
                }
            })
        });

        webview_builder = webview_builder.with_initialization_script(&init_script.read());

        let webview = webview_builder.build().unwrap();
        let lua_webview = LuaWebView { webview };
        let rc_lua_webview = Rc::new(lua_webview);

        window.webview = Some(Rc::clone(&rc_lua_webview));

        lua.create_userdata(rc_lua_webview)
    } else {
        Err(LuaError::RuntimeError("".into()))
    }
}
