pub use drawing::color::ColorFormat;

use anyhow::{format_err, Result};

use drawing::backend::Device;
use drawing::backend::Texture;
use drawing::font::*;
use drawing::primitive::Primitive;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::TextureFont;
use drawing_gl::*;

use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub type DrawingDevice = GlDevice;
pub type DrawingTexture = GlTexture;
pub type DrawingFont = TextureFont<DrawingDevice>;

pub struct DrawingContext {
    pub(crate) resources: Resources<DrawingDevice, DrawingFont>,
    pub(crate) device: DrawingDevice,
    pub(crate) renderer: Renderer,

    pub(crate) background_texture: i32,
}

impl DrawingContext {
    pub fn new() -> Result<Self> {
        Ok(DrawingContext {
            resources: Resources::new(),
            device: DrawingDevice::new()?,
            renderer: Renderer::new(),

            background_texture: -1,
        })
    }

    pub fn get_font(&mut self, font_name: &'static str) -> Result<&mut DrawingFont> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = Path::new("assets")
                .join(font_name)
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let font = DrawingFont::create(buffer)?;

            self.resources
                .fonts_mut()
                .insert(font_name.to_string(), font);
        }

        Ok(self
            .resources
            .fonts_mut()
            .get_mut(&font_name.to_string())
            .unwrap())
    }

    pub fn get_font_dimensions(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(u16, u16)> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = Path::new("assets")
                .join(font_name)
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let font = DrawingFont::create(buffer)?;

            self.resources
                .fonts_mut()
                .insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions(FontParams { size: size }, &text)
        } else {
            Ok((0, size as u16))
        }
    }

    pub fn get_font_dimensions_each_char(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(Vec<i16>, u16)> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = Path::new("assets")
                .join(font_name)
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let font = DrawingFont::create(buffer)?;

            self.resources
                .fonts_mut()
                .insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions_each_char(FontParams { size: size }, &text)
        } else {
            Ok((Vec::new(), size as u16))
        }
    }

    pub fn get_resources(&self) -> &Resources<DrawingDevice, DrawingFont> {
        &self.resources
    }

    pub fn get_resources_mut(&mut self) -> &mut Resources<DrawingDevice, DrawingFont> {
        &mut self.resources
    }

    pub fn create_texture(
        &mut self,
        memory: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
        updatable: bool,
    ) -> Result<i32> {
        let texture_id = self.resources.get_next_texture_id();
        let texture = self
            .device
            .create_texture(Some(memory), width, height, format, updatable)?;
        self.resources.textures_mut().insert(texture_id, texture);
        Ok(texture_id)
    }

    pub fn update_texture(
        &mut self,
        texture_id: i32,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        if let Some(texture) = self.resources.textures_mut().get_mut(&texture_id) {
            texture.update(memory, offset_x, offset_y, width, height)?;
        }
        Ok(())
    }

    /*pub fn update_size(&mut self, window_target: &mut GlWindow, width: u16, height: u16) {
        // TODO:
        //window_target.update_size(width, height);
    }*/

    pub fn begin(&mut self, gl_context_data: &GlContextData) -> Result<()> {
        self.device.begin(gl_context_data)
    }

    pub fn clear(
        &mut self,
        render_target: &<DrawingDevice as Device>::RenderTarget,
        color: &fui_core::Color,
    ) {
        self.device.clear(render_target, color)
    }

    pub fn draw(
        &mut self,
        render_target: &<DrawingDevice as Device>::RenderTarget,
        primitives: &Vec<Primitive>,
    ) -> Result<()> {
        self.renderer.draw(
            &mut self.device,
            render_target,
            primitives,
            &mut self.resources,
            false,
        )
    }

    pub fn end(&mut self, gl_context_data: &GlContextData) {
        //self.device.end(gl_context_data);
    }

    pub fn get_background_texture(&mut self) -> i32 {
        // create background texture
        if self.background_texture < 0 {
            let mut data = [0u8; 256 * 256 * 4];
            for i in 0..256 * 256 {
                data[i * 4 + 0] = 60;
                data[i * 4 + 1] = thread_rng().gen_range(90 - 15, 90 + 16);
                data[i * 4 + 2] = 60;
                data[i * 4 + 3] = 255;
            }

            self.background_texture = self
                .create_texture(&data, 256, 256, ColorFormat::RGBA, false)
                .unwrap();
        }

        self.background_texture
    }
}

impl fui_core::Resources for DrawingContext {
    fn get_font_dimensions(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(u16, u16)> {
        self.get_font_dimensions(font_name, size, text)
    }

    fn get_font_dimensions_each_char(
        &mut self,
        font_name: &'static str,
        size: u8,
        text: &str,
    ) -> Result<(Vec<i16>, u16)> {
        self.get_font_dimensions_each_char(font_name, size, text)
    }

    fn create_texture(
        &mut self,
        memory: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
        updatable: bool,
    ) -> Result<i32> {
        self.create_texture(memory, width, height, format, updatable)
    }

    fn update_texture(
        &mut self,
        texture_id: i32,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        self.update_texture(texture_id, memory, offset_x, offset_y, width, height)
    }

    fn get_texture_size(&mut self, texture_id: i32) -> Result<(u16, u16)> {
        if let Some(texture) = self.get_resources().textures().get(&texture_id) {
            Ok(texture.get_size())
        } else {
            Err(format_err!("Texture not found!"))
        }
    }
}

pub struct FuiDrawingContext<'a> {
    drawing_area_size: (u16, u16),
    resources: &'a mut crate::DrawingContext,
    background_texture: i32,
}

impl<'a> FuiDrawingContext<'a> {
    pub fn new(
        drawing_area_size: (u16, u16),
        resources: &'a mut crate::DrawingContext,
        background_texture: i32,
    ) -> Self {
        FuiDrawingContext {
            drawing_area_size,
            resources,
            background_texture,
        }
    }
}

impl<'a> fui_core::DrawingContext for FuiDrawingContext<'a> {
    fn get_drawing_area_size(&self) -> (u16, u16) {
        self.drawing_area_size
    }
    fn get_resources(&mut self) -> &mut dyn fui_core::Resources {
        self.resources
    }
    fn get_background_texture(&self) -> i32 {
        self.background_texture
    }
}
