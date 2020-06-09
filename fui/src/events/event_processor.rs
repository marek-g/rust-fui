use crate::drawing::Resources;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::control::*;
use crate::{DrawingContext, events::*};

pub struct EventProcessor {
    hover_detector: HoverDetector,
    gesture_detector: GestureDetector,

    // captures mouse after TapDown
    captured_control: Option<Weak<RefCell<dyn ControlObject>>>,

    // control with focus
    focused_control: Option<Weak<RefCell<dyn ControlObject>>>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hover_detector: HoverDetector::new(),
            gesture_detector: GestureDetector::new(),
            captured_control: None,
            focused_control: None,
        }
    }

    pub fn handle_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event: &InputEvent,
    ) {
        self.hover_detector
            .handle_event(root_view, drawing_context, event);
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
                        self.set_new_focused_control(&hit_control, drawing_context);

                        self.captured_control = Some(Rc::downgrade(hit_control));
                        self.hover_detector.stop(drawing_context);

                        self.send_event_to_control(
                            &self.captured_control,
                            drawing_context,
                            ControlEvent::TapDown { position: position },
                        );
                    }
                }

                Gesture::TapUp { position } => {
                    self.send_event_to_control(
                        &self.captured_control,
                        drawing_context,
                        ControlEvent::TapUp { position: position },
                    );

                    self.captured_control = None;
                    self.hover_detector.start(drawing_context);
                }

                Gesture::TapMove { position } => {
                    self.send_event_to_control(
                        &self.captured_control,
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
                self.send_event_to_control(
                    &self.focused_control,
                    drawing_context,
                    ControlEvent::KeyboardInput(key_event.clone()),
                );
            }

            _ => (),
        }
    }

    fn set_new_focused_control(
        &mut self,
        control: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
    ) {
        self.send_event_to_control(&self.focused_control, drawing_context, ControlEvent::FocusLeave);
        self.focused_control = Some(Rc::downgrade(control));
        self.send_event_to_control(&self.focused_control, drawing_context, ControlEvent::FocusEnter);
    }

    fn send_event_to_control(
        &self,
        control: &Option<Weak<RefCell<dyn ControlObject>>>,
        drawing_context: &mut dyn DrawingContext,
        event: ControlEvent,
    ) {
        if let Some(ref control) = control {
            if let Some(ref control) = control.upgrade() {
                control.borrow_mut().handle_event(drawing_context, event);
            }
        };
    }
}
