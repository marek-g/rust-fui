use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::control::HitTestResult;
use crate::control_object::*;
use crate::events::*;

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

    pub fn start(&mut self) {
        self.is_running = true;
        if let Some(ref hover_control) = self.get_hover_control() {
            hover_control
                .borrow_mut()
                .handle_event(ControlEvent::HoverEnter);
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        if let Some(ref hover_control) = self.get_hover_control() {
            hover_control
                .borrow_mut()
                .handle_event(ControlEvent::HoverLeave);
        }
    }

    pub fn handle_event(&mut self, root_view: &Rc<RefCell<dyn ControlObject>>, event: &InputEvent) {
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
                                hover_control
                                    .borrow_mut()
                                    .handle_event(ControlEvent::HoverLeave);
                            }
                            self.hover_control = Some(Rc::downgrade(hit_control));
                            if self.is_running {
                                hit_control
                                    .borrow_mut()
                                    .handle_event(ControlEvent::HoverEnter);
                            }
                        }
                    } else {
                        self.hover_control = Some(Rc::downgrade(hit_control));
                        if self.is_running {
                            hit_control
                                .borrow_mut()
                                .handle_event(ControlEvent::HoverEnter);
                        }
                    }
                } else {
                    if let Some(ref hover_control) = self.get_hover_control() {
                        if self.is_running {
                            hover_control
                                .borrow_mut()
                                .handle_event(ControlEvent::HoverLeave);
                        }
                        self.hover_control = None;
                    }
                }
            }

            InputEvent::CursorLeft { .. } => {
                if let Some(ref hover_control) = self.get_hover_control() {
                    if self.is_running {
                        hover_control
                            .borrow_mut()
                            .handle_event(ControlEvent::HoverLeave);
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
