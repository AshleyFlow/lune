#![allow(clippy::cargo_common_metadata)]

use context::GlobalsContextBuilder;
use library::inject_lune_standard_libraries;
use mlua::prelude::*;

pub mod context;
mod global;
mod globals;
mod library;
mod luaurc;

pub use self::global::LuneStandardGlobal;
pub use self::globals::version::set_global_version;

/**
    Injects all standard globals into the given Lua state / VM.

    This includes all enabled standard libraries, which can
    be used from Lua with `require("@lune/library-name")`.

    # Errors

    Errors when out of memory, or if *default* Lua globals are missing.
*/
pub fn inject_globals(lua: &Lua, mut context_builder: GlobalsContextBuilder) -> LuaResult<()> {
    let context = {
        inject_lune_standard_libraries(&mut context_builder)?;
        context_builder.build()
    };

    for global in LuneStandardGlobal::ALL {
        lua.globals()
            .set(global.name(), global.create(lua, &context)?)?;
    }

    Ok(())
}
