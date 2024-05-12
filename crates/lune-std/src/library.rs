use crate::context::GlobalsContextBuilder;
use mlua::prelude::*;

pub fn inject_lune_standard_libraries(
    mut context_builder: GlobalsContextBuilder,
) -> LuaResult<GlobalsContextBuilder> {
    context_builder = context_builder.set_alias("lune");

    #[cfg(feature = "fs")]
    context_builder.borrow_set("fs", lune_std_fs::module)?;

    #[cfg(feature = "luau")]
    context_builder.borrow_set("luau", lune_std_luau::module)?;

    #[cfg(feature = "net")]
    context_builder.borrow_set("net", lune_std_net::module)?;

    #[cfg(feature = "task")]
    context_builder.borrow_set("task", lune_std_task::module)?;

    #[cfg(feature = "process")]
    context_builder.borrow_set("process", lune_std_process::module)?;

    #[cfg(feature = "regex")]
    context_builder.borrow_set("regex", lune_std_regex::module)?;

    #[cfg(feature = "serde")]
    context_builder.borrow_set("serde", lune_std_serde::module)?;

    #[cfg(feature = "stdio")]
    context_builder.borrow_set("stdio", lune_std_stdio::module)?;

    #[cfg(feature = "roblox")]
    context_builder.borrow_set("roblox", lune_std_roblox::module)?;

    Ok(context_builder)
}
