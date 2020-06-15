use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::control::*;
use crate::{DrawingContext, events::*};

pub struct EventProcessor {
    hover_detector: HoverDetector,
    gesture_detector: GestureDetector,

    event_context: EventContext,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hover_detector: HoverDetector::new(),
            gesture_detector: GestureDetector::new(),
            event_context: EventContext::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event: &InputEvent,
    ) {
        self.hover_detector
            .handle_event(root_view, drawing_context, &mut self.event_context, event);
        self.handle_gesture_event(root_view, drawing_context, event);
        self.handle_keyboard_event(root_view, drawing_context, event);
    }

    pub fn handle_gesture_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event: &InputEvent,
    ) {
        self.gesture_detector
            .handle_event(root_view, event)
            .map(|ev| match ev {
                Gesture::TapDown { position } => {
                    let hit_test_result = root_view.borrow().hit_test(position);
                    let hit_control = match hit_test_result {
                        HitTestResult::Current => Some(root_view.clone()),
                        HitTestResult::Child(control) => Some(control),
                        HitTestResult::Nothing => None,
                    };

                    if let Some(ref hit_control) = hit_control {
                        self.event_context.set_new_focused_control(hit_control, drawing_context);

                        self.event_context.set_captured_control(Some(Rc::downgrade(hit_control)));
                        self.hover_detector.stop(&mut self.event_context, drawing_context);

                        self.event_context.send_event_to_control(
                            self.event_context.get_captured_control(),
                            drawing_context,
                            ControlEvent::TapDown { position: position },
                        );
                    }
                }

                Gesture::TapUp { position } => {
                    self.event_context.send_event_to_control(
                        self.event_context.get_captured_control(),
                        drawing_context,
                        ControlEvent::TapUp { position: position },
                    );

                    self.event_context.set_captured_control(None);
                    self.hover_detector.start(&mut self.event_context, drawing_context);
                }

                Gesture::TapMove { position } => {
                    self.event_context.send_event_to_control(
                        self.event_context.get_captured_control(),
                        drawing_context,
                        ControlEvent::TapMove { position: position },
                    );
                }
            });
    }

    pub fn handle_keyboard_event(
        &mut self,
        _root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event: &InputEvent,
    ) {
        match event {
            InputEvent::KeyboardInput(key_event) => {
                self.event_context.send_event_to_control(
                    self.event_context.get_focused_control(),
                    drawing_context,
                    ControlEvent::KeyboardInput(key_event.clone()),
                );
            }

            _ => (),
        }
    }
}
