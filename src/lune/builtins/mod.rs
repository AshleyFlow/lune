use std::str::FromStr;

use mlua::prelude::*;

mod datetime;
mod fs;
mod luau;
mod net;
mod process;
mod serde;
mod stdio;
mod task;
mod window;

#[cfg(feature = "roblox")]
mod roblox;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LuneBuiltin {
    DateTime,
    Fs,
    Luau,
    Net,
    Task,
    Process,
    Serde,
    Stdio,
    Window,
    #[cfg(feature = "roblox")]
    Roblox,
}

impl LuneBuiltin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::DateTime => "datetime",
            Self::Fs => "fs",
            Self::Luau => "luau",
            Self::Net => "net",
            Self::Task => "task",
            Self::Process => "process",
            Self::Serde => "serde",
            Self::Stdio => "stdio",
            Self::Window => "window",
            #[cfg(feature = "roblox")]
            Self::Roblox => "roblox",
        }
    }

    pub fn create<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaMultiValue<'lua>> {
        let res = match self {
            Self::DateTime => datetime::create(lua),
            Self::Fs => fs::create(lua),
            Self::Luau => luau::create(lua),
            Self::Net => net::create(lua),
            Self::Task => task::create(lua),
            Self::Process => process::create(lua),
            Self::Serde => serde::create(lua),
            Self::Stdio => stdio::create(lua),
            Self::Window => window::create(lua),
            #[cfg(feature = "roblox")]
            Self::Roblox => roblox::create(lua),
        };
        match res {
            Ok(v) => v.into_lua_multi(lua),
            Err(e) => Err(e.context(format!(
                "Failed to create builtin library '{}'",
                self.name()
            ))),
        }
    }
}

impl FromStr for LuneBuiltin {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "datetime" => Ok(Self::DateTime),
            "fs" => Ok(Self::Fs),
            "luau" => Ok(Self::Luau),
            "net" => Ok(Self::Net),
            "task" => Ok(Self::Task),
            "process" => Ok(Self::Process),
            "serde" => Ok(Self::Serde),
            "stdio" => Ok(Self::Stdio),
            "window" => Ok(Self::Window),
            #[cfg(feature = "roblox")]
            "roblox" => Ok(Self::Roblox),
            _ => Err(format!("Unknown builtin library '{s}'")),
        }
    }
}
