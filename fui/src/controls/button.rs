use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::primitive_extensions::PrimitiveTransformations;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use observable::*;
use style::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ButtonProperties {
    #[builder(default_code = "Callback::empty()")]
    pub clicked: Callback<()>,
}

pub struct ButtonState {
    pub is_hover: Property<bool>,
    pub is_pressed: Property<bool>,
}

pub struct Button {
    pub properties: ButtonProperties,
    pub state: ButtonState,
}

impl Button {
    pub fn new(properties: ButtonProperties) -> Self {
        Button {
            properties: properties,
            state: ButtonState {
                is_hover: Property::new(false),
                is_pressed: Property::new(false),
            },
        }
    }
}

impl ControlBehaviour for Control<Button> {
    fn handle_event(&mut self, event: ControlEvent) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.data.state.is_pressed.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, &self.children, *position) {
                    self.data.properties.clicked.emit(());
                }
                self.data.state.is_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, &self.children, *position) {
                    self.data.state.is_pressed.set(true);
                } else {
                    self.data.state.is_pressed.set(false);
                }
            }

            ControlEvent::HoverEnter => {
                self.data.state.is_hover.set(true);
            }

            ControlEvent::HoverLeave => {
                self.data.state.is_hover.set(false);
            }

            _ => (),
        }
    }
}

//
// Button Default Style
//

pub struct ButtonDefaultStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
}

impl ButtonDefaultStyle {
    pub fn new() -> Self {
        ButtonDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Button> for ButtonDefaultStyle {
    fn setup_dirty_watching(&mut self, data: &mut Button, control: &Rc<RefCell<Control<Button>>>) {
        self.event_subscriptions
            .push(data.state.is_hover.dirty_watching(control));
        self.event_subscriptions
            .push(data.state.is_pressed.dirty_watching(control));
    }

    fn get_preferred_size(
        &self,
        data: &Button,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) -> Size {
        let content_size = if let Some(ref content) = children.first() {
            content.borrow().get_preferred_size(drawing_context, size)
        } else {
            Size::new(0f32, 0f32)
        };
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(&mut self, data: &Button, children: &Vec<Rc<RefCell<ControlObject>>>, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        if let Some(ref content) = children.first() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Button, children: &Vec<Rc<RefCell<ControlObject>>>, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(&self, data: &Button, children: &Vec<Rc<RefCell<ControlObject>>>, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let background = if data.state.is_pressed.get() {
            [0.1, 0.5, 0.0, 0.2]
        } else {
            if data.state.is_hover.get() {
                [0.1, 1.0, 0.0, 0.4]
            } else {
                [0.1, 1.0, 0.0, 0.2]
            }
        };
        let line_color1 = if !data.state.is_pressed.get() {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        };
        let line_color2 = if !data.state.is_pressed.get() {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        vec.push(Primitive::Rectangle {
            color: background,
            rect: UserPixelRect::new(
                UserPixelPoint::new(x + 1.0, y + 1.0),
                UserPixelSize::new(width - 2.0, height - 2.0),
            ),
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

        if let Some(ref content) = children.first() {
            let mut vec2 = content
                .borrow_mut()
                .to_primitives(drawing_context);
            if data.state.is_pressed.get() {
                vec2.translate(UserPixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
        }    

        vec
    }
}
