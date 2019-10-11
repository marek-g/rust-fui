use drawing::backend::WindowTarget;
use common::Point;
use Window;

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

    pub fn handle_event(&mut self, window: &mut Window, event: &::winit::event::WindowEvent) -> Option<Gesture> {
        match event {
            ::winit::event::WindowEvent::CursorMoved { position, .. } => {
                let physical_pos = position.to_physical(window.get_drawing_target().get_window().hidpi_factor());
                self.mouse_pos = Point::new(physical_pos.x as f32, physical_pos.y as f32);
                return Some(Gesture::TapMove {
                    position: self.mouse_pos,
                })
            },

            ::winit::event::WindowEvent::MouseInput { button: ::winit::event::MouseButton::Left, state: ::winit::event::ElementState::Pressed, .. } => {
                return Some(Gesture::TapDown {
                    position: self.mouse_pos,
                });
            },

            ::winit::event::WindowEvent::MouseInput { button: ::winit::event::MouseButton::Left, state: ::winit::event::ElementState::Released, .. } => {
                return Some(Gesture::TapUp {
                    position: self.mouse_pos,
                });
            },

            _ => None
        }
    }
}
