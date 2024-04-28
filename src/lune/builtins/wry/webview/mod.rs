pub mod config;
pub mod input;
mod request;
mod response;

use self::{
    config::{LuaWebView, LuaWebViewConfig, LuaWebViewScript},
    input::{config::LuaWebViewMessage, JAVASCRIPT_API},
    request::LuaRequest,
    response::LuaResponse,
};
use super::{window::config::LuaWindow, EVENT_LOOP};
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use std::rc::{Rc, Weak};
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
        let mut webview_builder = WebViewBuilder::new(&window.window);

        let mut init_script = LuaWebViewScript::new();
        init_script.write(JAVASCRIPT_API);
        init_script.extract_from_option(config.init_script);

        if let Some(html) = config.html {
            webview_builder = webview_builder.with_html(html);
        } else if let Some(url) = config.url {
            webview_builder = webview_builder.with_url(url);
        }

        let incomplete_custom_protocol_config = {
            (config.custom_protocol_name.is_some() & config.custom_protocol_handler.is_none())
                | (config.custom_protocol_name.is_none() & config.custom_protocol_handler.is_some())
        };

        if incomplete_custom_protocol_config {
            return Err(LuaError::RuntimeError("config for custom_protocol is incomplete, both custom_protocol_name and custom_protocol_handler must be present".into()));
        }

        if let Some(custom_protocol_name) = config.custom_protocol_name {
            let custom_protocol_fn_key = Rc::new(config.custom_protocol_handler.unwrap());

            let inner_lua = lua
                .app_data_ref::<Weak<Lua>>()
                .expect("Missing weak lua ref")
                .upgrade()
                .expect("Lua was dropped unexpectedly");

            webview_builder = webview_builder.with_asynchronous_custom_protocol(
                custom_protocol_name,
                move |request, responder| {
                    let outter_lua = inner_lua
                        .app_data_ref::<Weak<Lua>>()
                        .expect("Missing weak lua ref")
                        .upgrade()
                        .expect("Lua was dropped unexpectedly");

                    let custom_protocol_fn_key = custom_protocol_fn_key.clone();

                    inner_lua.as_ref().spawn_local(async move {
                        let (head, body) = request.into_parts();
                        let lua_req = LuaRequest { head, body };

                        let lua = outter_lua.as_ref();
                        let lua_req_table = lua_req.into_lua_table(&outter_lua).unwrap();

                        let custom_protocol_fn = lua
                            .registry_value::<LuaFunction>(&custom_protocol_fn_key)
                            .unwrap();

                        let thread = lua.create_thread(custom_protocol_fn).unwrap();
                        let thread_id = lua.push_thread_back(thread, lua_req_table).unwrap();

                        lua.track_thread(thread_id);
                        lua.wait_for_thread(thread_id).await;

                        let lua_res_table =
                            outter_lua.get_thread_result(thread_id).unwrap().unwrap();
                        let lua_res =
                            LuaResponse::from_lua_multi(lua_res_table, &outter_lua).unwrap();

                        responder.respond(lua_res.into_response().unwrap());
                    });
                },
            );
        }

        let window_id = window.window.id();
        let ipc_sender = tokio::sync::watch::Sender::new(String::new());
        let inner_ipc_sender = ipc_sender.clone();

        webview_builder = EVENT_LOOP.with(|event_loop| {
            let event_loop_proxy = event_loop.borrow().create_proxy();

            webview_builder.with_ipc_handler(move |request| {
                let body = request.body().as_str();
                let message: Result<LuaWebViewMessage, serde_json::Error> =
                    serde_json::from_str(body);

                if let Ok(message) = message {
                    let msg = message.into_eventloop_message().unwrap();
                    let send = (window_id, msg);
                    event_loop_proxy.send_event(send).unwrap();
                } else if inner_ipc_sender.receiver_count() > 0 {
                    inner_ipc_sender.send(body.to_string()).unwrap();
                }
            })
        });

        webview_builder = webview_builder.with_initialization_script(&init_script.read());

        let webview = webview_builder.build().unwrap();
        let lua_webview = LuaWebView {
            webview,
            ipc_sender,
        };

        let rc_lua_webview = Rc::new(lua_webview);

        window.webview = Some(Rc::clone(&rc_lua_webview));

        lua.create_userdata(rc_lua_webview)
    } else {
        Err(LuaError::RuntimeError("".into()))
    }
}
