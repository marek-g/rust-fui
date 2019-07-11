use std::cell::RefCell;
use std::rc::Rc;

use common::Point;
use control::HitTestResult;
use control_object::ControlObject;
use observable::EventSubscription;

pub trait View {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>>;
}
