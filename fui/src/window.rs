use std::cell::RefCell;
use std::rc::Rc;

use RootView;
use View;
use ViewData;
use drawing_context::DrawingWindowTarget;

pub struct Window {
    drawing_window_target: DrawingWindowTarget,
    root_view: Option<RootView>,
}

impl Window {
    pub fn new(drawing_window_target: DrawingWindowTarget) -> Self {
        Window { drawing_window_target, root_view: None }
    }

    pub fn get_drawing_target(&self) -> &DrawingWindowTarget {
        &self.drawing_window_target
    }

    pub fn get_drawing_target_mut(&mut self) -> &mut DrawingWindowTarget {
        &mut self.drawing_window_target
    }

    pub fn get_drawing_target_and_root_view_mut(&mut self) -> (&mut DrawingWindowTarget, &mut Option<RootView>) {
        (&mut self.drawing_window_target, &mut self.root_view)
    }

    pub fn get_root_view_mut(&mut self) -> &mut Option<RootView> {
        &mut self.root_view
    }

    pub fn set_root_view(&mut self, view_data: ViewData) {
        self.root_view = Some(RootView::new(view_data));
    }

    pub fn set_root_view_model<V: View>(&mut self, view_model: &Rc<RefCell<V>>) {
        self.set_root_view(V::create_view(view_model));
    }

    pub fn clear_root(&mut self) {
        self.root_view = None;
    }
}
