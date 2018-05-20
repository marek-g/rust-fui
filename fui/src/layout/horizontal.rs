use std::f32;
use std::cell::RefCell;
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
    pub fn new(children: Vec<Box<ControlObject>>) -> Box<Self> {
        Box::new(Horizontal {
            properties: HorizontalProperties { children: children },
            style: Box::new(HorizontalDefaultStyle { desired_size: RefCell::new(Vec::new()) }),
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
        })
    }
}

impl Control for Horizontal {
    type Properties = HorizontalProperties;

    fn get_properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn get_style(&self) -> &Box<Style<Self::Properties>> {
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

pub struct HorizontalDefaultStyle {
    desired_size: RefCell<Vec<Size>>
}

impl Style<HorizontalProperties> for HorizontalDefaultStyle {
    fn get_preferred_size(&self, properties: &HorizontalProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let mut result = Size::new(0f32, 0f32);
        let available_size = Size::new(f32::INFINITY, size.height);

        let mut desired_size = self.desired_size.borrow_mut();

        desired_size.resize(properties.children.len(), Size::new(0f32, 0f32));
        for (i, child) in properties.children.iter().enumerate() {
            let child_size = child.get_preferred_size(drawing_context, available_size);
            desired_size[i] = child_size;
            result.width += child_size.width;
            result.height = result.height.max(child_size.height);
        }
        result
    }

    fn set_size(&mut self, properties: &mut HorizontalProperties, rect: Rect) {
        let mut child_rect = rect;
        let desired_size = self.desired_size.borrow();

        for (i, child) in properties.children.iter_mut().enumerate() {
            let child_size = desired_size[i];
            child_rect.width = child_size.width;
            child_rect.height = child_size.height;
            child.set_size(child_rect);
            child_rect.x += child_rect.width;
        }
    }

    fn to_primitives<'a>(&self, properties: &'a HorizontalProperties,
        drawing_context: &mut DrawingContext, rect: Rect) -> Vec<Primitive<'a>> {
        let mut vec = Vec::new();

        for child in &properties.children {
            vec.append(&mut child.to_primitives(drawing_context));
        }

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
        self.get_style().get_preferred_size(self.get_properties(), drawing_context, size)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(self.get_properties(),
            drawing_context, self.get_size())
    }
}
