use std::cell::RefCell;
use std::rc::{ Rc, Weak };
use control::ControlObject;
use Binding;

pub struct ViewData {
    pub root_control: Rc<RefCell<ControlObject>>,
    pub bindings: Vec<Box<Binding>>
}

pub trait View {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> ViewData;
}

pub struct RootView {
    pub view_data: ViewData,
    pub hover_control: Option<Weak<RefCell<ControlObject>>>
}

impl RootView {
    pub fn new(view_data: ViewData) -> Self {
        RootView {
            view_data: view_data,
            hover_control: None
        }
    }

    pub fn hit_test(&self, x: f32, y: f32) -> Option<Rc<RefCell<ControlObject>>> {
        self.hit_test_control(&self.view_data.root_control, x, y)
    }

    pub fn hit_test_control(&self, control: &Rc<RefCell<ControlObject>>, x: f32, y: f32) -> Option<Rc<RefCell<ControlObject>>> {
        let is_hit_test_visible = control.borrow().is_hit_test_visible();
        if is_hit_test_visible {
            let rect = control.borrow().get_rect();
            if x >= rect.x && x <= rect.x + rect.width &&
                y >= rect.y && y <= rect.y + rect.height {
                return Some(control.clone());
            }
        }

        for child in control.borrow_mut().get_children() {
            if let Some(ref result) = self.hit_test_control(&child, x, y) {
                return Some(result.clone())
            }
        }

        None
    }
}
