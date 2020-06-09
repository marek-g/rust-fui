use std::cell::RefCell;
use std::rc::Rc;

use fui::ControlObject;
use fui::EventProcessor;

use drawing_gl::*;
use drawing::backend::RenderTarget;

use crate::DrawingWindowTarget;

pub struct Window {
    pub drawing_window_target: DrawingWindowTarget,
    pub root_view: Option<Rc<RefCell<dyn ControlObject>>>,
    pub event_processor: EventProcessor,
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
}
