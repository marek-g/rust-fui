use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use drawing::backend::WindowTarget;
use common::Point;
use control::HitTestResult;
use control_object::*;
use events::ControlEvent;
use Window;

pub struct HoverDetector {
    hover_control: Option<Weak<RefCell<ControlObject>>>,
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
            hover_control.borrow_mut().handle_event(ControlEvent::HoverEnter);
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        if let Some(ref hover_control) = self.get_hover_control() {
            hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
        }
    }

    pub fn handle_event(&mut self, window: &mut Window, event: &::winit::event::WindowEvent) {
        match event {
            ::winit::event::WindowEvent::CursorMoved { position, .. } => {
                let physical_pos = position.to_physical(window.get_drawing_target().get_window().hidpi_factor());
                if let Some(ref mut root_view) = window.get_root_view_mut() {
                    let hit_test_result = root_view.borrow().hit_test(Point::new(physical_pos.x as f32, physical_pos.y as f32));
                    let hit_control = match hit_test_result {
                        HitTestResult::Current => Some(root_view.clone()),
                        HitTestResult::Child(control) => Some(control),
                        HitTestResult::Nothing => None,
                    };

                    if let Some(ref hit_control) = hit_control {
                        if let Some(ref hover_control) = self.get_hover_control() {
                            if !Rc::ptr_eq(hover_control, hit_control) {
                                if self.is_running {
                                    hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                                }
                                self.hover_control = Some(Rc::downgrade(hit_control));
                                if self.is_running {
                                    hit_control.borrow_mut().handle_event(ControlEvent::HoverEnter);
                                }
                            }
                        }
                        else {
                            self.hover_control = Some(Rc::downgrade(hit_control));
                            if self.is_running {
                                hit_control.borrow_mut().handle_event(ControlEvent::HoverEnter);
                            }
                        }
                    } else {
                        if let Some(ref hover_control) = self.get_hover_control() {
                            if self.is_running {
                                hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                            }
                            self.hover_control = None;
                        }
                    }
                }
            },

            ::winit::event::WindowEvent::CursorLeft { .. } => {
                if let Some(ref hover_control) = self.get_hover_control() {
                    if self.is_running {
                        hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                    }
                    self.hover_control = None;
                }
            },

            _ => ()
        }
    }

    fn get_hover_control(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref hover_control) = self.hover_control {
            hover_control.upgrade()
        } else {
            None
        }
    }
}
