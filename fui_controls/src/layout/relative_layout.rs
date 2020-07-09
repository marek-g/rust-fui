use std::cell::RefCell;
use std::f32;
use std::rc::{Rc, Weak};

use drawing::primitive::Primitive;
use fui::*;
use typed_builder::TypedBuilder;

pub enum RelativePlacement {
    FullWindow,
    BelowControl(Weak<RefCell<dyn ControlObject>>),
}

///
/// Warning!
///
/// RelativeLayout is currently of limited use and dangerous.
///
/// During the layout phase it borrows referenced control.
/// Because layout phase is recursive it is safe to reference
/// controls in a different layer only.
///
/// So far, the only safe way of use it is as in the Popup control.
///
#[derive(TypedBuilder)]
pub struct RelativeLayout {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,

    #[builder(default = RelativePlacement::FullWindow)]
    pub placement: RelativePlacement,
}

impl RelativeLayout {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultRelativeLayoutStyle::new(
                    DefaultRelativeLayoutStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default RelativeLayout Style
//

#[derive(TypedBuilder)]
pub struct DefaultRelativeLayoutStyleParams {}

pub struct DefaultRelativeLayoutStyle {
    rect: Rect,
}

impl DefaultRelativeLayoutStyle {
    pub fn new(_params: DefaultRelativeLayoutStyleParams) -> Self {
        DefaultRelativeLayoutStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
    }
}

impl Style<RelativeLayout> for DefaultRelativeLayoutStyle {
    fn setup(&mut self, _data: &mut RelativeLayout, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut RelativeLayout,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut RelativeLayout,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let mut result = Rect::new(0.0f32, 0.0f32, 0f32, 0f32);

        let children = control_context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                let available_size = Size::new(f32::INFINITY, size.height);

                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width += child_size.width;
                    result.height = result.height.max(child_size.height);
                }
            }
            Orientation::Vertical => {
                let available_size = Size::new(size.width, f32::INFINITY);

                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width = result.width.max(child_size.width);
                    result.height += child_size.height;
                }
            }
        }

        self.rect = result;
    }

    fn set_rect(
        &mut self,
        data: &mut RelativeLayout,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        self.rect = rect;

        let offset = match &data.placement {
            RelativePlacement::FullWindow => (0f32, 0f32),

            RelativePlacement::BelowControl(control) => {
                if let Some(control) = control.upgrade() {
                    let rect = control.borrow_mut().get_rect();
                    (rect.x + rect.width, rect.y + rect.height)
                } else {
                    (0f32, 0f32)
                }
            }
        };

        let mut child_rect = rect;
        child_rect.x += offset.0;
        child_rect.y += offset.1;

        let children = control_context.get_children();

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

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &RelativeLayout,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
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
        _data: &RelativeLayout,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let children = control_context.get_children();
        for child in children.into_iter() {
            let (mut vec2, mut overlay2) = child.borrow().to_primitives(drawing_context);
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
