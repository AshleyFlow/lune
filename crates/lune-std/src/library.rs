use crate::{
    context::{GlobalsContextBuilder, LuneModuleCreator},
    insert_feature_only_module,
};
use mlua::prelude::*;

pub fn inject_lune_standard_libraries(
    context_builder: &mut GlobalsContextBuilder,
) -> LuaResult<()> {
    context_builder.with_alias("lune", |modules| {
        insert_feature_only_module!(
            modules,
            "fs",
            LuneModuleCreator::LuaTable(lune_std_fs::module)
        );
        insert_feature_only_module!(
            modules,
            "luau",
            LuneModuleCreator::LuaTable(lune_std_luau::module)
        );
        insert_feature_only_module!(
            modules,
            "net",
            LuneModuleCreator::LuaTable(lune_std_net::module)
        );
        insert_feature_only_module!(
            modules,
            "task",
            LuneModuleCreator::LuaTable(lune_std_task::module)
        );
        insert_feature_only_module!(
            modules,
            "process",
            LuneModuleCreator::LuaTable(lune_std_process::module)
        );
        insert_feature_only_module!(
            modules,
            "regex",
            LuneModuleCreator::LuaTable(lune_std_regex::module)
        );
        insert_feature_only_module!(
            modules,
            "serde",
            LuneModuleCreator::LuaTable(lune_std_serde::module)
        );
        insert_feature_only_module!(
            modules,
            "stdio",
            LuneModuleCreator::LuaTable(lune_std_stdio::module)
        );
        insert_feature_only_module!(
            modules,
            "roblox",
            LuneModuleCreator::LuaTable(lune_std_roblox::module)
        );

        Ok(())
    })
}
