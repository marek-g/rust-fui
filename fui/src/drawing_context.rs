pub use drawing::color::ColorFormat;

use drawing_gl::*;
use drawing::backend::WindowBackend;
use drawing::backend::Backend;
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

type DrawingBackend = GlWindowBackend;
type DrawingTexture = GlTexture;
type DrawingFont = TextureFont<DrawingBackend>;

pub struct DrawingContext {
    resources: Resources<DrawingTexture, DrawingFont>,
    renderer: Renderer<DrawingBackend>
}

impl DrawingContext {
    pub fn create(window_builder: WindowBuilder, events_loop: &EventsLoop) -> Self {
        let backend = DrawingBackend::create_window_backend(window_builder, &events_loop);
        DrawingContext {
            resources: Resources::new(),
            renderer: Renderer::new(backend)
        }
    }

    pub fn get_font(&mut self, font_name: &'static str) -> Option<&mut DrawingFont> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join(font_name).into_os_string().into_string().unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);

            let font = DrawingFont::create(self.renderer.backend(), buffer);

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

            let font = DrawingFont::create(self.renderer.backend(), buffer);

            self.resources.fonts_mut().insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions(self.renderer.backend(), FontParams { size: size }, &text)
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
        let texture = self.renderer.backend().create_texture(memory, width, height, format, updatable);
        self.resources.textures_mut().insert(texture_id, texture);
        texture_id
    }

    pub fn update_texture(&mut self, texture_id: i32, memory: &[u8], offset_x: u16, offset_y: u16, width: u16, height: u16) {
        if let Some(texture) = self.resources.textures_mut().get_mut(&texture_id) {
            texture.update(&mut () /*self.renderer.backend().get_encoder()*/, memory, offset_x, offset_y, width, height).unwrap();
        }
    }

    pub fn update_window_size(&mut self, width: u16, height: u16) {
		self.renderer.update_window_size(width, height)
	}

    pub fn draw(&mut self, size: PhysPixelSize,
		primitives: Vec<Primitive>) {
        self.renderer.draw(size, primitives, &mut self.resources);
    }
}