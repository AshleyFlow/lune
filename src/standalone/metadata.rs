use std::{env, path::PathBuf};

use anyhow::{bail, Result};
use mlua::Compiler as LuaCompiler;
use once_cell::sync::Lazy;
use tokio::fs;

const MAGIC: &[u8; 8] = b"cr3sc3nt";
const MAGIC_NOCONSOLE: &[u8; 8] = b"cr5sc5nt";

static CURRENT_EXE: Lazy<PathBuf> =
    Lazy::new(|| env::current_exe().expect("failed to get current exe"));

/*
    TODO: Right now all we do is append the bytecode to the end
    of the binary, but we will need a more flexible solution in
    the future to store many files as well as their metadata.

    The best solution here is most likely to use a well-supported
    and rust-native binary serialization format with a stable
    specification, one that also supports byte arrays well without
    overhead, so the best solution seems to currently be Postcard:

    https://github.com/jamesmunns/postcard
    https://crates.io/crates/postcard
*/

/**
    Metadata for a standalone Lune executable. Can be used to
    discover and load the bytecode contained in a standalone binary.
*/
#[derive(Debug, Clone)]
pub struct Metadata {
    pub bytecode: Vec<u8>,
}

impl Metadata {
    /**
        Returns whether or not the currently executing Lune binary
        is a standalone binary, and if so, the bytes of the binary.
    */
    pub async fn check_env() -> (bool, Vec<u8>, bool) {
        let contents = fs::read(CURRENT_EXE.to_path_buf())
            .await
            .unwrap_or_default();

        let no_console = contents.ends_with(MAGIC_NOCONSOLE);
        let is_standalone = contents.ends_with(MAGIC) || no_console;
        (is_standalone, contents, no_console)
    }

    /**
        Creates a patched standalone binary from the given script contents.
    */
    pub async fn create_env_patched_bin(
        script_contents: impl Into<Vec<u8>>,
        no_console: bool,
    ) -> Result<Vec<u8>> {
        let mut patched_bin = fs::read(CURRENT_EXE.to_path_buf()).await?;

        // Compile luau input into bytecode
        let bytecode = LuaCompiler::new()
            .set_optimization_level(2)
            .set_coverage_level(0)
            .set_debug_level(1)
            .compile(script_contents.into());

        // Append the bytecode / metadata to the end
        let meta = Self { bytecode };
        patched_bin.extend_from_slice(&meta.to_bytes({
            if no_console {
                *MAGIC_NOCONSOLE
            } else {
                *MAGIC
            }
        }));

        Ok(patched_bin)
    }

    /**
        Tries to read a standalone binary from the given bytes.
    */
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();
        let magic: Option<&[u8; 8]> = {
            if bytes.ends_with(MAGIC) {
                Some(MAGIC)
            } else if bytes.ends_with(MAGIC_NOCONSOLE) {
                Some(MAGIC_NOCONSOLE)
            } else {
                None
            }
        };

        if bytes.len() < 16 || magic.is_none() {
            bail!("not a standalone binary")
        }

        // Extract bytecode size
        let bytecode_size_bytes = &bytes[bytes.len() - 16..bytes.len() - magic.unwrap().len()];
        let bytecode_size =
            usize::try_from(u64::from_be_bytes(bytecode_size_bytes.try_into().unwrap()))?;

        // Extract bytecode
        let bytecode = bytes[bytes.len() - 16 - bytecode_size..].to_vec();

        Ok(Self { bytecode })
    }

    /**
        Writes the metadata chunk to a byte vector, to later bet read using `from_bytes`.
    */
    pub fn to_bytes(&self, magic: [u8; 8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.bytecode);
        bytes.extend_from_slice(&(self.bytecode.len() as u64).to_be_bytes());
        bytes.extend_from_slice(&magic);
        bytes
    }
}
