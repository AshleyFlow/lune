use mlua::prelude::*;
use std::collections::HashMap;

pub type LuaModule = fn(&Lua) -> LuaResult<LuaTable>;

#[derive(Default, Clone, Debug)]
pub struct LuaAlias {
    pub children: HashMap<&'static str, LuaModule>,
    pub alias: &'static str,
}

#[derive(Default, Clone, Debug)]
pub struct GlobalsContext {
    pub(crate) aliases: Vec<LuaAlias>,
}

impl GlobalsContext {
    #[must_use]
    pub fn get_alias(&self, s: &str) -> Option<LuaAlias> {
        for alias in &self.aliases {
            if alias.alias == s {
                return Some(alias.clone());
            }
        }

        None
    }
}

#[derive(Default)]
pub struct GlobalsContextBuilder {
    aliases: Vec<LuaAlias>,
}

impl GlobalsContextBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn set_alias(mut self, name: &'static str) -> Self {
        let alias = LuaAlias {
            alias: name,
            ..Default::default()
        };
        self.aliases.push(alias);

        self
    }

    /**
        # Panics

        Errors when it gets called before '`set_alias`'

        # Errors

        Errors when it gets called before '`set_alias`'

        Errors when out of memory
    */
    pub fn set(mut self, key: &'static str, value: LuaModule) -> LuaResult<Self> {
        if self.aliases.is_empty() {
            return  Err(LuaError::RuntimeError(
                "Tried to set value before setting an alias, use 'set_alias' before calling 'set_value'"
            .into()));
        }

        let alias = self.aliases.last_mut().unwrap();
        alias.children.insert(key, value);

        Ok(self)
    }

    /**
        # Panics

        Errors when it gets called before '`set_alias`'

        # Errors

        Errors when it gets called before '`set_alias`'

        Errors when out of memory
    */
    pub fn borrow_set(&mut self, key: &'static str, value: LuaModule) -> LuaResult<()> {
        if self.aliases.is_empty() {
            return  Err(LuaError::RuntimeError(
                "Tried to set value before setting an alias, use 'set_alias' before calling 'set_value'"
            .into()));
        }

        let alias = self.aliases.last_mut().unwrap();
        alias.children.insert(key, value);

        Ok(())
    }

    #[must_use]
    pub fn build(self) -> GlobalsContext {
        GlobalsContext {
            aliases: self.aliases,
        }
    }
}
