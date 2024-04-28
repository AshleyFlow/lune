use super::lua_table_to_headers;
use bstr::{BString, ByteSlice};
use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Response};
use mlua::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub enum LuaResponseKind {
    PlainText,
    Table,
}

pub struct LuaResponse {
    pub kind: LuaResponseKind,
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Option<Vec<u8>>,
}

impl LuaResponse {
    pub fn into_response1(self) -> LuaResult<Response<Full<Bytes>>> {
        Ok(match self.kind {
            LuaResponseKind::PlainText => Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Full::new(Bytes::from(self.body.unwrap())))
                .into_lua_err()?,
            LuaResponseKind::Table => {
                let mut response = Response::builder()
                    .status(self.status)
                    .body(Full::new(Bytes::from(self.body.unwrap_or_default())))
                    .into_lua_err()?;
                response.headers_mut().extend(self.headers);
                response
            }
        })
    }

    pub fn into_response2(self) -> LuaResult<Response<Cow<'static, [u8]>>> {
        Ok(match self.kind {
            LuaResponseKind::PlainText => Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Cow::Owned(self.body.unwrap()))
                .unwrap(),
            LuaResponseKind::Table => {
                let mut response = Response::builder()
                    .status(self.status)
                    .body(Cow::Owned(self.body.unwrap_or_default()))
                    .into_lua_err()?;
                response.headers_mut().extend(self.headers);
                response
            }
        })
    }
}

impl FromLua<'_> for LuaResponse {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        match value {
            // Plain strings from the handler are plaintext responses
            LuaValue::String(s) => Ok(Self {
                kind: LuaResponseKind::PlainText,
                status: 200,
                headers: HeaderMap::new(),
                body: Some(s.as_bytes().to_vec()),
            }),
            // Tables are more detailed responses with potential status, headers, body
            LuaValue::Table(t) => {
                let status: Option<u16> = t.get("status")?;
                let headers: Option<LuaTable> = t.get("headers")?;
                let body: Option<BString> = t.get("body")?;

                let headers_map = lua_table_to_headers(headers, lua)?;
                let body_bytes = body.map(|s: BString| s.as_bytes().to_vec());

                Ok(Self {
                    kind: LuaResponseKind::Table,
                    status: status.unwrap_or(200),
                    headers: headers_map,
                    body: body_bytes,
                })
            }
            // Anything else is an error
            value => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "NetServeResponse",
                message: None,
            }),
        }
    }
}
