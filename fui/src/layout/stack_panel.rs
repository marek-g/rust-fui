use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use children_collection::*;
use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use style::*;
use typed_builder::TypedBuilder;
use view::*;

pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(TypedBuilder)]
pub struct StackPanel {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,
}

impl View for StackPanel {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(self, StackPanelDefaultStyle::new(), context)
    }
}

//
// StackPanel Default Style
//

pub struct StackPanelDefaultStyle {
    rect: Rect,
}

impl StackPanelDefaultStyle {
    pub fn new() -> Self {
        StackPanelDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
    }
}

impl Style<StackPanel> for StackPanelDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut StackPanel,
        _control: &Rc<RefCell<Control<StackPanel>>>,
    ) {
    }

    fn handle_event(
        &mut self,
        _data: &mut StackPanel,
        _children: &Box<dyn ChildrenSource>,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &StackPanel,
        children: &Box<dyn ChildrenSource>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) {
        let mut result = Rect::new(0.0f32, 0.0f32, 0f32, 0f32);

        match data.orientation {
            Orientation::Horizontal => {
                let available_size = Size::new(f32::INFINITY, size.height);

                for child in children.iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width += child_size.width;
                    result.height = result.height.max(child_size.height);
                }
            },
            Orientation::Vertical => {
                let available_size = Size::new(size.width, f32::INFINITY);

                for child in children.iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width = result.width.max(child_size.width);
                    result.height += child_size.height;
                }
            },
        }

        self.rect = result;
    }

    fn set_rect(
        &mut self,
        data: &StackPanel,
        children: &Box<dyn ChildrenSource>,
        rect: Rect,
    ) {
        self.rect = rect;

        let mut child_rect = rect;

        match data.orientation {
            Orientation::Horizontal => {
                for child in children.iter() {
                    let child_size = child.borrow_mut().get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;
                    child.borrow_mut().set_rect(child_rect);
                    child_rect.x += child_rect.width;
                }
            },
            Orientation::Vertical => {
                for child in children.iter() {
                    let child_size = child.borrow_mut().get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;
                    child.borrow_mut().set_rect(child_rect);
                    child_rect.y += child_rect.height;
                }
            },
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &StackPanel,
        children: &Box<dyn ChildrenSource>,
        point: Point,
    ) -> HitTestResult {
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
        _data: &StackPanel,
        children: &Box<dyn ChildrenSource>,
        drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        for child in children.iter() {
            vec.append(&mut child.borrow().to_primitives(drawing_context));
        }

        vec
    }
}
