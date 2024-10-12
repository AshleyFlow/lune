#[derive(Debug, Default)]
pub struct VirtualFileSystem {}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self::default()
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
