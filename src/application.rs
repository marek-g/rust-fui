extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use self::piston_window::PistonWindow;
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };

pub struct Application {

    main_window : PistonWindow,
    gl: GlGraphics,

    rotation: f64
}

impl Application {

    pub fn new(title : &str) -> Self {
        let opengl_version = OpenGL::V3_2;

        let mut window : PistonWindow = WindowSettings::new(
            title,
            [600, 600]
        )
            .opengl(opengl_version)
            .resizable(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

        window.set_ups(60);
        window.set_max_fps(60);

        Application {
            main_window: window,
            gl: GlGraphics::new(opengl_version),
            rotation: 0.0
        }
    }

    pub fn run(&mut self) {
        let mut events = self.main_window.events();
        while let Some(e) = events.next(&mut self.main_window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 250.0);
        let rotation = self.rotation;
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

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 1.0 * args.dt;
    }
}
