use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use style::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct HorizontalProperties {
}

pub struct Horizontal {
    pub properties: HorizontalProperties,
}

impl Horizontal {
    pub fn new(properties: HorizontalProperties) -> Self {
        Horizontal {
            properties: properties,
        }
    }
}

impl ControlBehaviour for Control<Horizontal> {
    fn handle_event(&mut self, _event: ControlEvent) {}
}

//
// Horizontal Default Style
//

pub struct HorizontalDefaultStyle {
    rect: Rect,
    desired_size: RefCell<Vec<Size>>,
}

impl HorizontalDefaultStyle {
    pub fn new() -> Self {
        HorizontalDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            desired_size: RefCell::new(Vec::new()),
        }
    }
}

impl Style<Horizontal> for HorizontalDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut Horizontal,
        _control: &Rc<RefCell<Control<Horizontal>>>,
    ) {
    }

    fn get_preferred_size(
        &self,
        data: &Horizontal,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) -> Size {
        let mut result = Size::new(0f32, 0f32);
        let available_size = Size::new(f32::INFINITY, size.height);

        let mut desired_size = self.desired_size.borrow_mut();

        desired_size.resize(children.len(), Size::new(0f32, 0f32));
        for (i, child) in children.iter().enumerate() {
            let child_size = child
                .borrow()
                .get_preferred_size(drawing_context, available_size);
            desired_size[i] = child_size;
            result.width += child_size.width;
            result.height = result.height.max(child_size.height);
        }
        result
    }

    fn set_rect(&mut self, data: &Horizontal, children: &Vec<Rc<RefCell<ControlObject>>>, rect: Rect) {
        self.rect = rect;

        let mut child_rect = rect;
        let desired_size = self.desired_size.borrow();

        for (i, child) in children.iter().enumerate() {
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

    fn hit_test(&self, data: &Horizontal, children: &Vec<Rc<RefCell<ControlObject>>>, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) {
            for child in children.iter() {
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

    fn to_primitives(
        &self,
        data: &Horizontal,
        children: &Vec<Rc<RefCell<ControlObject>>>,
        drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in children {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}
