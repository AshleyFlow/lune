use crate::lune::builtins::winit::config::LuaDimension;
use serde::Deserialize;

// LuaWebViewMessage
#[derive(Deserialize, Debug)]
pub struct LuaWebViewMessage {
    pub __internal: bool,
    pub position: Option<LuaDimension>,
    pub mousebutton: Option<String>,
    pub keycode: Option<String>,
    pub pressed: Option<bool>,
}
