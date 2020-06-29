use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Weak, Rc};
use winit::dpi::LogicalSize;

use fui::*;

use crate::DrawingContext;
use crate::DrawingWindowTarget;
use crate::Window;

pub struct WindowEntry {
    pub window: Rc<RefCell<Window>>,
    pub services: Rc<RefCell<Services>>,
}

pub struct WindowManager {
    drawing_context: Rc<RefCell<DrawingContext>>,
    main_window_id: Option<winit::window::WindowId>,
    windows: HashMap<winit::window::WindowId, WindowEntry>,
}

impl WindowManager {
    pub fn new(drawing_context: Rc<RefCell<DrawingContext>>) -> Self {
        WindowManager {
            drawing_context: drawing_context,
            main_window_id: None,
            windows: HashMap::new(),
        }
    }

    pub fn add_window(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
        view: Rc<RefCell<dyn ControlObject>>,
    ) -> Result<winit::window::WindowId> {

        let mut window_target = {
            let first_window = self
                .windows
                .iter()
                .next()
                .map(|(_id, entry)| entry.window.clone());
            
            if let Some(first_window) = first_window {
                self.drawing_context.borrow_mut().create_window(
                    window_builder,
                    &event_loop,
                    Some(&first_window.borrow_mut().drawing_window_target),
                )?
            } else {
                self.drawing_context.borrow_mut().create_window(
                    window_builder,
                    &event_loop,
                    None,
                )?
            }
        };

        let physical_size = window_target.get_window().inner_size();
        let window_id = window_target.get_window().id();

        window_target.update_size(physical_size.width as u16, physical_size.height as u16);
        let mut window = Window::new(window_target);
        window.add_layer(view);

        let window_rc = Rc::new(RefCell::new(window));
        let window_service_rc: Rc<RefCell<dyn WindowService>> = window_rc.clone();

        let services = Rc::new(RefCell::new(Services::new(&window_service_rc)));

        let window_entry = WindowEntry {
            window: window_rc,
            services: services,
        };
        self.windows.insert(window_id, window_entry);

        if let None = self.main_window_id {
            self.main_window_id = Some(window_id);
        }

        Ok(window_id)
    }

    pub fn add_window_view_model<V: ViewModel>(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
        view_model: &Rc<RefCell<V>>,
    ) -> Result<winit::window::WindowId> {
        self.add_window(
            window_builder,
            &event_loop,
            ViewModel::to_view(&view_model),
        )
    }

    pub fn get_main_window_id(&self) -> Option<winit::window::WindowId> {
        self.main_window_id
    }

    pub fn get_windows_mut(
        &mut self,
    ) -> &mut HashMap<winit::window::WindowId, WindowEntry> {
        &mut self.windows
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.main_window_id = None;
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        // It is important to drop windows before drawing_context!
        // Windows cleanup graphics resources and drawing context drops graphics device.
        self.windows.clear();
    }
}
