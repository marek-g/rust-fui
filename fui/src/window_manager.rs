extern crate winit;

use drawing::backend::WindowTarget;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use winit::dpi::LogicalSize;

use control_object::ControlObject;
use DrawingContext;
use RcView;
use View;
use ViewContext;
use Window;

use Result;

pub struct WindowManager {
    drawing_context: Rc<RefCell<DrawingContext>>,
    main_window_id: Option<winit::window::WindowId>,
    windows: HashMap<winit::window::WindowId, Window>,
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
        let shared_window_target = self
            .windows
            .iter()
            .next()
            .map(|(id, window)| window.get_drawing_target());
        let mut window_target = self.drawing_context.borrow_mut().create_window(
            window_builder,
            &event_loop,
            shared_window_target,
        )?;
        let logical_size = window_target.get_window().inner_size();
        let window_id = window_target.get_window().id();

        let physical_size = logical_size.to_physical(window_target.get_window().hidpi_factor());
        window_target.update_size(physical_size.width as u16, physical_size.height as u16);
        let mut window = Window::new(window_target);
        window.set_root_view(view);
        self.windows.insert(window_id, window);

        if let None = self.main_window_id {
            self.main_window_id = Some(window_id);
        }

        Ok(window_id)
    }

    pub fn add_window_view_model<V: RcView>(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        event_loop: &winit::event_loop::EventLoop<()>,
        view_model: &Rc<RefCell<V>>,
    ) -> Result<winit::window::WindowId> {
        self.add_window(
            window_builder,
            &event_loop,
            RcView::to_view(&view_model, ViewContext::empty()),
        )
    }

    pub fn get_main_window_id(&self) -> Option<winit::window::WindowId> {
        self.main_window_id
    }

    pub fn get_windows_mut(&mut self) -> &mut HashMap<winit::window::WindowId, Window> {
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
