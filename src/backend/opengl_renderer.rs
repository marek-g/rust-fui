extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use self::piston::input::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use self::opengl_graphics::glyph_cache::GlyphCache;
use render::primitive::{ Primitive, PrimitiveKind };

pub struct OpenGLRenderer<'a> {
    gl: GlGraphics,
    glyph_cache: GlyphCache<'a>,
}

impl<'a> OpenGLRenderer<'a> {
    pub fn new(gl_version: OpenGL) -> OpenGLRenderer<'a> {
        OpenGLRenderer {
            gl: GlGraphics::new(gl_version),
            // TODO: error handling
            glyph_cache: GlyphCache::new("c:\\windows\\fonts\\arial.ttf").unwrap()
        }
    }

    pub fn draw_primitives(&mut self, args: &RenderArgs,
                           primitives: Vec<Primitive>) {
        use self::graphics::*;

        //let (x, y) = ((args.width / 2) as f64,
        //              (args.height / 2) as f64);

        let gl = &mut self.gl;
        let glyph_cache = &mut self.glyph_cache;

        gl.draw(args.viewport(), |context, gl| {
            // Clear the screen.
            clear([0.0, 1.0, 0.0, 1.0], gl);

            for primitive in &primitives {
                match primitive.kind {

                    PrimitiveKind::Line { ref color, thickness, x1, y1, x2, y2 } => {
                        line([color[1], color[2], color[3], color[0]],
                            thickness as f64 / 2.0,
                            [x1 as f64, y1 as f64, x2 as f64, y2 as f64],
                            context.transform, gl);
                    },

                    PrimitiveKind::Rectangle { ref color, x, y, width, height } => {
                        rectangle([color[1], color[2], color[3], color[0]],
                                  [x as f64, y as f64, width as f64, height as f64],
                                  context.transform, gl);
                    },

                    PrimitiveKind::Text { ref color, x, y, size, text: ref src_text } => {
                        text([color[1], color[2], color[3], color[0]],
                            size as u32,
                            src_text,
                            glyph_cache,
                            context.transform.trans(x as f64, y as f64), gl);
                    }

                }
            }
        });
    }
}