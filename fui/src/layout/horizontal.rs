use control::*;
use common::rect::Rect;
use common::size::Size;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };

pub struct HorizontalProperties {
    pub children: Vec<Box<ControlObject>>
}

pub struct Horizontal {
    pub properties: HorizontalProperties,
    style: Box<Style<HorizontalProperties>>,

    rect: Rect,
}

impl Horizontal {
    pub fn new(children: Vec<Box<ControlObject>>) -> Self {
        Horizontal {
            properties: HorizontalProperties { children: children },
            style: Box::new(HorizontalDefaultStyle {}),
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
        }
    }
}

impl Control for Horizontal {
    type Properties = HorizontalProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_syle(&self) -> &Box<Style<Self::Properties>> {
        &self.style
    }

    fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
        self.style.set_size(&mut self.properties, rect);
    }

    fn get_size(&self) -> Rect {
        self.rect
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        true
    }
}


//
// Horizontal Default Style
//

pub struct HorizontalDefaultStyle;

impl Style<HorizontalProperties> for HorizontalDefaultStyle {
    fn get_preferred_size(&self, properties: &HorizontalProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        size
    }

    fn set_size(&mut self, properties: &mut HorizontalProperties, rect: Rect) {    
    }

    fn to_primitives<'a>(&self, properties: &'a HorizontalProperties,
        rect: Rect,
        drawing_context: &mut DrawingContext) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        vec
    }
}


//
// object safe trait
//

impl ControlObject for Horizontal {
    fn set_size(&mut self, rect: Rect) {
        (self as &mut Control<Properties = HorizontalProperties>).set_size(rect)
    }

    fn handle_event(&mut self, event: &::winit::Event) -> bool {
        (self as &mut Control<Properties = HorizontalProperties>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_syle().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_syle().to_primitives(&self.get_properties(),
            self.get_size(),
            drawing_context)
    }
}
