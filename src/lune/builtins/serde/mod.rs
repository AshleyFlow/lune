use bstr::BString;
use mlua::prelude::*;

pub(super) mod compress_decompress;
pub(super) mod encode_decode;

use compress_decompress::{compress, decompress, CompressDecompressFormat};
use encode_decode::{EncodeDecodeConfig, EncodeDecodeFormat};

use crate::lune::util::TableBuilder;

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("encode", serde_encode)?
        .with_function("decode", serde_decode)?
        .with_async_function("compress", serde_compress)?
        .with_async_function("decompress", serde_decompress)?
        .build_readonly()
}

fn serde_encode<'lua>(
    lua: &'lua Lua,
    (format, val, pretty): (EncodeDecodeFormat, LuaValue<'lua>, Option<bool>),
) -> LuaResult<LuaString<'lua>> {
    let config = EncodeDecodeConfig::from((format, pretty.unwrap_or_default()));
    config.serialize_to_string(lua, val)
}

fn serde_decode(lua: &Lua, (format, str): (EncodeDecodeFormat, BString)) -> LuaResult<LuaValue> {
    let config = EncodeDecodeConfig::from(format);
    config.deserialize_from_string(lua, str)
}

async fn serde_compress(
    lua: &Lua,
    (format, str): (CompressDecompressFormat, BString),
) -> LuaResult<LuaString> {
    let bytes = compress(format, str).await?;
    lua.create_string(bytes)
}

async fn serde_decompress(
    lua: &Lua,
    (format, str): (CompressDecompressFormat, BString),
) -> LuaResult<LuaString> {
    let bytes = decompress(format, str).await?;
    lua.create_string(bytes)
}
