use control::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use callback::*;
use events::*;

pub struct ButtonProperties {
    pub content: Box<ControlObject>,
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
    pub fn new(content: Box<ControlObject>) -> Box<Self> {
        Box::new(Button {
            properties: ButtonProperties { content: content },
            events: ButtonEvents { clicked: Callback::new() },
            style: Box::new(ButtonDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            }),
        })
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

    fn get_children(&mut self) -> Vec<&mut Box<ControlObject>> {
        Vec::new()
    }

    fn is_hit_test_visible(&self) -> bool {
        true
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        match event {
            ControlEvent::TapUp{ ref position } => {
                let rect = self.style.get_rect();
                if position.0 >= rect.x && position.0 <= rect.x + rect.width &&
                    position.1 >= rect.y && position.1 <= rect.y + rect.height {
                    self.events.clicked.emit(&());
                }
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

impl Style<ButtonProperties> for ButtonDefaultStyle {

    fn get_preferred_size(&self, properties: &ButtonProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let content_size = properties.content.get_preferred_size(drawing_context, size);
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(&mut self, properties: &mut ButtonProperties, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(rect.x + 10.0f32, rect.y + 10.0f32, rect.width - 20.0f32, rect.height - 20.0f32);
        properties.content.set_rect(content_rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn to_primitives<'a>(&self, properties: &'a ButtonProperties,
        drawing_context: &mut DrawingContext, rect: Rect) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        vec.push(Primitive::Rectangle {
            color: [0.1, 1.0, 0.0, 0.2],
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

        vec.append(&mut properties.content.to_primitives(drawing_context));

        vec
    }

}


//
// object safe trait
//

impl ControlObject for Button {
    fn set_rect(&mut self, rect: Rect) {
        let style = &mut self.style;
        let properties = &mut self.properties;
        style.set_rect(properties, rect);
    }

    fn get_rect(&self) -> Rect {
        self.get_style().get_rect()
    }

    fn is_hit_test_visible(&self) -> bool {
        (self as &Control<Properties = ButtonProperties>).is_hit_test_visible()
    }

    fn get_children(&mut self) -> Vec<&mut Box<ControlObject>> {
        (self as &mut Control<Properties = ButtonProperties>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Properties = ButtonProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(&self.get_properties(),
            drawing_context, self.get_style().get_rect())
    }

}
