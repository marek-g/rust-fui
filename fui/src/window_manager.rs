extern crate winit;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::LogicalSize;
use drawing::backend::WindowTarget;

use control_object::ControlObject;
use DrawingContext;
use Window;
use View;
use ViewContext;

use ::Result;

pub struct WindowManager {
    drawing_context: Rc<RefCell<DrawingContext>>,
    main_window_id: Option<winit::WindowId>,
    windows: HashMap<winit::WindowId, Window>,
}

impl WindowManager {
    pub fn new(drawing_context: Rc<RefCell<DrawingContext>>) -> Self {
        WindowManager {
            drawing_context: drawing_context,
            main_window_id: None,
            windows: HashMap::new()
        }
    }

    pub fn add_window(&mut self,
        window_builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
        view: Rc<RefCell<ControlObject>>) -> Result<winit::WindowId>
    {
        let mut window_target = self.drawing_context.borrow_mut().create_window(window_builder, &events_loop)?;
        let logical_size = window_target.get_window().get_inner_size().unwrap_or(LogicalSize::new(0.0, 0.0));
        let window_id = window_target.get_window().id();

        let physical_size = logical_size.to_physical(window_target.get_window().get_hidpi_factor());
        window_target.update_size(physical_size.width as u16, physical_size.height as u16);
        
        let mut window = Window::new(window_target);
        window.set_root_view(view);
        self.windows.insert(window_id, window);

        if let None = self.main_window_id {
            self.main_window_id = Some(window_id);
        }

        Ok(window_id)
    }

    pub fn add_window_view_model<V: View>(&mut self,
        window_builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
        view_model: V) -> Result<winit::WindowId> {
        self.add_window(window_builder, &events_loop, view_model.to_view(ViewContext { children: Vec::new() }))
    }

    pub fn get_main_window_id(&self) -> Option<winit::WindowId> {
        self.main_window_id
    }

    pub fn get_windows_mut(&mut self) -> &mut HashMap<winit::WindowId, Window> {
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
