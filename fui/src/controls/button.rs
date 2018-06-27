use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control::*;
use control_object::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::primitive_extensions::PrimitiveTransformations;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use observable::*;
use events::*;

pub struct ButtonProperties {
    pub content: Rc<RefCell<ControlObject>>,
}

pub struct ButtonEvents {
    pub clicked: Callback<()>,
}

pub struct ButtonState {
    pub is_hover: bool,
    pub is_pressed: bool,
}

pub struct Button {
    pub properties: ButtonProperties,
    pub events: ButtonEvents,
    pub state: ButtonState,
}

impl Button {
    pub fn new(content: Rc<RefCell<ControlObject>>) -> Self {
        Button {
            properties: ButtonProperties { content: content },
            events: ButtonEvents { clicked: Callback::new() },
            state: ButtonState { is_hover: false, is_pressed: false },
        }
    }
}

impl ControlBehaviour for Control<Button> {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        match event {
            ControlEvent::TapDown{ .. } => {
                self.data.state.is_pressed = true;
                self.set_is_dirty(true);
                true
            },

            ControlEvent::TapUp{ ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, *position) {
                    self.data.events.clicked.emit(());
                }
                self.data.state.is_pressed = false;
                self.set_is_dirty(true);
                true
            },

            ControlEvent::TapMove{ ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, *position) {
                    if !self.data.state.is_pressed {
                        self.data.state.is_pressed = true;
                        self.set_is_dirty(true);
                    }
                } else {
                    if self.data.state.is_pressed {
                        self.data.state.is_pressed = false;
                        self.set_is_dirty(true);
                    }
                }
                true
            },

            ControlEvent::HoverEnter => {
                self.data.state.is_hover = true;
                self.set_is_dirty(true);
                true
            },

            ControlEvent::HoverLeave => {
                self.data.state.is_hover = false;
                self.set_is_dirty(true);
                true
            },

            _ => false
        }
    }
}


//
// Button Default Style
//

pub struct ButtonDefaultStyle {
    rect: Rect,
}

impl ButtonDefaultStyle {
    pub fn new() -> Self {
        ButtonDefaultStyle {
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
        }
    }
}

impl Style<Button> for ButtonDefaultStyle {
    fn get_preferred_size(&self, data: &Button,
        drawing_context: &mut DrawingContext, size: Size) -> Size {
        let content_size = data.properties.content.borrow().get_preferred_size(drawing_context, size);
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(&mut self, data: &Button, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(rect.x + 10.0f32, rect.y + 10.0f32, rect.width - 20.0f32, rect.height - 20.0f32);
        data.properties.content.borrow_mut().set_rect(content_rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Button, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, data: &Button,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let background = if data.state.is_pressed { [0.1, 0.5, 0.0, 0.2] }
            else { if data.state.is_hover { [0.1, 1.0, 0.0, 0.4] } else { [0.1, 1.0, 0.0, 0.2] } };
        let line_color1 = if !data.state.is_pressed { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] };
        let line_color2 = if !data.state.is_pressed { [0.0, 0.0, 0.0, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };

        vec.push(Primitive::Rectangle {
            color: background,
            rect: UserPixelRect::new(UserPixelPoint::new(x + 1.0, y + 1.0),
                UserPixelSize::new(width - 2.0, height - 2.0))
        });

        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        });

        let mut vec2 = data.properties.content.borrow_mut().to_primitives(drawing_context);
        if data.state.is_pressed { vec2.translate(UserPixelPoint::new(1.0f32, 1.0f32)); }
        vec.append(&mut vec2);

        vec
    }
}
