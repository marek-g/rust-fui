use winit::EventsLoop;
use winit::WindowBuilder;

use drawing_gfx::backend::{ GfxWindowBackend, GfxTexture, GfxResources, GfxFactory };
use drawing_gfx::font_gfx_text::GfxTextFont;
use drawing::backend::WindowBackend;
use drawing::renderer::Renderer;
use drawing::resources::Resources;
use drawing::units::*;
use drawing::primitive::Primitive;

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

    pub fn update_window_size(&mut self, width: u16, height: u16) {
		self.renderer.update_window_size(width, height)
	}

    pub fn draw(&mut self, size: PhysPixelSize,
		primitives: Vec<Primitive>) {
        self.renderer.draw(size, primitives, &mut self.resources);
    }

    pub fn text_width(&self, size: f32, text: &str) -> f32 {
        //self.backend_app.text_width(size, text)
        100.0
    }
}