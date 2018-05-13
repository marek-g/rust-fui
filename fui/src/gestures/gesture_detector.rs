use common::rect::Rect;
use callback::Callback;

pub struct GestureDetector<'a> {
    pub on_hover_enter: Callback<'a, ()>,
    pub on_hover_leave: Callback<'a, ()>,
    pub on_tap_down: Callback<'a, (f32, f32)>,
    pub on_tap_up: Callback<'a, (f32, f32)>,

    rect: Rect,

    mouse_pos_x: f32,
    mouse_pos_y: f32,

    is_hover: bool
}

impl<'a> GestureDetector<'a> {
    pub fn new() -> Self {
        GestureDetector {
            on_hover_enter: Callback::new(),
            on_hover_leave: Callback::new(),
            on_tap_down: Callback::new(),
            on_tap_up: Callback::new(),
            rect: Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32),
            mouse_pos_x: 0f32, mouse_pos_y: 0f32,
            is_hover: false
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn handle_event(&mut self, event: &::winit::Event) {
        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos_x = position.0 as f32;
                    self.mouse_pos_y = position.1 as f32;

                    if self.mouse_pos_x >= self.rect.x && self.mouse_pos_x < self.rect.x + self.rect.width &&
                        self.mouse_pos_y >= self.rect.y && self.mouse_pos_y < self.rect.y + self.rect.height {
                        if !self.is_hover {
                            self.is_hover = true;
                            self.on_hover_enter.emit(());
                        }
                    } else {
                        if self.is_hover {
                            self.is_hover = false;
                            self.on_hover_leave.emit(());
                        }
                    }

                    //println!("mouse: {:?}", position);
                },
                ::winit::WindowEvent::CursorLeft { .. } => {
                    if self.is_hover {
                        self.is_hover = false;
                        self.on_hover_leave.emit(());
                    }
                }
                _ => ()
            }
        }
    }
}