use super::window::config::LuaWindow;
use bstr::BString;
use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};

pub struct LuaPixels {
    pixels: Pixels,
}

impl LuaPixels {
    pub async fn new<'lua>(
        window: &'lua LuaWindow,
        (field1, field2): (&'lua LuaValue<'lua>, &'lua LuaValue<'lua>),
    ) -> Self {
        let width = field1.as_u32().unwrap();
        let height = field2.as_u32().unwrap();

        let size = window.window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window.window);
        let pixels = Pixels::new_async(width, height, surface_texture)
            .await
            .unwrap();

        Self { pixels }
    }
}

impl LuaUserData for LuaPixels {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frame", |lua, this| {
            let mut t = lua.create_table()?;

            for pixel in this.pixels.frame() {
                let t_inner = t;

                t_inner.push(*pixel as i32)?;

                t = t_inner;
            }

            Ok(t)
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "resize",
            |_lua, this, (width, height): (LuaNumber, LuaNumber)| {
                this.pixels
                    .resize_surface(width as u32, height as u32)
                    .into_lua_err()
            },
        );

        methods.add_method_mut("mutate_frame", |lua, this, handler: LuaFunction| {
            let frame_mut = this.pixels.frame_mut();

            for (i, pixel) in frame_mut.chunks_exact_mut(4).enumerate() {
                let buffer = lua.create_buffer(&pixel)?;
                handler.call::<_, ()>((i, &buffer))?;

                let new_pixel = lua.unpack::<BString>(LuaValue::UserData(buffer))?;
                pixel.copy_from_slice(new_pixel.as_slice());
            }

            Ok(())
        });

        methods.add_method("render", |_lua, this, _: ()| {
            this.pixels.render().into_lua_err()
        });
    }
}

pub async fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let field2 = values.get(1).expect("Parameter 2 is missing");
    let field3 = values.get(2).expect("Parameter 3 is missing");
    let mut window = field1.as_userdata().unwrap().borrow_mut::<LuaWindow>()?;
    // let lua_pixels = Rc::new(LuaPixels::new(&window).await);

    // window.pixels = Some(Rc::clone(&lua_pixels));

    lua.create_userdata(LuaPixels::new(&window, (field2, field3)).await)
}
