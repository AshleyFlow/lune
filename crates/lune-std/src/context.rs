use mlua::prelude::*;
use std::collections::HashMap;

/**
    Will only insert the item into the hashmap if the provided feature flag is enabled

    # Example

    ```rs
    context_builder.with_alias("lune", |modules| {
        insert_feature_only_module!(modules, "fs", lune_std_fs::module);

        /*
                turns into:

        #[cfg(feature = "fs")]
        modules.insert("fs", lune_std_fs::module);

        */

        Ok(())
    })?;
    ```
*/
#[macro_export]
macro_rules! insert_feature_only_module {
    ($modules:ident, $feature:literal, $module:expr) => {
        #[cfg(feature = $feature)]
        $modules.insert($feature, $module);
    };
}

/**
    Will insert the item into the hashmap

    # Example

    ```rs
    context_builder.with_alias("lune", |modules| {
        insert_module!(modules, "fs", lune_std_fs::module);

        /*
                turns into:

        modules.insert("fs", lune_std_fs::module);

        */

        Ok(())
    })?;
    ```
*/
#[macro_export]
macro_rules! insert_module {
    ($modules:ident, $feature:literal, $module:expr) => {
        $modules.insert($feature, $module);
    };
}

pub type LuneModuleCreator = fn(&Lua) -> LuaResult<LuaTable>;

#[derive(Default, Clone, Debug)]
pub struct LuneModule {
    pub children: HashMap<&'static str, LuneModuleCreator>,
    pub alias: &'static str,
}

#[derive(Default, Clone, Debug)]
pub struct GlobalsContext {
    pub(crate) modules: Vec<LuneModule>,
}

impl GlobalsContext {
    #[must_use]
    pub fn get_alias(&self, s: &str) -> Option<&LuneModule> {
        self.modules.iter().find(|x| x.alias == s)
    }
}

#[derive(Default)]
pub struct GlobalsContextBuilder {
    modules: Vec<LuneModule>,
}

impl GlobalsContextBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /**
        # Errors

        Errors if the handler errors

        # Example

        ```rs
        let create_pixels = |lua: &Lua| -> LuaResult<LuaTable> {
            ... // return a lua table
        };

        builder.with_alias(|modules| {
            // There are multiple ways of inserting a module

            // .1
            modules.insert("pixels", create_pixels);

            // .2
            // does the exact same thing as .1
            insert_module!(modules, "pixels", create_pixels);

            // .3
            // does the exact same thing as .1
            // but only if a feature flag with the name of "pixels" is enabled
            insert_feature_only_module!(modules, "pixels", create_pixels);

            Ok(())
        })?;
        ```
    */
    pub fn with_alias(
        &mut self,
        name: &'static str,
        handler: fn(&mut HashMap<&str, LuneModuleCreator>) -> LuaResult<()>,
    ) -> LuaResult<()> {
        let mut modules = HashMap::new();
        handler(&mut modules)?;

        let alias = LuneModule {
            alias: name,
            children: modules,
        };

        self.modules.push(alias);

        Ok(())
    }

    #[must_use]
    pub fn build(self) -> GlobalsContext {
        GlobalsContext {
            modules: self.modules,
        }
    }
}
