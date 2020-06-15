use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::control::*;
use crate::{DrawingContext, events::*};

pub struct EventContext {
    // captures mouse after TapDown
    captured_control: Option<Weak<RefCell<dyn ControlObject>>>,

    // control with focus
    focused_control: Option<Weak<RefCell<dyn ControlObject>>>,
}

impl EventContext {
    pub fn new() -> Self {
        EventContext {
            captured_control: None,
            focused_control: None,
        }
    }

    pub fn get_captured_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref control) = self.captured_control {
            control.upgrade()
        } else {
            None
        }
    }

    pub fn set_captured_control(&mut self, control: Option<Weak<RefCell<dyn ControlObject>>>) {
        self.captured_control = control;
    }

    pub fn get_focused_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref control) = self.focused_control {
            control.upgrade()
        } else {
            None
        }
    }

    pub fn set_new_focused_control(
        &mut self,
        control: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
    ) {
        self.send_event_to_control(self.get_focused_control(), drawing_context, ControlEvent::FocusLeave);
        self.focused_control = Some(Rc::downgrade(control));
        self.send_event_to_control(Some(control.clone()), drawing_context, ControlEvent::FocusEnter);
    }

    pub fn send_event_to_control(
        &mut self,
        control: Option<Rc<RefCell<dyn ControlObject>>>,
        drawing_context: &mut dyn DrawingContext,
        event: ControlEvent,
    ) {
        if let Some(ref control) = control {
            control.borrow_mut().handle_event(drawing_context, event);
        };
    }
}
