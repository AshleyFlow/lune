use crate::context::{GlobalsContextBuilder, LuneModuleCreator};
use mlua::prelude::*;

#[derive(Clone, Copy)]
#[rustfmt::skip]
pub enum LuneStandardLibrary {
    #[cfg(feature = "fs")]      Fs,
    #[cfg(feature = "luau")]    Luau,
    #[cfg(feature = "net")]     Net,
    #[cfg(feature = "task")]    Task,
    #[cfg(feature = "process")] Process,
    #[cfg(feature = "regex")]   Regex,
    #[cfg(feature = "serde")]   Serde,
    #[cfg(feature = "stdio")]   Stdio,
    #[cfg(feature = "roblox")]  Roblox,
}

impl LuneStandardLibrary {
    #[rustfmt::skip]
    const ALL: &'static [Self] = &[
        #[cfg(feature = "fs")]      Self::Fs,
        #[cfg(feature = "luau")]    Self::Luau,
        #[cfg(feature = "net")]     Self::Net,
        #[cfg(feature = "task")]    Self::Task,
        #[cfg(feature = "process")] Self::Process,
        #[cfg(feature = "regex")]   Self::Regex,
        #[cfg(feature = "serde")]   Self::Serde,
        #[cfg(feature = "stdio")]   Self::Stdio,
        #[cfg(feature = "roblox")]  Self::Roblox,
    ];

    #[must_use]
    #[rustfmt::skip]
    fn name(&self) -> &str {
        #[allow(clippy::enum_glob_use)]
        use LuneStandardLibrary::*;

        match self {
            #[cfg(feature = "fs")]      Fs      => "fs",
            #[cfg(feature = "luau")]    Luau    => "luau",
            #[cfg(feature = "net")]     Net     => "net",
            #[cfg(feature = "task")]    Task    => "task",
            #[cfg(feature = "process")] Process => "process",
            #[cfg(feature = "regex")]   Regex   => "regex",
            #[cfg(feature = "serde")]   Serde   => "serde",
            #[cfg(feature = "stdio")]   Stdio   => "stdio",
            #[cfg(feature = "roblox")]  Roblox  => "roblox",
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn module_creator(self) -> LuneModuleCreator {
        #[allow(clippy::enum_glob_use)]
        use LuneStandardLibrary::*;

        match self {
            #[cfg(feature = "fs")]      Fs      => LuneModuleCreator::LuaTable(lune_std_fs::module),
            #[cfg(feature = "luau")]    Luau    => LuneModuleCreator::LuaTable(lune_std_luau::module),
            #[cfg(feature = "net")]     Net     => LuneModuleCreator::LuaTable(lune_std_net::module),
            #[cfg(feature = "task")]    Task    => LuneModuleCreator::LuaTable(lune_std_task::module),
            #[cfg(feature = "process")] Process => LuneModuleCreator::LuaTable(lune_std_process::module),
            #[cfg(feature = "regex")]   Regex   => LuneModuleCreator::LuaTable(lune_std_regex::module),
            #[cfg(feature = "serde")]   Serde   => LuneModuleCreator::LuaTable(lune_std_serde::module),
            #[cfg(feature = "stdio")]   Stdio   => LuneModuleCreator::LuaTable(lune_std_stdio::module),
            #[cfg(feature = "roblox")]  Roblox  => LuneModuleCreator::LuaTable(lune_std_roblox::module),
        }
    }
}

pub fn inject_lune_standard_libraries(
    context_builder: &mut GlobalsContextBuilder,
) -> LuaResult<()> {
    context_builder.with_alias("lune", |modules| {
        for x in LuneStandardLibrary::ALL {
            modules.insert(x.name(), x.module_creator());
        }

        Ok(())
    })
}
