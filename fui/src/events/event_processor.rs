extern crate winit;

use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use common::Point;
use control_object::*;
use events::*;
use RootView;
use Window;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    HoverEnter,
    HoverLeave,
    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },
}

pub struct EventProcessor {
    hover_detector: HoverDetector,
    gesture_detector: GestureDetector,
    captured_control: Option<Weak<RefCell<ControlObject>>>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hover_detector: HoverDetector::new(),
            gesture_detector: GestureDetector::new(),
            captured_control: None,
        }
    }

    pub fn handle_event(&mut self, window: &mut Window, event: &winit::WindowEvent) {
        self.hover_detector.handle_event(window, event);
        self.handle_gesture_event(window, event);
    }

    pub fn handle_gesture_event(&mut self, window: &mut Window, event: &winit::WindowEvent) {
        self.gesture_detector.handle_event(window, event).map(|ev| match ev {
            Gesture::TapDown { position } => {
                if let Some(ref mut root_view) = window.get_root_view_mut() {
                    if let Some(ref hit_control) = root_view.hit_test(position) {
                        self.captured_control = Some(Rc::downgrade(hit_control));
                        self.hover_detector.stop();
                        self.send_event_to_captured_control(ControlEvent::TapDown { position: position });
                    }
                }
            },

            Gesture::TapUp { position } => {
                self.send_event_to_captured_control(ControlEvent::TapUp { position: position });
                self.captured_control = None;
                self.hover_detector.start();
            },

            Gesture::TapMove { position } => {
                self.send_event_to_captured_control(ControlEvent::TapMove { position: position });
            },
        });
    }

    fn get_captured_control(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref captured_control) = self.captured_control {
            captured_control.upgrade()
        } else {
            None
        }
    }

    fn send_event_to_captured_control(&mut self, event: ControlEvent) {
        if let Some(ref captured_control) = self.get_captured_control() {
            captured_control.borrow_mut().handle_event(event);
        }
    }
}
