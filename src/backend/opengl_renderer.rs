extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use self::piston::input::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use render::primitive::{ Primitive, PrimitiveKind };

pub struct OpenGLRenderer {
    gl: GlGraphics,
}

impl OpenGLRenderer {
    pub fn new(gl_version: OpenGL) -> OpenGLRenderer {
        OpenGLRenderer { gl: GlGraphics::new(gl_version) }
    }

    pub fn draw_primitives(&mut self, args: &RenderArgs,
                           primitives: Vec<Primitive>) {
        use self::graphics::*;

        //let (x, y) = ((args.width / 2) as f64,
        //              (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |context, gl| {
            // Clear the screen.
            clear([0.0, 1.0, 0.0, 1.0], gl);

            for primitive in &primitives {
                match primitive.kind {
                    PrimitiveKind::Rectangle { ref color, x, y, width, height } => {
                        rectangle([color[1], color[2], color[3], color[0]],
                                  [x as f64, y as f64, width as f64, height as f64],
                                  context.transform, gl);
                    },
                    _ => ()
                }
            }
        });
    }
}