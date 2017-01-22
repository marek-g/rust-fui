extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_dxgi;
extern crate gfx_device_dx11;
extern crate winit;

use self::gfx_core::Factory;
use self::gfx_core::Device;

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
    title: &'static str
}

impl GFXApplication {
    pub fn new(title: &'static str) -> Self {
        GFXApplication {
            title: title
        }
    }

    pub fn run(&mut self) {
        let windowBuilder = winit::WindowBuilder::new()
            .with_title(self.title);
        let (window, mut device, mut factory, targetView) =
            gfx_window_dxgi::init::<ColorFormat>(windowBuilder).unwrap();
        let main_depth = factory.create_depth_stencil_view_only::<DepthFormat>(window.size.0, window.size.1)
            .unwrap();
        let backend = device.get_shader_model();
        //let mut device = gfx_device_dx11::Deferred::from(device);


        //let mut encoder: gfx::Encoder<_, gfx_device_dx11::CommandBuffer<gfx_device_dx11::DeferredContext>> = factory.create_command_buffer().into();
        //let mut encoder = factory.create_command_buffer::<gfx_device_dx11::CommandBuffer<gfx_device_dx11::DeferredContext>>().into();
        //let mut encoder : gfx_device_dx11::CommandBuffer<gfx_device_dx11::DeferredContext> = factory.create_command_buffer_native().into();
        //let mut encoder: gfx_device_dx11::CommandBuffer<gfx_device_dx11::DeferredContext> = factory.create_command_buffer().into();

        let mut encoder: gfx::Encoder<gfx_device_dx11::Resources, gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>>  = factory.create_command_buffer().into();

        'main: loop {
            for event in window.poll_events() {
                match event {
                    winit::Event::Closed => return,
                    _ => (),
                }
            }

            encoder.clear(&targetView, [0.1, 0.2, 0.3, 1.0]);
            //encoder.draw(&slice, &pso, &data);
            encoder.flush(&mut device);
            window.swap_buffers(1);
            device.cleanup();
        }
    }
}
