extern crate winit;

use control::ControlObject;
use events::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    TapDown { position: (f32, f32) },
    TapUp { position: (f32, f32) }
}

pub struct EventProcessor {
    gesture_detector: GestureDetector,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor { gesture_detector: GestureDetector::new() }
    }

    pub fn handle_event(&mut self, root_control: &mut Box<ControlObject>, event: &winit::Event) {
        self.gesture_detector.handle_event(event).map(|ev| match ev {
            Gesture::TapUp { position, captured_position } => {
                self.dispatch_event_by_hit_target(root_control, captured_position, ControlEvent::TapUp { position: position });
            },
            _ => ()
        });
    }

    fn dispatch_event_by_hit_target(&mut self, root_control: &mut Box<ControlObject>, hit_target: (f32, f32), event: ControlEvent) {
        let is_hit_test_visible = root_control.is_hit_test_visible();
        if is_hit_test_visible {
            let rect = root_control.get_rect();
            if hit_target.0 >= rect.x && hit_target.0 <= rect.x + rect.width &&
                hit_target.1 >= rect.y && hit_target.1 <= rect.y + rect.height {
                root_control.handle_event(event);
                return;
            }
        }

        for child in root_control.get_children() {
            self.dispatch_event_by_hit_target(child, hit_target, event.clone());
        }
    }
}