use std::cell::RefCell;
use std::{
    collections::VecDeque,
    rc::{Rc, Weak},
};

use crate::control::*;
use crate::{events::*, DrawingContext};

struct QueuedEvent {
    pub control: Rc<RefCell<dyn ControlObject>>,
    pub event: ControlEvent,
}

pub struct EventProcessor {
    hovered_control: Option<Weak<RefCell<dyn ControlObject>>>,
    captured_control: Option<Weak<RefCell<dyn ControlObject>>>,
    focused_control: Option<Weak<RefCell<dyn ControlObject>>>,

    is_hover_enabled: bool,

    gesture_detector: GestureDetector,

    event_queue: VecDeque<QueuedEvent>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hovered_control: None,
            captured_control: None,
            focused_control: None,

            is_hover_enabled: true,

            gesture_detector: GestureDetector::new(),

            event_queue: VecDeque::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event: &InputEvent,
    ) {
        self.handle_keyboard_event(root_view, event);
        self.handle_gesture_event(root_view, event);
        self.handle_hover_event(root_view, event);

        while let Some(queue_event) = self.event_queue.pop_front() {
            self.send_event_to_control(
                Some(queue_event.control),
                drawing_context,
                queue_event.event,
            );
        }
    }

    fn handle_keyboard_event(
        &mut self,
        _root_view: &Rc<RefCell<dyn ControlObject>>,
        event: &InputEvent,
    ) {
        match event {
            InputEvent::KeyboardInput(key_event) => {
                self.queue_event(
                    self.get_focused_control(),
                    ControlEvent::KeyboardInput(key_event.clone()),
                );
            }

            _ => (),
        }
    }

    fn handle_gesture_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        event: &InputEvent,
    ) {
        self.gesture_detector
            .handle_event(root_view, event)
            .map(|ev| match ev {
                Gesture::TapDown { position } => {
                    let captured_control = self.get_captured_control();
                    if let Some(captured_control) = captured_control {
                        self.queue_event(
                            Some(captured_control),
                            ControlEvent::TapDown { position: position },
                        );
                    } else {
                        let hit_test_result = root_view.borrow().hit_test(position);
                        let hit_control = match hit_test_result {
                            HitTestResult::Current => Some(root_view.clone()),
                            HitTestResult::Child(control) => Some(control),
                            HitTestResult::Nothing => None,
                        };

                        if let Some(ref hit_control) = hit_control {
                            self.set_focused_control(Some(hit_control.clone()));

                            self.set_captured_control(Some(hit_control.clone()));

                            self.queue_event(
                                self.get_captured_control(),
                                ControlEvent::TapDown { position: position },
                            );
                        }
                    }
                }

                Gesture::TapUp { position } => {
                    let captured_control = self.get_captured_control();
                    self.set_captured_control(None);
                    self.queue_event(captured_control, ControlEvent::TapUp { position: position });
                }

                Gesture::TapMove { position } => {
                    self.queue_event(
                        self.get_captured_control(),
                        ControlEvent::TapMove { position: position },
                    );
                }
            });
    }

    fn disable_hover(&mut self) {
        self.queue_event(self.get_hovered_control(), ControlEvent::HoverLeave);
        self.is_hover_enabled = false;
    }

    fn enable_hover(&mut self) {
        self.is_hover_enabled = true;
        self.queue_event(self.get_hovered_control(), ControlEvent::HoverEnter);
    }

    fn handle_hover_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        event: &InputEvent,
    ) {
        match event {
            InputEvent::CursorMoved { position, .. } => {
                let hit_test_result = root_view.borrow().hit_test(*position);
                let hit_control = match hit_test_result {
                    HitTestResult::Current => Some(root_view.clone()),
                    HitTestResult::Child(control) => Some(control),
                    HitTestResult::Nothing => None,
                };

                self.set_hovered_control(hit_control);
            }

            InputEvent::CursorLeft { .. } => {
                self.set_hovered_control(None);
            }

            _ => (),
        }
    }

    /// Sends event to the control.
    ///
    /// As it borrows mutably the control object,
    /// it can only be safely called from within handle_event().
    ///
    /// Please use the queue_event() in all the other places.
    fn send_event_to_control(
        &mut self,
        control: Option<Rc<RefCell<dyn ControlObject>>>,
        drawing_context: &mut dyn DrawingContext,
        event: ControlEvent,
    ) {
        if let Some(ref control) = control {
            control
                .borrow_mut()
                .handle_event(drawing_context, self, event);
        };
    }
}

impl EventContext for EventProcessor {
    fn get_hovered_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.is_hover_enabled {
            if let Some(ref control) = self.hovered_control {
                control.upgrade()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_hovered_control(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>) {
        if !self.is_hover_enabled {
            self.hovered_control = control.map(|ref c| Rc::downgrade(c));
            return;
        }

        if let Some(control) = control {
            if let Some(ref hovered_control) = self.get_hovered_control() {
                if !Rc::ptr_eq(hovered_control, &control) {
                    self.queue_event(Some(hovered_control.clone()), ControlEvent::HoverLeave);
                    self.hovered_control = Some(Rc::downgrade(&control));
                    self.queue_event(Some(control), ControlEvent::HoverEnter);
                }
            } else {
                self.hovered_control = Some(Rc::downgrade(&control));
                self.queue_event(Some(control), ControlEvent::HoverEnter);
            }
        } else {
            if let Some(ref hovered_control) = self.get_hovered_control() {
                self.queue_event(Some(hovered_control.clone()), ControlEvent::HoverLeave);
                self.hovered_control = None;
            }
        }
    }

    fn get_captured_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref control) = self.captured_control {
            control.upgrade()
        } else {
            None
        }
    }

    fn set_captured_control(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>) {
        self.captured_control = control.map(|ref c| Rc::downgrade(c));
    }

    fn get_focused_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref control) = self.focused_control {
            control.upgrade()
        } else {
            None
        }
    }

    fn set_focused_control(&mut self, control: Option<Rc<RefCell<dyn ControlObject>>>) {
        self.queue_event(self.get_focused_control(), ControlEvent::FocusLeave);
        self.focused_control = control.clone().map(|ref c| Rc::downgrade(c));
        self.queue_event(control, ControlEvent::FocusEnter);
    }

    fn queue_event(
        &mut self,
        control: Option<Rc<RefCell<dyn ControlObject>>>,
        event: ControlEvent,
    ) {
        if let Some(control) = control {
            self.event_queue.push_back(QueuedEvent { control, event })
        }
    }
}
