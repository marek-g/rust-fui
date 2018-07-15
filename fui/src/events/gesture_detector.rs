use common::Point;

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

    pub fn handle_event(&mut self, event: &::winit::WindowEvent) -> Option<Gesture> {
        match event {
            ::winit::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Point::new(position.0 as f32, position.1 as f32);
                return Some(Gesture::TapMove {
                    position: self.mouse_pos,
                })
            },

            ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Pressed, .. } => {
                return Some(Gesture::TapDown {
                    position: self.mouse_pos,
                });
            },

            ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Released, .. } => {
                return Some(Gesture::TapUp {
                    position: self.mouse_pos,
                });
            },

            _ => None
        }
    }
}
