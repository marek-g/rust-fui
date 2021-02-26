use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use fui_core::Grid;
use fui_core::{Children, ControlObject, ObservableCollection, ObservableVec, ViewContext};
use fui_core::{EventProcessor, WindowService};
use fui_macros::ui;

use crate::DrawingWindowTarget;

pub struct Window {
    pub drawing_window_target: DrawingWindowTarget,
    pub event_processor: EventProcessor,
    pub root_control: Rc<RefCell<dyn ControlObject>>,

    control_layers: ObservableVec<Rc<RefCell<dyn ControlObject>>>,
}

impl Window {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        let control_layers = ObservableVec::<Rc<RefCell<dyn ControlObject>>>::new();

        let content = ui!(
            Grid {
                &control_layers,
            }
        );

        Window {
            drawing_window_target,
            event_processor: EventProcessor::new(),
            root_control: content,
            control_layers,
        }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn get_root_control(&self) -> &Rc<RefCell<dyn ControlObject>> {
        &self.root_control
    }

    pub fn get_layers(&self) -> &ObservableVec<Rc<RefCell<dyn ControlObject>>> {
        &self.control_layers
    }
}

impl WindowService for Window {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
    }

    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>) {
        self.control_layers
            .remove_filter(|el| Rc::ptr_eq(el, control));
    }
}
