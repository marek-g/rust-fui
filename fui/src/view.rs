use std::cell::RefCell;
use std::rc::Rc;

use common::Point;
use control::HitTestResult;
use control_object::ControlObject;
use observable::EventSubscription;

pub struct ViewData {
    pub root_control: Rc<RefCell<ControlObject>>,
    pub bindings: Vec<EventSubscription>
}

pub trait View {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> ViewData;
}

pub struct RootView {
    pub view_data: ViewData,
}

impl RootView {
    pub fn new(view_data: ViewData) -> Self {
        RootView {
            view_data: view_data,
        }
    }

    pub fn hit_test(&self, point: Point) -> Option<Rc<RefCell<ControlObject>>> {
        let hit_test_result = self.view_data.root_control.borrow().hit_test(point);
        match hit_test_result {
            HitTestResult::Current => Some(self.view_data.root_control.clone()),
            HitTestResult::Child(control) => Some(control),
            HitTestResult::Nothing => None,
        }
    }
}
