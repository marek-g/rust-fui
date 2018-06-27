use std::f32;
use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control::*;
use control_object::*;
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
}

impl Horizontal {
    pub fn new(children: Vec<Rc<RefCell<ControlObject>>>) -> Self {
        Horizontal {
            properties: HorizontalProperties { children: children },
        }
    }
}

impl ControlBehaviour for Control<Horizontal> {
    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        self.data.properties.children.clone()
    }

    fn handle_event(&mut self, _event: ControlEvent) -> bool {
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

impl HorizontalDefaultStyle {
    pub fn new() -> Self {
        HorizontalDefaultStyle {
            rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            desired_size: RefCell::new(Vec::new())
        }
    }
}

impl Style<Horizontal> for HorizontalDefaultStyle {
    fn get_preferred_size(&self, data: &Horizontal, drawing_context: &mut DrawingContext, size: Size) -> Size {
        let mut result = Size::new(0f32, 0f32);
        let available_size = Size::new(f32::INFINITY, size.height);

        let mut desired_size = self.desired_size.borrow_mut();

        desired_size.resize(data.properties.children.len(), Size::new(0f32, 0f32));
        for (i, child) in data.properties.children.iter().enumerate() {
            let child_size = child.borrow().get_preferred_size(drawing_context, available_size);
            desired_size[i] = child_size;
            result.width += child_size.width;
            result.height = result.height.max(child_size.height);
        }
        result
    }

    fn set_rect(&mut self, data: &Horizontal, rect: Rect) {
        self.rect = rect;

        let mut child_rect = rect;
        let desired_size = self.desired_size.borrow();

        for (i, child) in data.properties.children.iter().enumerate() {
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

    fn hit_test(&self, data: &Horizontal, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            for child in data.properties.children.iter() {
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

    fn to_primitives(&self, data: &Horizontal,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in &data.properties.children {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}
