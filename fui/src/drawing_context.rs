use drawing_gfx::backend::{ GfxWindowBackend, GfxTexture, GfxResources, GfxFactory };
use drawing_gfx::font_gfx_text::*;
use drawing::backend::WindowBackend;
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

pub struct DrawingContext {
    resources: Resources<GfxTexture, GfxTextFont<GfxResources, GfxFactory>>,
    renderer: Renderer<GfxWindowBackend>
}

impl DrawingContext {
    pub fn create(window_builder: WindowBuilder, events_loop: &EventsLoop) -> Self {
        let backend = GfxWindowBackend::create_window_backend(window_builder, &events_loop);
        DrawingContext {
            resources: Resources::new(),
            renderer: Renderer::new(backend)
        }
    }

    pub fn get_font(&mut self, font_name: &'static str) -> Option<&mut GfxTextFont<GfxResources, GfxFactory>> {
        if let None = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            let font_path = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap().join(font_name).into_os_string().into_string().unwrap();
            let mut file = File::open(font_path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);

            let font = GfxTextFont::create(self.renderer.backend(), buffer);

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

            let font = GfxTextFont::create(self.renderer.backend(), buffer);

            self.resources.fonts_mut().insert(font_name.to_string(), font);
        }

        if let Some(font) = self.resources.fonts_mut().get_mut(&font_name.to_string()) {
            font.get_dimensions(self.renderer.backend(), FontParams { size: size }, &text)
        } else {
            (0, size as u16)
        }
    }

    pub fn get_resources(&self) -> &Resources<GfxTexture, GfxTextFont<GfxResources, GfxFactory>> {
        &self.resources
    }

    pub fn get_resources_mut(&mut self) -> &mut Resources<GfxTexture, GfxTextFont<GfxResources, GfxFactory>> {
        &mut self.resources
    }

    pub fn update_window_size(&mut self, width: u16, height: u16) {
		self.renderer.update_window_size(width, height)
	}

    pub fn draw(&mut self, size: PhysPixelSize,
		primitives: Vec<Primitive>) {
        self.renderer.draw(size, primitives, &mut self.resources);
    }
}