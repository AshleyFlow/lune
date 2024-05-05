use super::{usertypes::LuaRGBA, window::config::LuaWindow};
use crate::lune::builtins::wry::usertypes::LuaDimension;
use bstr::BString;
use mlua::prelude::*;
use pixels::{Pixels, SurfaceTexture};

pub struct LuaPixels {
    pixels: Pixels,
    buffer: LuaRegistryKey,
}

impl LuaPixels {
    pub fn new<'lua>(
        lua: &'lua Lua,
        window: &'lua LuaWindow,
        (field1, field2): (&'lua LuaValue<'lua>, &'lua LuaValue<'lua>),
    ) -> LuaResult<Self> {
        let width = field1.as_u32().unwrap();
        let height = field2.as_u32().unwrap();

        let size = window.window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window.window);
        let pixels = Pixels::new(width, height, surface_texture).into_lua_err()?;
        let buffer = lua.create_buffer(pixels.frame())?;

        Ok(Self {
            pixels,
            buffer: lua.create_registry_value(buffer)?,
        })
    }

    pub fn update_buffer(&mut self, lua: &Lua) -> LuaResult<()> {
        let buffer = lua.create_buffer(self.pixels.frame())?;
        self.buffer = lua.create_registry_value(buffer)?;
        Ok(())
    }
}

impl LuaUserData for LuaPixels {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("frame", |lua, this| {
            lua.registry_value::<LuaAnyUserData>(&this.buffer)
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

        methods.add_method_mut("_mutate_frame", |lua, this, handler: LuaFunction| {
            let frame_mut = this.pixels.frame_mut();

            for (i, pixel) in frame_mut.chunks_exact_mut(4).enumerate() {
                let buffer = lua.create_buffer(&pixel)?;
                handler.call::<_, ()>((i, &buffer))?;

                let new_pixel = lua.unpack::<BString>(LuaValue::UserData(buffer))?;
                pixel.copy_from_slice(new_pixel.as_slice());
            }

            Ok(())
        });

        methods.add_method_mut("mutate_frame", |lua, this, buffer: LuaAnyUserData| {
            let frame_mut = this.pixels.frame_mut();
            let new_frame = lua.unpack::<BString>(LuaValue::UserData(buffer))?;
            frame_mut.copy_from_slice(new_frame.as_slice());

            Ok(())
        });

        methods.add_method_mut("render", |lua, this, _: ()| {
            let buffer = lua.registry_value::<LuaAnyUserData>(&this.buffer)?;
            let frame_mut = this.pixels.frame_mut();
            let new_frame = lua.unpack::<BString>(LuaValue::UserData(buffer))?;
            frame_mut.copy_from_slice(new_frame.as_slice());

            this.pixels.render().into_lua_err()
        });

        methods.add_method_mut(
            "draw_pixel",
            |lua, this, (dimension, rgba): (LuaDimension, LuaRGBA)| {
                let width = this.pixels.texture().width() as usize;
                let frame_mut = this.pixels.frame_mut();

                for (i, pixel) in frame_mut.chunks_exact_mut(4).enumerate() {
                    let x = i % width;
                    let y = i / width;

                    if x == dimension.x as usize && y == dimension.y as usize {
                        pixel[0] = rgba.r;
                        pixel[1] = rgba.g;
                        pixel[2] = rgba.b;
                        pixel[3] = rgba.a;
                    }
                }

                this.update_buffer(lua)?;
                this.pixels.render().into_lua_err()
            },
        );

        methods.add_method_mut(
            "draw_line",
            |lua, this, (dimension1, dimension2, rgba): (LuaDimension, LuaDimension, LuaRGBA)| {
                let width = this.pixels.texture().width() as i32;
                let frame_mut = this.pixels.frame_mut();

                let (mut x1, mut y1) = (dimension1.x as i32, dimension1.y as i32);
                let (x2, y2) = (dimension2.x as i32, dimension2.y as i32);

                let dx = (x2 - x1).abs();
                let dy = -(y2 - y1).abs();
                let sx = if x1 < x2 { 1 } else { -1 };
                let sy = if y1 < y2 { 1 } else { -1 };
                let mut err = dx + dy;

                loop {
                    // Calculate the index in the frame_mut array
                    let index = (y1 * width + x1) * 4;
                    if index < frame_mut.len() as i32 {
                        // Set the pixel color
                        frame_mut[index as usize..index as usize + 4]
                            .copy_from_slice(&[rgba.r, rgba.g, rgba.b, rgba.a]);
                    }

                    // Check if we've reached the end point
                    if x1 == x2 && y1 == y2 {
                        break;
                    }

                    let e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x1 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y1 += sy;
                    }
                }

                this.update_buffer(lua)?;
                this.pixels.render().into_lua_err()
            },
        );
    }
}

pub async fn create<'lua>(
    lua: &'lua Lua,
    values: LuaMultiValue<'lua>,
) -> LuaResult<LuaAnyUserData<'lua>> {
    let field1 = values.get(0).expect("Parameter 1 is missing");
    let field2 = values.get(1).expect("Parameter 2 is missing");
    let field3 = values.get(2).expect("Parameter 3 is missing");
    let window = field1.as_userdata().unwrap().borrow_mut::<LuaWindow>()?;

    lua.create_userdata(LuaPixels::new(lua, &window, (field2, field3))?)
}
