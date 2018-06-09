use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control::ControlObject;
use view::RootView;
use events::ControlEvent;

pub struct HoverDetector {
    hover_control: Option<Weak<RefCell<ControlObject>>>
}

impl HoverDetector {
    pub fn new() -> Self {
        HoverDetector {
            hover_control: None,
        }
    }

    pub fn get_hover_control(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref hover_control) = self.hover_control {
            hover_control.upgrade()
        } else {
            None
        }
    }

    pub fn handle_event(&mut self, root_view: &mut RootView, event: &::winit::Event) {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::CursorMoved { position, .. } => {
                    if let Some(ref hit_control) = root_view.hit_test(position.0 as f32, position.1 as f32) {
                        if let Some(ref hover_control) = self.get_hover_control() {
                            if !Rc::ptr_eq(hover_control, hit_control) {
                                hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                                self.hover_control = Some(Rc::downgrade(hit_control));
                                hit_control.borrow_mut().handle_event(ControlEvent::HoverEnter);
                            }
                        }
                        else {
                            self.hover_control = Some(Rc::downgrade(hit_control));
                            hit_control.borrow_mut().handle_event(ControlEvent::HoverEnter);
                        }
                    } else {
                        if let Some(ref hover_control) = self.get_hover_control() {
                            hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                            self.hover_control = None;
                        }
                    }
                },

                ::winit::WindowEvent::CursorLeft { .. } => {
                    if let Some(ref hover_control) = self.get_hover_control() {
                        hover_control.borrow_mut().handle_event(ControlEvent::HoverLeave);
                        self.hover_control = None;
                    }
                },

                _ => ()
            }
        }
    }
}
