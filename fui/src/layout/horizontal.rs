use std::f32;
use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use events::*;

pub struct HorizontalProperties {
    pub children: Vec<Rc<RefCell<ControlObject>>>
}

pub struct HorizontalData {
    pub properties: HorizontalProperties,
    pub parent: Option<Weak<RefCell<ControlObject>>>,
}

pub struct Horizontal {
    pub data: HorizontalData,
    style: Box<Style<HorizontalData>>,
}

impl Horizontal {
    pub fn new(children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<Self>> {
        let horizontal = Rc::new(RefCell::new(Horizontal {
            data: HorizontalData {
                properties: HorizontalProperties { children: children },
                parent: None,
            },
            style: Box::new(HorizontalDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
                desired_size: RefCell::new(Vec::new())
            }),
        }));

        for child in horizontal.borrow_mut().data.properties.children.iter_mut() {
            let horizontal_weak = Rc::downgrade(&horizontal);
            child.borrow_mut().set_parent(horizontal_weak);
        }

        horizontal
    }
}

impl Control for Horizontal {
    type Data = HorizontalData;

    fn get_data(&self) -> &Self::Data {
        &self.data
    }

    fn get_style(&self) -> &Box<Style<Self::Data>> {
        &self.style
    }

    //fn is_dirty(&self) -> bool;
    //fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref test) = self.data.parent {
            test.upgrade()
        } else {
            None
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.data.parent = Some(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        self.data.properties.children.clone()
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

impl Style<HorizontalData> for HorizontalDefaultStyle {
    fn get_preferred_size(&self, data: &HorizontalData, drawing_context: &mut DrawingContext, size: Size) -> Size {
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

    fn set_rect(&mut self, data: &mut HorizontalData, rect: Rect) {
        self.rect = rect;

        let mut child_rect = rect;
        let desired_size = self.desired_size.borrow();

        for (i, child) in data.properties.children.iter_mut().enumerate() {
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

    fn hit_test(&self, data: &HorizontalData, point: Point) -> HitTestResult {
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

    fn to_primitives(&self, data: &HorizontalData,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in &data.properties.children {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}


//
// object safe trait
//

impl ControlObject for Horizontal {
    //fn is_dirty(&self) -> bool;
    //fn set_is_dirty(&mut self, is_dirty: bool);

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        (self as &Control<Data = HorizontalData>).get_parent()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        (self as &mut Control<Data = HorizontalData>).set_parent(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut Control<Data = HorizontalData>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Data = HorizontalData>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_data(), drawing_context, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        let style = &mut self.style;
        let data = &mut self.data;
        style.set_rect(data, rect);
    }

    fn get_rect(&self) -> Rect {
        self.get_style().get_rect()
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.get_style().hit_test(self.get_data(), point)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(self.get_data(),
            drawing_context)
    }
}
