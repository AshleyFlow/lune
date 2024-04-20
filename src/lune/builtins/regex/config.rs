use mlua::prelude::*;
use regex::Regex;

pub struct LuaRegex {
    regex: Regex,
}

impl LuaRegex {
    pub fn new(pattern: String) -> LuaRegex {
        return Self {
            regex: Regex::new(pattern.as_str()).unwrap(),
        };
    }
}

impl LuaUserData for LuaRegex {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("match", lua_regex_match);
    }
}

fn lua_regex_match<'lua>(
    lua: &'lua Lua,
    this: &LuaRegex,
    string: LuaString,
) -> LuaResult<LuaTable<'lua>> {
    // If this panics, it's likely caused by a bug
    let captures = this
        .regex
        .captures_iter(string.to_str().expect("Failed to turn LuaString into str."));
    let t = lua.create_table()?;

    for capture in captures {
        let full_match = capture.get(0).map_or("", |m| m.as_str());
        t.push(full_match.into_lua(lua)?)?;

        for i in 1..=capture.len() {
            if let Some(matched) = capture.get(i) {
                t.push(matched.as_str().into_lua(lua)?)?;
            }
        }
    }

    Ok(t)
}
