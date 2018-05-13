use common::rect::Rect;
use common::size::Size;
use drawing_context::DrawingContext;
use controls::control::*;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use event::*;
use gestures::gesture_detector::GestureDetector;

pub struct ButtonProperties {
    pub text: String,
}

pub struct ButtonEvents {
    pub clicked: Event<()>
}

pub struct Button<'a> {
    pub properties: ButtonProperties,
    pub events: ButtonEvents,
    style: Box<Style<ButtonProperties>>,

    rect: Rect,
    gesture_detector: GestureDetector<'a>
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        let mut gesture_detector = GestureDetector::new();

        gesture_detector.on_hover_enter.set(|_|{ println!("on hover enter"); });
        gesture_detector.on_hover_leave.set(|_|{ println!("on hover leave"); });

        Button {
            properties: ButtonProperties { text: "Hello World!".to_string() },
            events: ButtonEvents { clicked: Event::new() },
            style: Box::new(ButtonDefaultStyle { font_name: "OpenSans-Regular.ttf", font_size: 20u8 }),
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            gesture_detector: gesture_detector
        }
    }
}

impl<'a> Control for Button<'a> {
    type Properties = ButtonProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_syle(&self) -> &Box<Style<Self::Properties>> {
        &self.style
    }

    fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
        self.gesture_detector.set_rect(rect);
    }

    fn get_size(&self) -> Rect {
        self.rect
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        self.gesture_detector.handle_event(&event);

        if let ::winit::Event::WindowEvent { ref event, .. } = event {
            match event {
                ::winit::WindowEvent::MouseInput { button: ::winit::MouseButton::Left, state: ::winit::ElementState::Released, .. } => {
                    self.events.clicked.emit(&());
                },
                _ => ()
            }
        }

        //println!("event: {:?}", event);
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

    fn get_preferred_size(&self, properties: &ButtonProperties, _size: Size, drawing_context: &mut DrawingContext) -> Size {
        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);
        Size::new((text_width as f32) * 1.2, (text_height as f32) * 1.2)
    }

    fn to_primitives<'a>(&self, properties: &'a ButtonProperties,
        rect: Rect,
        drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let (text_width, text_height) = drawing_context.get_font_dmensions(self.font_name, self.font_size, &properties.text);

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

        vec.push(Primitive::Text {
            resource_key: self.font_name,
            color: [1.0, 1.0, 1.0, 1.0],
            position: UserPixelPoint::new(x + (width - text_width as f32) / 2.0, y + (height - text_height as f32) / 2.0),
            size: self.font_size as u16,
            text: &properties.text,
        });

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

    fn get_preferred_size(&self, size: Size, drawing_context: &mut DrawingContext) -> Size {
        self.get_syle().get_preferred_size(self.get_properties(), size, drawing_context)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_syle().to_primitives(&self.get_properties(),
            self.get_size(),
            drawing_context)
    }

}
