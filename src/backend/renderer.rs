extern crate opengl_graphics;
extern crate piston;

use backend::opengl_renderer::OpenGLRenderer;
use self::opengl_graphics::OpenGL;

use self::piston::input::*;
use render::primitive::Primitive;


pub struct Renderer<'a> {
    renderer : OpenGLRenderer<'a>
}

impl<'a> Renderer<'a> {

    pub fn new() -> Renderer<'a> {
        Renderer {
            renderer : OpenGLRenderer::new(OpenGL::V3_2)
        }
    }

    pub fn draw_primitives(&mut self, args: &RenderArgs,
                           primitives: Vec<Primitive>) {
        self.renderer.draw_primitives(&args, primitives);
    }

    pub fn text_width(&mut self, size: f32, text: &str) -> f32 {
        self.renderer.text_width(size, &text)
    }
}