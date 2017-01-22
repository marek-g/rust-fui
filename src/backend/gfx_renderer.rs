extern crate gfx;
extern crate winit;

use render::primitive::Primitive;

pub struct GFXRenderer {
    /*gl: gfx_device_dx11,
    glyph_cache: GlyphCache<'a>,

    rotate: f64*/
}

impl GFXRenderer {
    pub fn new() -> GFXRenderer {
        GFXRenderer {
            /*gl: gfx_device_dx11::create::create(|s|
                window.get_proc_address(s) as *const std::os::raw::c_void),
            // TODO: error handling
            glyph_cache: GlyphCache::new("c:\\windows\\fonts\\arial.ttf").unwrap(),
            rotate: 0.0*/
        }
    }

    pub fn draw_primitives(&mut self, primitives: Vec<Primitive>,
        width: f32, height: f32) {
        /*use self::graphics::*;

        self.rotate += 0.1;
        let rotate = self.rotate;
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
                            context.transform.trans(400.0,200.0).rot_rad(rotate).trans(-200.0,-100.0), gl);
                    },

                    PrimitiveKind::Rectangle { ref color, x, y, width, height } => {
                        rectangle([color[1], color[2], color[3], color[0]],
                                  [x as f64, y as f64, width as f64, height as f64],
                                  context.transform.trans(400.0,200.0).rot_rad(rotate).trans(-200.0,-100.0), gl);
                    },

                    PrimitiveKind::Text { ref color, x, y, size, text: ref src_text } => {
                        text([color[1], color[2], color[3], color[0]],
                            size as u32,
                            src_text,
                            glyph_cache,
                            context.transform.trans(400.0,200.0).rot_rad(rotate).trans(x as f64, (y + size) as f64).trans(-200.0,-100.0), gl);
                    }

                }
            }
        });*/
    }

    pub fn text_width(&mut self, size: f32, text: &str) -> f32 {
        //self.glyph_cache.width(size as FontSize, &text) as f32
        1.0
    }
}