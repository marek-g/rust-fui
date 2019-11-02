extern crate winit;

pub use drawing::color::ColorFormat;

use Result;

use drawing::backend::Device;
use drawing::backend::Texture;
use drawing::backend::WindowTarget;
use drawing::color::*;
use drawing::font::*;
use drawing::primitive::Primitive;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;
use drawing::TextureFont;
use drawing_gl::*;

use find_folder;
use std::fs::File;
use std::io::Read;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub type DrawingDevice = GlDevice;
pub type DrawingTexture = GlTexture;
pub type DrawingWindowTarget = GlWindowTarget;
pub type DrawingFont = TextureFont<DrawingDevice>;

pub struct DrawingContext {
    resources: Resources<DrawingDevice, DrawingFont>,
    device: DrawingDevice,
    renderer: Renderer,
}

impl DrawingContext {
    pub fn new() -> Result<Self> {
        Ok(DrawingContext {
            resources: Resources::new(),
            device: DrawingDevice::new()?,
            renderer: Renderer::new(),
        })
    }

    pub fn create_window(
        &mut self,
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
        shared_window_target: Option<&DrawingWindowTarget>,
    ) -> Result<DrawingWindowTarget> {
        Ok(self
            .device
            .create_window_target(window_builder, &event_loop, shared_window_target)?)
    }

    pub fn get_font(&mut self, font_name: &'static str) -> Result<&mut DrawingFont> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join(font_name)
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let font = DrawingFont::create(&mut self.device, buffer)?;

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
            let font_path = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join(font_name)
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let font = DrawingFont::create(&mut self.device, buffer)?;

            self.resources
                .fonts_mut()
                .insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions(&mut self.device, FontParams { size: size }, &text)
        } else {
            Ok((0, size as u16))
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

    pub fn update_size(
        &mut self,
        window_target: &mut DrawingWindowTarget,
        width: u16,
        height: u16,
    ) {
        window_target.update_size(width, height);
    }

    pub fn begin(&mut self, window_target: &DrawingWindowTarget) -> Result<()> {
        self.device.begin(window_target)
    }

    pub fn clear(
        &mut self,
        render_target: &<DrawingDevice as Device>::RenderTarget,
        color: &Color,
    ) {
        self.device.clear(render_target, color)
    }

    pub fn draw(
        &mut self,
        render_target: &<DrawingDevice as Device>::RenderTarget,
        size: PhysPixelSize,
        primitives: Vec<Primitive>,
    ) -> Result<()> {
        self.renderer.draw(
            &mut self.device,
            render_target,
            size,
            primitives,
            &mut self.resources,
        )
    }

    pub fn end(&mut self, window_target: &DrawingWindowTarget) {
        self.device.end(window_target);
    }
}
