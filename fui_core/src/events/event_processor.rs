use std::cell::RefCell;
use std::{
    collections::VecDeque,
    rc::{Rc, Weak},
};

use crate::control::*;
use crate::{events::*, DrawingContext, Point};

struct QueuedEvent {
    pub control: Rc<RefCell<dyn ControlObject>>,
    pub event: ControlEvent,
}

pub struct EventProcessor {
    hovered_controls: Vec<Weak<RefCell<dyn ControlObject>>>,
    captured_control: Option<Weak<RefCell<dyn ControlObject>>>,
    focused_control: Option<Weak<RefCell<dyn ControlObject>>>,

    is_hover_enabled: bool,
    cursor_pos: Option<Point>,

    gesture_detector: GestureDetector,

    event_queue: VecDeque<QueuedEvent>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            hovered_controls: Vec::new(),
            captured_control: None,
            focused_control: None,

            is_hover_enabled: true,
            cursor_pos: None,

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

    fn handle_hover_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        event: &InputEvent,
    ) {
        match event {
            InputEvent::CursorMoved { position, .. } => {
                self.cursor_pos = Some(*position);
                self.recalculate_hover(root_view);
            }

            InputEvent::CursorLeft { .. } => {
                self.cursor_pos = None;
                self.clear_hover();
            }

            InputEvent::MouseInput {
                state: ElementState::Released,
                ..
            } => {
                self.recalculate_hover(root_view);
            }

            _ => (),
        }
    }

    fn recalculate_hover(&mut self, root_view: &Rc<RefCell<dyn ControlObject>>) {
        if let Some(position) = self.cursor_pos {
            let mut controls_to_hover = root_view.borrow().get_controls_at_point(position);

            // if there is captured control, only captured control can be hovered
            if let Some(captured_control) = &self.captured_control {
                let mut i = 0;
                while i != controls_to_hover.len() {
                    if !Weak::ptr_eq(&controls_to_hover[i], &captured_control) {
                        controls_to_hover.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }

            // leave hover
            let to_leave_hover = self
                .hovered_controls
                .iter()
                .rev()
                .filter(|&c| !controls_to_hover.iter().any(|c2| Weak::ptr_eq(c, c2)))
                .map(|c| c.clone())
                .collect::<Vec<_>>();
            for c in to_leave_hover {
                self.queue_event(c.upgrade(), ControlEvent::HoverLeave);
            }

            // enter hover
            for control_to_hover in &controls_to_hover {
                let exists = self
                    .hovered_controls
                    .iter()
                    .any(|c| Weak::ptr_eq(c, &control_to_hover));
                if !exists {
                    self.queue_event(control_to_hover.upgrade(), ControlEvent::HoverEnter);
                }
            }

            self.hovered_controls = controls_to_hover;
        }
    }

    fn clear_hover(&mut self) {
        let mut to_leave_hover = Vec::new();
        to_leave_hover.append(&mut self.hovered_controls);
        for c in to_leave_hover {
            self.queue_event(c.upgrade(), ControlEvent::HoverLeave);
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
