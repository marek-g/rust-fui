use std::cell::RefCell;
use std::rc::Rc;

use fui_core::ControlObject;
use fui_core::{EventProcessor, WindowService};

use crate::DrawingWindowTarget;

pub struct Window {
    pub drawing_window_target: DrawingWindowTarget,
    pub event_processor: EventProcessor,
    pub is_dirty: bool,
    control_layers: Vec<Rc<RefCell<dyn ControlObject>>>,
}

impl Window {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window {
            drawing_window_target,
            event_processor: EventProcessor::new(),
            is_dirty: false,
            control_layers: Vec::new(),
        }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn get_layers(&self) -> &Vec<Rc<RefCell<dyn ControlObject>>> {
        &self.control_layers
    }
}

impl WindowService for Window {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
        self.is_dirty = true;
    }

    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>) {
        let mut i = 0;
        while i != self.control_layers.len() {
            if Rc::ptr_eq(&self.control_layers[i], control) {
                self.control_layers.remove(i);
            } else {
                i += 1;
            }
        }
        self.is_dirty = true;
    }
}
