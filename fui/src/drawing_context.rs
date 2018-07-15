extern crate winit;

pub use drawing::color::ColorFormat;

use drawing_gl::*;
use drawing::backend::Device;
use drawing::backend::WindowTarget;
use drawing::backend::Texture;
use drawing::TextureFont;
use drawing::font::*;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;
use drawing::primitive::Primitive;

use winit::EventsLoop;
use winit::WindowBuilder;
use find_folder;
use std::fs::File;
use std::io::Read;

pub type DrawingDevice = GlDevice;
pub type DrawingTexture = GlTexture;
pub type DrawingWindowTarget = GlWindowTarget;
pub type DrawingFont = TextureFont<DrawingDevice>;

pub struct DrawingContext {
    resources: Resources<DrawingTexture, DrawingFont>,
    device: DrawingDevice,
    renderer: Renderer,
}

impl DrawingContext {
    pub fn new() -> Self {
        DrawingContext {
            resources: Resources::new(),
            device: DrawingDevice::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn create_window(&mut self, window_builder: WindowBuilder, events_loop: &EventsLoop) -> DrawingWindowTarget {
        self.device.create_window_target(window_builder, &events_loop)
    }

    pub fn get_font(&mut self, font_name: &'static str) -> Option<&mut DrawingFont> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join(font_name).into_os_string().into_string().unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);

            let font = DrawingFont::create(&mut self.device, buffer);

            self.resources.fonts_mut().insert(font_name.to_string(), font);
        }

        self.resources.fonts_mut().get_mut(&font_name.to_string())
    }

    pub fn get_font_dimensions(&mut self, font_name: &'static str, size: u8, text: &str) -> (u16, u16) {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join(font_name).into_os_string().into_string().unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);

            let font = DrawingFont::create(&mut self.device, buffer);

            self.resources.fonts_mut().insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions(&mut self.device, FontParams { size: size }, &text)
        } else {
            (0, size as u16)
        }
    }

    pub fn get_resources(&self) -> &Resources<DrawingTexture, DrawingFont> {
        &self.resources
    }

    pub fn get_resources_mut(&mut self) -> &mut Resources<DrawingTexture, DrawingFont> {
        &mut self.resources
    }

    pub fn create_texture(&mut self, memory: &[u8], width: u16, height: u16, format: ColorFormat, updatable: bool) -> i32 {
        let texture_id = self.resources.get_next_texture_id();
        let texture = self.device.create_texture(Some(memory), width, height, format, updatable).unwrap();
        self.resources.textures_mut().insert(texture_id, texture);
        texture_id
    }

    pub fn update_texture(&mut self, texture_id: i32, memory: &[u8], offset_x: u16, offset_y: u16, width: u16, height: u16) {
        if let Some(texture) = self.resources.textures_mut().get_mut(&texture_id) {
            texture.update(memory, offset_x, offset_y, width, height).unwrap();
        }
    }

    pub fn update_window_size(&mut self, window: &mut DrawingWindowTarget, width: u16, height: u16) {
		window.update_window_size(width, height)
	}

    pub fn draw(&mut self, window: &DrawingWindowTarget,
        size: PhysPixelSize,
		primitives: Vec<Primitive>) {
        self.renderer.draw(&mut self.device, &window.get_render_target(), size, primitives, &mut self.resources);
    }
}
