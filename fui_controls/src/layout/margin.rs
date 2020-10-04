use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Margin {
    #[builder(default = Thickness::new(5.0f32, 5.0f32, 5.0f32, 5.0f32))]
    pub thickness: Thickness,
}

impl Margin {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultMarginStyle::new(
                    DefaultMarginStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Margin Style
//

#[derive(TypedBuilder)]
pub struct DefaultMarginStyleParams {}

pub struct DefaultMarginStyle {
    rect: Rect,
}

impl DefaultMarginStyle {
    pub fn new(_params: DefaultMarginStyleParams) -> Self {
        DefaultMarginStyle {
            rect: Rect::empty(),
        }
    }
}

impl Style<Margin> for DefaultMarginStyle {
    fn setup(&mut self, _data: &mut Margin, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut Margin,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut Margin,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        mut size: Size,
    ) {
        if size.width.is_finite() {
            size.width = 0.0f32.max(size.width - data.thickness.left - data.thickness.right);
        }
        if size.height.is_finite() {
            size.height = 0.0f32.max(size.height - data.thickness.top - data.thickness.bottom);
        }

        let children = control_context.get_children();
        let content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + data.thickness.left + data.thickness.right,
            content_size.height + data.thickness.top + data.thickness.bottom,
        );
    }

    fn set_rect(
        &mut self,
        data: &mut Margin,
        control_context: &mut ControlContext,
        mut rect: Rect,
    ) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + data.thickness.left,
            rect.y + data.thickness.top,
            rect.width - data.thickness.left - data.thickness.right,
            rect.height - data.thickness.top - data.thickness.bottom,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        };
    }

    fn get_rect(&self, control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &Margin,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let child_hit_test = c.hit_test(point);
                match child_hit_test {
                    HitTestResult::Current => return HitTestResult::Child(content.clone()),
                    HitTestResult::Child(..) => return child_hit_test,
                    HitTestResult::Nothing => (),
                }
            }
        }
        HitTestResult::Nothing
    }

    fn to_primitives(
        &self,
        _data: &Margin,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow().to_primitives(drawing_context)
        } else {
            (Vec::new(), Vec::new())
        }
    }
}
