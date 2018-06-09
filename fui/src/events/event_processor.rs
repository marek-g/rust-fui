extern crate winit;

use events::*;
use RootView;

#[derive(Clone, Debug, PartialEq)]
pub enum ControlEvent {
    HoverEnter,
    HoverLeave,
    TapDown { position: (f32, f32) },
    TapUp { position: (f32, f32) }
}

pub struct EventProcessor {
    hover_detector: HoverDetector,
    gesture_detector: GestureDetector,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hover_detector: HoverDetector::new(),
            gesture_detector: GestureDetector::new(),
        }
    }

    pub fn handle_event(&mut self, root_view: &mut RootView, event: &winit::Event) {
        self.hover_detector.handle_event(root_view, event);

        self.gesture_detector.handle_event(event).map(|ev| match ev {
            Gesture::TapUp { position, captured_position } => {
                self.dispatch_event_by_hit_target(root_view, captured_position, ControlEvent::TapUp { position: position });
            },
            _ => ()
        });
    }

    fn dispatch_event_by_hit_target(&mut self, root_view: &RootView, hit_target: (f32, f32), event: ControlEvent) {
        if let Some(ref destination) = root_view.hit_test(hit_target.0, hit_target.1) {
            destination.borrow_mut().handle_event(event);
        }
    }
}