use std::cell::RefCell;
use std::rc::Rc;

use crate::common::Point;
use crate::control::ControlObject;
use crate::events::*;

pub enum Gesture {
    TapDown { position: Point },
    TapUp { position: Point },
    TapMove { position: Point },
}

pub struct GestureDetector {
    mouse_pos: Point,
}

impl GestureDetector {
    pub fn new() -> Self {
        GestureDetector {
            mouse_pos: Point::new(0f32, 0f32),
        }
    }

    pub fn handle_event(
        &mut self,
        _root_view: &Rc<RefCell<dyn ControlObject>>,
        event: &InputEvent,
    ) -> Option<Gesture> {
        match event {
            InputEvent::CursorMoved { position, .. } => {
                //let physical_pos =
                //    position.to_physical(window.get_drawing_target().get_window().hidpi_factor());
                //self.mouse_pos = Point::new(physical_pos.x as f32, physical_pos.y as f32);
                self.mouse_pos = *position;
                return Some(Gesture::TapMove {
                    position: self.mouse_pos,
                });
            }

            InputEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                ..
            } => {
                return Some(Gesture::TapDown {
                    position: self.mouse_pos,
                });
            }

            InputEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Released,
                ..
            } => {
                return Some(Gesture::TapUp {
                    position: self.mouse_pos,
                });
            }

            _ => None,
        }
    }
}
