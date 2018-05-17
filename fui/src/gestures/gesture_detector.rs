use common::rect::Rect;

pub enum Gesture {
    HoverEnter,
    HoverLeave,
    TapDown { position: (f32, f32), inside: bool },
    TapUp { position: (f32, f32), inside: bool, tap_down_inside: bool }
}

pub struct GestureDetector {
    rect: Rect,

    mouse_pos_x: f32,
    mouse_pos_y: f32,

    is_hover: bool,
    is_tap_down_inside: bool
}

impl GestureDetector {
    pub fn new() -> Self {
        GestureDetector {
            rect: Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32),
            mouse_pos_x: 0f32, mouse_pos_y: 0f32,
            is_hover: false,
            is_tap_down_inside: false
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn handle_event(&mut self, event: &::winit::Event) -> Option<Gesture> {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos_x = position.0 as f32;
                    self.mouse_pos_y = position.1 as f32;

                    if self.is_pointer_inside() {
                        if !self.is_hover {
                            self.is_hover = true;

                            return Some(Gesture::HoverEnter);
                        }
                    } else {
                        if self.is_hover {
                            self.is_hover = false;

                            return Some(Gesture::HoverLeave);
                        }
                    }
                },
                ::winit::WindowEvent::CursorLeft { .. } => {
                    if self.is_hover {
                        self.is_hover = false;

                        return Some(Gesture::HoverLeave);
                    }
                },
                ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Pressed, .. } => {
                    self.is_tap_down_inside = self.is_pointer_inside();
                    return Some(Gesture::TapDown {
                        position: (self.mouse_pos_x, self.mouse_pos_y),
                        inside: self.is_tap_down_inside
                    });
                },
                ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Released, .. } => {
                    return Some(Gesture::TapUp {
                        position: (self.mouse_pos_x, self.mouse_pos_y),
                        inside: self.is_pointer_inside(),
                        tap_down_inside: self.is_tap_down_inside
                    });
                },
                _ => ()
            }
        }
        None
    }

    fn is_pointer_inside(&self) -> bool {
        self.mouse_pos_x >= self.rect.x && self.mouse_pos_x < self.rect.x + self.rect.width &&
            self.mouse_pos_y >= self.rect.y && self.mouse_pos_y < self.rect.y + self.rect.height
    }
}
