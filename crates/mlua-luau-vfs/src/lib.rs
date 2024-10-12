use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[cfg(test)]
mod test;

mod utils;

#[derive(Debug, Default)]
pub struct VirtualFileSystem {
    pub(crate) files: HashMap<PathBuf, &'static [u8]>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Get the attached struct from lua VM
     */
    pub fn get(lua: &mlua::Lua) -> Option<mlua::AppDataRefMut<'_, Self>> {
        lua.app_data_mut()
    }

    pub fn read<T>(&mut self, path: T) -> Option<Vec<u8>>
    where
        T: AsRef<Path>,
    {
        let normalized_path = utils::normalize_path(path);
        self.files.get(&normalized_path).map(|x| x.to_vec())
    }

    pub fn write<T>(&mut self, path: T, contents: &'static [u8])
    where
        T: AsRef<Path>,
    {
        let normalized_path = utils::normalize_path(path);
        self.files.insert(normalized_path, contents);
    }

    pub fn remove<T>(&mut self, path: T)
    where
        T: AsRef<Path>,
    {
        let normalized_path = utils::normalize_path(path);
        self.files.remove(&normalized_path);
    }

    pub fn exists<T>(&self, path: T) -> bool
    where
        T: AsRef<Path>,
    {
        let normalized_path = utils::normalize_path(path);
        self.files.contains_key(normalized_path.as_path())
    }

    /**
    Attach the struct to the provided lua VM
     */
    pub fn attach(self, lua: &mlua::Lua) -> mlua::Result<()> {
        if lua.app_data_ref::<Self>().is_some() {
            Err(mlua::Error::runtime(
                "This lua VM already has a VirtualFileSystem struct",
            ))
        } else {
            lua.set_app_data(self);

            Ok(())
        }
    }
}
