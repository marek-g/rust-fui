use anyhow::Result;
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use fui_core::*;

use crate::{DrawingContext, DrawingWindowTarget};

pub struct WinitWindow {
    pub drawing_target: DrawingWindowTarget,
}

impl WinitWindow {
    pub fn new(drawing_target: DrawingWindowTarget) -> Self {
        Self { drawing_target }
    }
}

pub struct WindowManager {
    drawing_context: Rc<RefCell<DrawingContext>>,
    event_loop_proxy: winit::event_loop::EventLoopProxy<()>,
    main_window_id: Option<winit::window::WindowId>,
    windows: HashMap<winit::window::WindowId, Rc<RefCell<Window<WinitWindow>>>>,
    window_services: HashMap<winit::window::WindowId, Rc<RefCell<Services>>>,
    background_texture: i32,
    exit_flag: bool,
}

impl WindowManager {
    pub fn new(
        drawing_context: Rc<RefCell<DrawingContext>>,
        event_loop_proxy: winit::event_loop::EventLoopProxy<()>,
    ) -> Self {
        WindowManager {
            drawing_context: drawing_context,
            event_loop_proxy,
            main_window_id: None,
            windows: HashMap::new(),
            window_services: HashMap::new(),
            background_texture: -1,
            exit_flag: false,
        }
    }

    fn create_background_texture(drawing_context: &mut DrawingContext) -> Result<i32> {
        let mut data = [0u8; 256 * 256 * 4];
        for i in 0..256 * 256 {
            data[i * 4 + 0] = 60;
            data[i * 4 + 1] = thread_rng().gen_range(90 - 15, 90 + 16);
            data[i * 4 + 2] = 60;
            data[i * 4 + 3] = 255;
        }
        drawing_context.create_texture(&data, 256, 256, ColorFormat::RGBA, false)
    }

    pub fn create_window(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Rc<RefCell<Window<WinitWindow>>>> {
        let mut window_target = {
            let first_window = self
                .windows
                .iter()
                .next()
                .map(|(_id, window)| window.clone());

            if let Some(first_window) = first_window {
                self.drawing_context.borrow_mut().create_window(
                    window_builder,
                    &event_loop,
                    Some(&first_window.borrow_mut().native_window.drawing_target),
                )?
            } else {
                self.drawing_context.borrow_mut().create_window(
                    window_builder,
                    &event_loop,
                    None,
                )?
            }
        };

        // create background texture
        if self.background_texture < 0 {
            self.background_texture =
                WindowManager::create_background_texture(&mut self.drawing_context.borrow_mut())?;
        }

        let physical_size = window_target.get_window().inner_size();
        let window_id = window_target.get_window().id();

        window_target.update_size(physical_size.width as u16, physical_size.height as u16);
        let winit_window = WinitWindow::new(window_target);
        let window = Window::new(winit_window);

        let window_rc = Rc::new(RefCell::new(window));
        let window_service_rc: Rc<RefCell<dyn WindowService>> = window_rc.clone();

        let services = Rc::new(RefCell::new(Services::new(&window_service_rc)));
        window_rc
            .borrow()
            .get_root_control()
            .borrow_mut()
            .get_context_mut()
            .set_services(Some(Rc::downgrade(&services)));

        self.windows.insert(window_id, window_rc.clone());
        self.window_services.insert(window_id, services);

        if let None = self.main_window_id {
            self.main_window_id = Some(window_id);
        }

        Ok(window_rc)
    }

    pub fn add_window(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
        view: Rc<RefCell<dyn ControlObject>>,
    ) -> Result<Rc<RefCell<Window<WinitWindow>>>> {
        let mut window_rc = self.create_window(window_builder, event_loop)?;

        window_rc.borrow_mut().add_layer(view);

        Ok(window_rc)
    }

    pub fn add_window_view_model<V: ViewModel>(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
        view_model: &Rc<RefCell<V>>,
    ) -> Result<Rc<RefCell<Window<WinitWindow>>>> {
        self.add_window(
            window_builder,
            &event_loop,
            ViewModel::create_view(&view_model),
        )
    }

    pub fn get_main_window_id(&self) -> Option<winit::window::WindowId> {
        self.main_window_id
    }

    pub fn get_windows_mut(
        &mut self,
    ) -> &mut HashMap<winit::window::WindowId, Rc<RefCell<Window<WinitWindow>>>> {
        &mut self.windows
    }

    pub fn close_all_windows(&mut self) {
        self.windows.clear();
        self.window_services.clear();
        self.main_window_id = None;
    }

    pub fn is_exit_flag(&self) -> bool {
        self.exit_flag
    }

    pub fn exit(&mut self) {
        self.exit_flag = true;
        self.event_loop_proxy.send_event(());
    }

    pub fn get_background_texture(&self) -> i32 {
        self.background_texture
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.close_all_windows();
    }
}
