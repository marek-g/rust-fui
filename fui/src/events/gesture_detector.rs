use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use common::Point;
use control::ControlObject;
use view::RootView;
use events::ControlEvent;
use events::gesture_detector_core::*;

pub struct GestureDetector {
    gesture_detector_core: GestureDetectorCore,
    captured_control: Option<Weak<RefCell<ControlObject>>>
}

impl GestureDetector {
    pub fn new() -> Self {
        GestureDetector {
            gesture_detector_core: GestureDetectorCore::new(),
            captured_control: None,
        }
    }

    pub fn get_captured_control(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref captured_control) = self.captured_control {
            captured_control.upgrade()
        } else {
            None
        }
    }

    pub fn handle_event(&mut self, root_view: &mut RootView, event: &::winit::Event) {
        self.gesture_detector_core.handle_event(event).map(|ev| match ev {
            Gesture::TapDown { position } => {
                if let Some(ref hit_control) = root_view.hit_test(position) {
                    self.captured_control = Some(Rc::downgrade(hit_control));
                    self.send_event_to_captured_control(ControlEvent::TapDown { position: position });
                }
            },

            Gesture::TapUp { position } => {
                self.send_event_to_captured_control(ControlEvent::TapUp { position: position });
                self.captured_control = None;
            },

            Gesture::TapMove { position } => {
                self.send_event_to_captured_control(ControlEvent::TapMove { position: position });
                self.captured_control = None;
            },

            _ => ()
        });
    }

    fn send_event_to_captured_control(&mut self, event: ControlEvent) {
        if let Some(ref captured_control) = self.get_captured_control() {
            captured_control.borrow_mut().handle_event(event);
        }
    }
}
