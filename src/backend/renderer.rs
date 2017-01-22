use backend::gfx_renderer::GFXRenderer;
use render::primitive::Primitive;

pub struct Renderer {
    renderer: GFXRenderer
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            renderer: GFXRenderer::new()
        }
    }

    pub fn draw_primitives(&mut self, primitives: Vec<Primitive>,
        width: f32, height: f32) {
        self.renderer.draw_primitives(primitives, width, height);
    }

    pub fn text_width(&mut self, size: f32, text: &str) -> f32 {
        self.renderer.text_width(size, &text)
    }
}