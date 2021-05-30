use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Shadow {}

impl Shadow {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultShadowStyle::new(
                    DefaultShadowStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Shadow Style
//

#[derive(TypedBuilder)]
pub struct DefaultShadowStyleParams {
    #[builder(default = 6.0f32)]
    size: f32,
}

pub struct DefaultShadowStyle {
    params: DefaultShadowStyleParams,
}

impl DefaultShadowStyle {
    pub fn new(params: DefaultShadowStyleParams) -> Self {
        DefaultShadowStyle { params }
    }
}

impl Style<Shadow> for DefaultShadowStyle {
    fn setup(&mut self, _data: &mut Shadow, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut Shadow,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut Shadow,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();

        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut Shadow,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(drawing_context, rect);
        }
    }

    fn hit_test(
        &self,
        _data: &Shadow,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            let children = control_context.get_children();
            if let Some(ref content) = children.into_iter().next() {
                let c = content.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    if child_hit_test.is_some() {
                        return child_hit_test;
                    }
                }
            }
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn to_primitives(
        &self,
        _data: &Shadow,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();
        let rect = control_context.get_rect();

        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        default_theme::shadow_under_rect(&mut vec, x, y, width, height, self.params.size);

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
