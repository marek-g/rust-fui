use std::cell::RefCell;
use std::rc::Rc;

use control_object::ControlObject;
use drawing_context::DrawingWindowTarget;
use View;

pub struct Window {
    drawing_window_target: DrawingWindowTarget,
    root_view: Option<Rc<RefCell<ControlObject>>>,
    need_swap_buffers: bool,
}

impl Window {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window { drawing_window_target, root_view: None, need_swap_buffers: false }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn get_drawing_target_mut(&mut self) -> &mut DrawingWindowTarget {
        &mut self.drawing_window_target
    }

    pub fn get_drawing_target_and_root_view_mut(&mut self) -> (&mut DrawingWindowTarget, &mut Option<Rc<RefCell<ControlObject>>>) {
        (&mut self.drawing_window_target, &mut self.root_view)
    }

    pub fn get_root_view_mut(&mut self) -> &mut Option<Rc<RefCell<ControlObject>>> {
        &mut self.root_view
    }

    pub fn set_root_view(&mut self, root_view: Rc<RefCell<ControlObject>>) {
        self.root_view = Some(root_view);
    }

    pub fn set_root_view_model<V: View>(&mut self, view_model: V) {
        self.set_root_view(view_model.to_view(Vec::new()));
    }

    pub fn clear_root(&mut self) {
        self.root_view = None;
    }

    pub fn set_need_swap_buffers(&mut self, need_swap_buffers: bool) {
        self.need_swap_buffers = need_swap_buffers;
    }

    pub fn get_need_swap_buffers(&mut self) -> bool {
        self.need_swap_buffers
    }
}
