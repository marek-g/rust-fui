use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct StackPanel {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,
}

impl Control for StackPanel {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultStackPanelStyle::new(DefaultStackPanelStyleParams::builder().build()))
            }),
            context)
    }
}

//
// Default StackPanel Style
//


#[derive(TypedBuilder)]
pub struct DefaultStackPanelStyleParams {}

pub struct DefaultStackPanelStyle {
    rect: Rect,
}

impl DefaultStackPanelStyle {
    pub fn new(_params: DefaultStackPanelStyleParams) -> Self {
        DefaultStackPanelStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
    }
}

impl Style<StackPanel> for DefaultStackPanelStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut StackPanel,
        _control: &Rc<RefCell<StyledControl<StackPanel>>>,
    ) {
    }

    fn handle_event(
        &mut self,
        _data: &mut StackPanel,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut StackPanel,
        context: &mut ControlContext,
        resources: &mut dyn Resources,
        size: Size,
    ) {
        let mut result = Rect::new(0.0f32, 0.0f32, 0f32, 0f32);

        let children = context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                let available_size = Size::new(f32::INFINITY, size.height);

                for child in children.into_iter() {
                    child.borrow_mut().measure(resources, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width += child_size.width;
                    result.height = result.height.max(child_size.height);
                }
            }
            Orientation::Vertical => {
                let available_size = Size::new(size.width, f32::INFINITY);

                for child in children.into_iter() {
                    child.borrow_mut().measure(resources, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width = result.width.max(child_size.width);
                    result.height += child_size.height;
                }
            }
        }

        self.rect = result;
    }

    fn set_rect(&mut self, data: &mut StackPanel, context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        let mut child_rect = rect;

        let children = context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                for child in children.into_iter() {
                    let child_size = child.borrow_mut().get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;
                    child.borrow_mut().set_rect(child_rect);
                    child_rect.x += child_rect.width;
                }
            }
            Orientation::Vertical => {
                for child in children.into_iter() {
                    let child_size = child.borrow_mut().get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;
                    child.borrow_mut().set_rect(child_rect);
                    child_rect.y += child_rect.height;
                }
            }
        }
    }

    fn get_rect(&self, _context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &StackPanel,
        context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = context.get_children();
            for child in children.into_iter() {
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
        context: &ControlContext,
        resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let children = context.get_children();
        for child in children.into_iter() {
            vec.append(&mut child.borrow().to_primitives(resources));
        }

        vec
    }
}
