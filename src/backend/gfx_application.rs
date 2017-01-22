extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_dxgi;
extern crate gfx_device_dx11;
extern crate winit;

use self::gfx_core::Factory;
use self::gfx_core::Device;
use gfx::traits::FactoryExt;
use ::render::primitive::Primitive;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct GFXApplication {
    window: gfx_window_dxgi::Window,
    device: gfx_device_dx11::Device,
    factory: gfx_device_dx11::Factory,
    target_view: gfx_core::handle::RenderTargetView<gfx_device_dx11::Resources, ColorFormat>,
    pipeline: gfx::PipelineState<gfx_device_dx11::Resources, pipe::Meta>,
    encoder: gfx::Encoder<gfx_device_dx11::Resources, gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>>,
}

impl GFXApplication {
    pub fn new(window_builder: winit::WindowBuilder) -> Self {
        let (window, mut device, mut factory, target_view) =
            gfx_window_dxgi::init::<ColorFormat>(window_builder).unwrap();

        //let main_depth = factory.create_depth_stencil_view_only::<DepthFormat>(
        //    self.window.size.0, self.window.size.1)
        //    .unwrap();
        //let backend = device.get_shader_model();
        //let mut device = gfx_device_dx11::Deferred::from(device);

        let vertex_shader = include_bytes!("data/vertex.fx");
        let pixel_shader = include_bytes!("data/pixel.fx");

        let pipeline = factory.create_pipeline_simple(
            vertex_shader,
            pixel_shader,
            pipe::new()
        ).unwrap();

        let mut encoder: gfx::Encoder<gfx_device_dx11::Resources, gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>>  = factory.create_command_buffer().into();

        GFXApplication {
            window: window,
            device: device,
            factory: factory,
            target_view: target_view,
            pipeline: pipeline,
            encoder: encoder,
        }
    }

    pub fn poll_events(&self) -> winit::PollEventsIterator {
        self.window.poll_events()
    }

    pub fn get_render_width(&self) -> f32 {
        self.window.size.0 as f32
    }

    pub fn get_render_height(&self) -> f32 {
        self.window.size.1 as f32
    }

    pub fn draw_primitives(&mut self, primitives: Vec<Primitive>,
                           width: f32, height: f32) {
        const TRIANGLE: [Vertex; 3] = [
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 1.0, 1.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 0.0] }
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let mut data = pipe::Data {
            vbuf: vertex_buffer,
            out: self.target_view.clone()
        };

        self.encoder.clear(&data.out, [0.1, 0.2, 0.3, 1.0]);
        self.encoder.draw(&slice, &self.pipeline, &data);
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers(1);
        self.device.cleanup();


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
