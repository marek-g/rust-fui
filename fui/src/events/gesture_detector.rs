pub enum Gesture {
    TapDown { position: (f32, f32) },
    TapUp { position: (f32, f32), captured_position: (f32, f32) }
}

pub struct GestureDetector {
    mouse_pos: (f32, f32),
    captured_mouse_pos: Option<(f32, f32)>
}

impl GestureDetector {
    pub fn new() -> Self {
        GestureDetector {
            mouse_pos: (0f32, 0f32),
            captured_mouse_pos: None
        }
    }

    pub fn handle_event(&mut self, event: &::winit::Event) -> Option<Gesture> {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos = (position.0 as f32, position.1 as f32);

                    /*if self.is_pointer_inside() {
                        if !self.is_hover {
                            self.is_hover = true;

                            return Some(Gesture::HoverEnter);
                        }
                    } else {
                        if self.is_hover {
                            self.is_hover = false;

                            return Some(Gesture::HoverLeave);
                        }
                    }*/
                },
                /*::winit::WindowEvent::CursorLeft { .. } => {
                    if self.is_hover {
                        self.is_hover = false;

                        return Some(Gesture::HoverLeave);
                    }
                },*/
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
