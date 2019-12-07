use std::cell::RefCell;
use std::rc::Rc;

use fui::ControlObject;
use fui::View;
use fui::ViewContext;

pub struct Window<DrawingWindowTarget> {
    drawing_window_target: DrawingWindowTarget,
    root_view: Option<Rc<RefCell<dyn ControlObject>>>,
}

impl<DrawingWindowTarget> Window<DrawingWindowTarget> {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window {
            drawing_window_target,
            root_view: None,
        }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn get_drawing_target_mut(&mut self) -> &mut DrawingWindowTarget {
        &mut self.drawing_window_target
    }

    pub fn get_drawing_target_and_root_view_mut(
        &mut self,
    ) -> (
        &mut DrawingWindowTarget,
        &mut Option<Rc<RefCell<dyn ControlObject>>>,
    ) {
        (&mut self.drawing_window_target, &mut self.root_view)
    }

    pub fn get_root_view_mut(&mut self) -> &mut Option<Rc<RefCell<dyn ControlObject>>> {
        &mut self.root_view
    }

    pub fn set_root_view(&mut self, root_view: Rc<RefCell<dyn ControlObject>>) {
        self.root_view = Some(root_view);
    }

    pub fn set_root_view_model<V: View>(&mut self, view_model: V) {
        self.set_root_view(view_model.to_view(ViewContext::empty()));
    }

    pub fn clear_root(&mut self) {
        self.root_view = None;
    }
}
