use std::f32;
use std::cell::RefCell;
use std::rc::Rc;

use control::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use events::*;

pub struct HorizontalProperties {
    pub children: Vec<Rc<RefCell<ControlObject>>>
}

pub struct Horizontal {
    pub properties: HorizontalProperties,
    style: Box<Style<HorizontalProperties>>,
}

impl Horizontal {
    pub fn new(children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Horizontal {
            properties: HorizontalProperties { children: children },
            style: Box::new(HorizontalDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
                desired_size: RefCell::new(Vec::new())
            }),
        }))
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

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        self.properties.children.clone()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        true
    }
}


//
// Horizontal Default Style
//

pub struct HorizontalDefaultStyle {
    rect: Rect,
    desired_size: RefCell<Vec<Size>>
}

impl Style<HorizontalProperties> for HorizontalDefaultStyle {
    fn get_preferred_size(&self, properties: &HorizontalProperties, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let mut result = Size::new(0f32, 0f32);
        let available_size = Size::new(f32::INFINITY, size.height);

        let mut desired_size = self.desired_size.borrow_mut();

        desired_size.resize(properties.children.len(), Size::new(0f32, 0f32));
        for (i, child) in properties.children.iter().enumerate() {
            let child_size = child.borrow().get_preferred_size(drawing_context, available_size);
            desired_size[i] = child_size;
            result.width += child_size.width;
            result.height = result.height.max(child_size.height);
        }
        result
    }

    fn set_rect(&mut self, properties: &mut HorizontalProperties, rect: Rect) {
        self.rect = rect;

        let mut child_rect = rect;
        let desired_size = self.desired_size.borrow();

        for (i, child) in properties.children.iter_mut().enumerate() {
            let child_size = desired_size[i];
            child_rect.width = child_size.width;
            child_rect.height = child_size.height;
            child.borrow_mut().set_rect(child_rect);
            child_rect.x += child_rect.width;
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, properties: &HorizontalProperties, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            for child in properties.children.iter() {
                let c = child.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    match child_hit_test {
                        HitTestResult::Current => return HitTestResult::Child(child.clone()),
                        HitTestResult::Child(..) => return child_hit_test,
                        HitTestResult::Nothing => (),
                    }
                }
            }
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(&self, properties: &HorizontalProperties,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in &properties.children {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}


//
// object safe trait
//

impl ControlObject for Horizontal {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut Control<Properties = HorizontalProperties>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Properties = HorizontalProperties>).handle_event(event)
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
