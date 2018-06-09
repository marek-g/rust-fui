use common::Point;

pub enum Gesture {
    TapDown { position: Point },
    TapUp { position: Point, captured_position: Point }
}

pub struct GestureDetector {
    mouse_pos: Point,
    captured_mouse_pos: Option<Point>
}

impl GestureDetector {
    pub fn new() -> Self {
        GestureDetector {
            mouse_pos: Point::new(0f32, 0f32),
            captured_mouse_pos: None
        }
    }

    pub fn handle_event(&mut self, event: &::winit::Event) -> Option<Gesture> {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos = Point::new(position.0 as f32, position.1 as f32);
                },

                ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Pressed, .. } => {
                    self.captured_mouse_pos = Some(self.mouse_pos);
                    return Some(Gesture::TapDown {
                        position: self.mouse_pos,
                    });
                },

                ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Released, .. } => {
                    let captured_mouse_pos = self.captured_mouse_pos.unwrap_or(self.mouse_pos);
                    self.captured_mouse_pos = None;
                    return Some(Gesture::TapUp {
                        position: self.mouse_pos,
                        captured_position: captured_mouse_pos,
                    });
                },

                _ => ()
            }
        }
        None
    }
}
