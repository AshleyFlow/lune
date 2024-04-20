use mlua::prelude::*;
use regex::Regex;

pub struct LuaRegex {
    pub pattern: String,
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
    let regex = Regex::new(this.pattern.as_str()).unwrap();
    let captures =
        regex.captures_iter(string.to_str().expect("Failed to turn LuaString into str."));
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

// impl<'lua> FromLua<'lua> for LuaRegex {
//     fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
//         let userdata = value.as_userdata();

//         if let Some(userdata) = userdata {
//             if let Ok(this) = userdata.borrow::<Self>() {
//                 return Ok(this.clone());
//             } else {
//                 return Err(LuaError::FromLuaConversionError {
//                     from: value.type_name(),
//                     to: "LuaRegex",
//                     message: None,
//                 });
//             }
//         }

//         Err(LuaError::FromLuaConversionError {
//             from: value.type_name(),
//             to: "userdata",
//             message: None,
//         })
//     }
// }
