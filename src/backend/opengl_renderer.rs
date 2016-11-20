extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use self::piston::input::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use render::primitive::Primitive;

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

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 250.0);
        let rotation = 0.1;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .rot_rad(rotation)
                .trans(-250.0/2.0, -250.0/2.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}