extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_dxgi;
extern crate gfx_device_dx11;
extern crate winit;

use self::gfx_core::Factory;
use self::gfx_core::Device;
use gfx::traits::FactoryExt;
use ::common::color::Color;
use ::common::matrix::*;
use ::render::primitive::{ Primitive, PrimitiveKind };
use std::ops::Mul;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex ColorVertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    vertex TexturedVertex {
        pos: [f32; 2] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline ColorPipeline {
        vbuf: gfx::VertexBuffer<ColorVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        //out: gfx::RenderTarget<ColorFormat> = "Target0",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }

    pipeline TexturedPipeline {
        vbuf: gfx::VertexBuffer<TexturedVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

pub struct GFXApplication {
    window: gfx_window_dxgi::Window,
    device: gfx_device_dx11::Device,
    factory: gfx_device_dx11::Factory,
    target_view: gfx_core::handle::RenderTargetView<gfx_device_dx11::Resources, ColorFormat>,
    color_pipeline_triangles: gfx::PipelineState<gfx_device_dx11::Resources, ColorPipeline::Meta>,
    color_pipeline_lines: gfx::PipelineState<gfx_device_dx11::Resources, ColorPipeline::Meta>,
    textured_pipeline_triangles: gfx::PipelineState<gfx_device_dx11::Resources, TexturedPipeline::Meta>,
    encoder: gfx::Encoder<gfx_device_dx11::Resources, gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>>,

    view_matrix: Matrix2x3<f32>,

    rotate: f64,
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

        let colored_vertex_shader = include_bytes!("data/colored_vertex.fx");
        let colored_pixel_shader = include_bytes!("data/colored_pixel.fx");
        let textured_vertex_shader = include_bytes!("data/textured_vertex.fx");
        let textured_pixel_shader = include_bytes!("data/textured_pixel.fx");

        let colored_shaderset = factory.create_shader_set(
            colored_vertex_shader,
            colored_pixel_shader,
        ).unwrap();

        let textured_shaderset = factory.create_shader_set(
            textured_vertex_shader,
            textured_pixel_shader,
        ).unwrap();

        let color_pipeline_triangles = factory.create_pipeline_state(
            &colored_shaderset,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill(),
            ColorPipeline::new()
        ).unwrap();

        let color_pipeline_lines = factory.create_pipeline_state(
            &colored_shaderset,
            gfx::Primitive::LineList,
            gfx::state::Rasterizer::new_fill(),
            ColorPipeline::new()
        ).unwrap();

        let textured_pipeline_triangles = factory.create_pipeline_state(
            &textured_shaderset,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill(),
            TexturedPipeline::new()
        ).unwrap();

        let mut encoder: gfx::Encoder<gfx_device_dx11::Resources, gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>>  = factory.create_command_buffer().into();

        let width = window.size.0 as f32;
        let height = window.size.1 as f32;

        GFXApplication {window: window,
            device: device,
            factory: factory,
            target_view: target_view,
            color_pipeline_triangles: color_pipeline_triangles,
            color_pipeline_lines: color_pipeline_lines,
            textured_pipeline_triangles: textured_pipeline_triangles,
            encoder: encoder,

            view_matrix: GFXApplication::view_matrix_from_resolution(width, height),
            rotate: 0.0,
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
        self.rotate += 0.1;
        let rotate = self.rotate;
        //let (x, y) = ((args.width / 2) as f64,
        //              (args.height / 2) as f64);

        //let glyph_cache = &mut self.glyph_cache;

        // Clear the screen.
        self.encoder.clear(&self.target_view, [0.1, 0.2, 0.3, 1.0]);

        for primitive in &primitives {
            match primitive.kind {

                PrimitiveKind::Line { ref color, thickness, x1, y1, x2, y2 } => {
                    let matrix = self.view_matrix.clone();
                    /*let matrix = matrix * nalgebra::append_translation(&matrix, nalgebra::Vector3::new(400.0,200.0,0.0));
                    let matrix = nalgebra::append_rotation(&matrix, nalgebra::Vector3::new(rotate,0.0,0.0));
                    let matrix = nalgebra::append_translation(&matrix, nalgebra::Vector3::new(-200.0,-100.0,0.0));*/
                    self.line(color,
                        thickness,
                        [x1, y1, x2, y2],
                        matrix);
                        //context.transform.trans(400.0,200.0).rot_rad(rotate).trans(-200.0,-100.0));
                },

                PrimitiveKind::Rectangle { ref color, x, y, width, height } => {
                    let matrix = self.view_matrix.clone();
                    self.rectangle(color,
                                   [x, y, x + width, y + height],
                                   matrix);
                                   //nalgebra::new_identity(3));
                              //context.transform.trans(400.0,200.0).rot_rad(rotate).trans(-200.0,-100.0));
                },

                PrimitiveKind::Text { ref color, x, y, size, text: ref src_text } => {
                    let matrix = self.view_matrix.clone();
                    self.text(color,
                        size as u32,
                        src_text,
                        //glyph_cache,
                        matrix);
                        //context.transform.trans(400.0,200.0).rot_rad(rotate).trans(x as f64, (y + size) as f64).trans(-200.0,-100.0));
                }
            }
        }

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers(1);
        self.device.cleanup();
    }

    pub fn text_width(&mut self, size: f32, text: &str) -> f32 {
        //self.glyph_cache.width(size as FontSize, &text) as f32
        120.0
    }

    fn line(&mut self, color: &Color, thickness: f32, points: [f32; 4], matrix: Matrix2x3<f32>) {
        if thickness == 1.0f32 {
            self.line_native(color, points, matrix);
        } else {
            self.line_triangulated(color, thickness, points, matrix);
        }
    }

    fn line_native(&mut self, color: &Color, points: [f32; 4], matrix: Matrix2x3<f32>) {
        let p1 = matrix * [ points[0], points[1] ];
        let p2 = matrix * [ points[2], points[3] ];

        let LINE: [ColorVertex; 2] = [
            ColorVertex { pos: [ p1[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p2[1] ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&LINE, ());

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            out: self.target_view.clone()
        };

        let locals = Locals { transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_lines, &data);
    }

    fn line_triangulated(&mut self, color: &Color, thickness: f32, points: [f32; 4], matrix: Matrix2x3<f32>) {
        let len = (((points[0] - points[2])*(points[0] - points[2]) + (points[3] - points[1])*(points[3] - points[1]))  as f32).sqrt();
        let normal_x = (points[3] - points[1]) / len;
        let normal_y = -(points[0] - points[2]) / len;

        let diff_x = normal_x * thickness * 0.5f32;
        let diff_y = normal_y * thickness * 0.5f32;
        let p1a_x = points[0] - diff_x;
        let p1a_y = points[1] - diff_y;
        let p1b_x = points[0] + diff_x;
        let p1b_y = points[1] + diff_y;
        let p2a_x = points[2] - diff_x;
        let p2a_y = points[3] - diff_y;
        let p2b_x = points[2] + diff_x;
        let p2b_y = points[3] + diff_y;

        let p1a = matrix * [ p1a_x, p1a_y ];
        let p1b = matrix * [ p1b_x, p1b_y ];
        let p2a = matrix * [ p2a_x, p2a_y ];
        let p2b = matrix * [ p2b_x, p2b_y ];

        let TRIANGLE: [ColorVertex; 6] = [
            ColorVertex { pos: [ p1a[0], p1a[1] ], color: *color },
            ColorVertex { pos: [ p2a[0], p2a[1] ], color: *color },
            ColorVertex { pos: [ p1b[0], p1b[1] ], color: *color },
            ColorVertex { pos: [ p1b[0], p1b[1] ], color: *color },
            ColorVertex { pos: [ p2a[0], p2a[1] ], color: *color },
            ColorVertex { pos: [ p2b[0], p2b[1] ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            out: self.target_view.clone()
        };

        let locals = Locals { transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_triangles, &data);
    }

    fn rectangle(&mut self, color: &Color, points: [f32; 4], matrix: Matrix2x3<f32>) {
        let p1 = matrix * [ points[0], points[1] ];
        let p2 = matrix * [ points[2], points[3] ];

        let TRIANGLE: [ColorVertex; 6] = [
            ColorVertex { pos: [ p1[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p1[0], p2[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p2[1] ], color: *color },
            ColorVertex { pos: [ p1[0], p2[1] ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            out: self.target_view.clone()
        };

        let locals = Locals { transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_triangles, &data);
    }

    fn text(&mut self, color: &Color, size: u32, src_text: &'static str, matrix: Matrix2x3<f32>) {

    }

    fn view_matrix_from_resolution(width: f32, height: f32) -> Matrix2x3<f32> {
        Matrix2x3::new(2.0f32 / width, 0.0f32, -1.0f32,
                       0.0f32, -2.0f32 / height, 1.0f32)
    }
}
