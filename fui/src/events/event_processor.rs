extern crate winit;

use common::Point;
use events::*;
use RootView;

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
        self.gesture_detector.handle_event(root_view, event);
    }
}
