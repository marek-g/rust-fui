use std::cell::RefCell;
use std::rc::Rc;

use fui::ControlObject;
use fui::EventProcessor;
use fui::View;
use fui::ViewContext;

pub struct Window<DrawingWindowTarget> {
    pub drawing_window_target: DrawingWindowTarget,
    pub root_view: Option<Rc<RefCell<dyn ControlObject>>>,
    pub event_processor: EventProcessor,
}

impl<DrawingWindowTarget> Window<DrawingWindowTarget> {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window {
            drawing_window_target,
            root_view: None,
            event_processor: EventProcessor::new(),
        }
    }
}
