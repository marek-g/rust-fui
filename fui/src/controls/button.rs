use control::*;
use common::rect::Rect;
use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use event::*;
use gestures::gesture_detector::{ GestureDetector, Gesture };

pub struct ButtonProperties {
    pub content: Box<ControlObject>,
}

pub struct ButtonEvents<'a> {
    pub clicked: Event<'a, ()>
}

pub struct Button<'a> {
    pub properties: ButtonProperties,
    pub events: ButtonEvents<'a>,
    style: Box<Style<ButtonProperties>>,

    rect: Rect,
    gesture_detector: GestureDetector,

    pub xxx: i32,
}

impl<'a> Button<'a> {
    pub fn new(content: Box<ControlObject>) -> Self {
        Button::<'a> {
            properties: ButtonProperties { content: content },
            events: ButtonEvents { clicked: Event::new() },
            style: Box::new(ButtonDefaultStyle { font_name: "OpenSans-Regular.ttf", font_size: 20u8 }),
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            gesture_detector: GestureDetector::new(),
            xxx: 10,
        }
    }
}

impl<'a> Control for Button<'a> {
    type Properties = ButtonProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_style(&self) -> &Box<Style<Self::Properties>> {
        &self.style
    }

    fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
        self.style.set_size(&mut self.properties, rect);
        self.gesture_detector.set_rect(rect);
    }

    fn get_size(&self) -> Rect {
        self.rect
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        self.gesture_detector.handle_event(&event).map(|gesture| match gesture {
            Gesture::HoverEnter => { self.xxx += 1; println!("on hover enter: {:?}", self.xxx); },
            Gesture::HoverLeave => { self.xxx += 1; println!("on hover leave: {:?}", self.xxx); },
            Gesture::TapUp { inside, tap_down_inside, .. } => {
                if inside && tap_down_inside {
                    self.events.clicked.emit(&());
                }
            }
            _ => ()
        });

        true
    }
}


//
// Button Default Style
//

pub struct ButtonDefaultStyle {
    font_name: &'static str,
    font_size: u8,
}

impl Style<ButtonProperties> for ButtonDefaultStyle {

    fn get_preferred_size(&self, properties: &ButtonProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let content_size = properties.content.get_preferred_size(drawing_context, size);
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_size(&mut self, properties: &mut ButtonProperties, rect: Rect) {
        let content_rect = Rect::new(rect.x + 10.0f32, rect.y + 10.0f32, rect.width - 20.0f32, rect.height - 20.0f32);
        properties.content.set_size(content_rect);
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

impl<'a> ControlObject for Button<'a> {

    fn set_size(&mut self, rect: Rect) {
        (self as &mut Control<Properties = ButtonProperties>).set_size(rect)
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        (self as &mut Control<Properties = ButtonProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(&self.get_properties(),
            drawing_context, self.get_size())
    }

}
