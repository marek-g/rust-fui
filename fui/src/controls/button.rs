use std::cell::RefCell;
use std::rc::Rc;

use control::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use callback::*;
use events::*;

pub struct ButtonProperties {
    pub content: Rc<RefCell<ControlObject>>,
    pub is_hover: bool
}

pub struct ButtonEvents {
    pub clicked: Callback<()>
}

pub struct Button {
    pub properties: ButtonProperties,
    pub events: ButtonEvents,
    style: Box<Style<ButtonProperties>>,
}

impl Button {
    pub fn new(content: Rc<RefCell<ControlObject>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Button {
            properties: ButtonProperties { content: content, is_hover: false },
            events: ButtonEvents { clicked: Callback::new() },
            style: Box::new(ButtonDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            }),
        }))
    }
}

impl Control for Button {
    type Properties = ButtonProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_style(&self) -> &Box<Style<Self::Properties>> {
        &self.style
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        match event {
            ControlEvent::TapUp{ ref position } => {
                let rect = self.style.get_rect();
                if position.is_inside(&rect) {
                    self.events.clicked.emit(&());
                }
                true
            },

            ControlEvent::HoverEnter => {
                self.properties.is_hover = true; true
            },

            ControlEvent::HoverLeave => {
                self.properties.is_hover = false; true
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

impl Style<ButtonProperties> for ButtonDefaultStyle {

    fn get_preferred_size(&self, properties: &ButtonProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let content_size = properties.content.borrow().get_preferred_size(drawing_context, size);
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(&mut self, properties: &mut ButtonProperties, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(rect.x + 10.0f32, rect.y + 10.0f32, rect.width - 20.0f32, rect.height - 20.0f32);
        properties.content.borrow_mut().set_rect(content_rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, properties: &ButtonProperties, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, properties: &ButtonProperties,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let background = if properties.is_hover { [0.1, 1.0, 0.0, 0.4] } else { [0.1, 1.0, 0.0, 0.2] };

        vec.push(Primitive::Rectangle {
            color: background,
            rect: UserPixelRect::new(UserPixelPoint::new(x + 1.0, y + 1.0),
                UserPixelSize::new(width - 2.0, height - 2.0))
        });

        vec.push(Primitive::Line {
            color: [1.0, 1.0, 1.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: [1.0, 1.0, 1.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: [0.0, 0.0, 0.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        });
        vec.push(Primitive::Line {
            color: [0.0, 0.0, 0.0, 1.0],
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        });

        let mut vec2 = properties.content.borrow_mut().to_primitives(drawing_context);
        vec.append(&mut vec2);

        vec
    }

}


//
// object safe trait
//

impl ControlObject for Button {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut Control<Properties = ButtonProperties>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Properties = ButtonProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        let style = &mut self.style;
        let properties = &mut self.properties;
        style.set_rect(properties, rect);
    }

    fn get_rect(&self) -> Rect {
        self.get_style().get_rect()
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.get_style().hit_test(self.get_properties(), point)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(self.get_properties(),
            drawing_context)
    }

}
