use std::cell::RefCell;
use std::rc::Rc;

use fui::ControlObject;
use fui::{WindowService, EventProcessor};

use drawing_gl::*;
use drawing::backend::RenderTarget;

use crate::DrawingWindowTarget;

pub struct Window {
    pub drawing_window_target: DrawingWindowTarget,
    pub event_processor: EventProcessor,
    root_view: Option<Rc<RefCell<dyn ControlObject>>>,
}

impl Window {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window {
            drawing_window_target,
            root_view: None,
            event_processor: EventProcessor::new(),
        }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn set_root_view(&mut self, root_view: Option<Rc<RefCell<dyn ControlObject>>>) {
        if let Some(ref root_view) = root_view {
            //root_view.borrow_mut().setup(services);
        }
        self.root_view = root_view;
    }

    pub fn get_root_view(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        self.root_view.clone()
    }
}

impl WindowService for Window {
    fn add_new_layer(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>) {
        todo!()
    }
    fn remove_layer(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>) {
        todo!()
    }
}
