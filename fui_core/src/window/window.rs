use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use crate::{
    Children, ControlObject, EventProcessor, Grid, ObservableVec, ViewContext, WindowService,
};
use fui_macros::ui;

pub trait NativeWindow {
    fn repaint(&mut self);
}

pub struct Window<NW: NativeWindow> {
    pub native_window: NW,
    pub event_processor: EventProcessor,
    pub root_control: Rc<RefCell<dyn ControlObject>>,

    control_layers: ObservableVec<Rc<RefCell<dyn ControlObject>>>,
}

impl<NW: NativeWindow> Window<NW> {
    pub fn new(native_window: NW) -> Self {
        let control_layers = ObservableVec::<Rc<RefCell<dyn ControlObject>>>::new();

        let content = ui!(
            Grid {
                &control_layers,
            }
        );

        Window {
            native_window,
            event_processor: EventProcessor::new(),
            root_control: content,
            control_layers,
        }
    }

    pub fn get_native_window(&self) -> &NW {
        &self.native_window
    }

    pub fn get_root_control(&self) -> &Rc<RefCell<dyn ControlObject>> {
        &self.root_control
    }

    pub fn get_layers(&self) -> &ObservableVec<Rc<RefCell<dyn ControlObject>>> {
        &self.control_layers
    }
}

impl<NW: NativeWindow> WindowService for Window<NW> {
    fn add_layer(&mut self, control: Rc<RefCell<dyn ControlObject>>) {
        self.control_layers.push(control);
    }

    fn remove_layer(&mut self, control: &Rc<RefCell<dyn ControlObject>>) {
        self.control_layers
            .remove_filter(|el| Rc::ptr_eq(el, control));
    }

    fn repaint(&mut self) {
        self.native_window.repaint();
    }
}
