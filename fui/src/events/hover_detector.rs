use view::RootView;

pub struct HoverDetector {
    mouse_pos: (f32, f32),
}

impl HoverDetector {
    pub fn new() -> Self {
        HoverDetector {
            mouse_pos: (0f32, 0f32),
        }
    }

    pub fn handle_event(&mut self, root_view: &mut RootView, event: &::winit::Event) {
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
                _ => ()
            }
        }
    }
}
