use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::control::*;
use crate::{DrawingContext, events::*};

pub struct HoverDetector {
    hover_control: Option<Weak<RefCell<dyn ControlObject>>>,
    is_running: bool,
}

impl HoverDetector {
    pub fn new() -> Self {
        HoverDetector {
            hover_control: None,
            is_running: true,
        }
    }

    pub fn start(&mut self, event_context: &mut EventContext, drawing_context: &mut dyn DrawingContext) {
        self.is_running = true;
        event_context.send_event_to_control(self.get_hover_control(), drawing_context, ControlEvent::HoverEnter);
    }

    pub fn stop(&mut self, event_context: &mut EventContext, drawing_context: &mut dyn DrawingContext) {
        self.is_running = false;
        event_context.send_event_to_control(self.get_hover_control(), drawing_context, ControlEvent::HoverLeave);
    }

    pub fn handle_event(
        &mut self,
        root_view: &Rc<RefCell<dyn ControlObject>>,
        drawing_context: &mut dyn DrawingContext,
        event_context: &mut EventContext,
        event: &InputEvent,
    ) {
        match event {
            InputEvent::CursorMoved { position, .. } => {
                //let physical_pos =
                //    position.to_physical(window.get_drawing_target().get_window().hidpi_factor());
                let hit_test_result = root_view.borrow().hit_test(*position);
                let hit_control = match hit_test_result {
                    HitTestResult::Current => Some(root_view.clone()),
                    HitTestResult::Child(control) => Some(control),
                    HitTestResult::Nothing => None,
                };

                if let Some(ref hit_control) = hit_control {
                    if let Some(ref hover_control) = self.get_hover_control() {
                        if !Rc::ptr_eq(hover_control, hit_control) {
                            if self.is_running {
                                event_context.send_event_to_control(Some(hover_control.clone()), drawing_context, ControlEvent::HoverLeave);
                            }
                        }
                    }
                    self.hover_control = Some(Rc::downgrade(hit_control));
                    if self.is_running {
                        event_context.send_event_to_control(Some(hit_control.clone()), drawing_context, ControlEvent::HoverEnter);
                    }
                } else {
                    if let Some(ref hover_control) = self.get_hover_control() {
                        if self.is_running {
                            event_context.send_event_to_control(Some(hover_control.clone()), drawing_context, ControlEvent::HoverLeave);
                        }
                        self.hover_control = None;
                    }
                }
            }

            InputEvent::CursorLeft { .. } => {
                if let Some(ref hover_control) = self.get_hover_control() {
                    if self.is_running {
                        event_context.send_event_to_control(Some(hover_control.clone()), drawing_context, ControlEvent::HoverLeave);
                    }
                    self.hover_control = None;
                }
            }

            _ => (),
        }
    }

    fn get_hover_control(&self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if let Some(ref hover_control) = self.hover_control {
            hover_control.upgrade()
        } else {
            None
        }
    }
}
