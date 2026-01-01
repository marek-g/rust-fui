use std::cell::RefCell;
use std::rc::Rc;

use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct GestureArea {
    #[builder(default = Callback::empty())]
    pub tap_down: Callback<()>,

    #[builder(default = Callback::empty())]
    pub tap_up: Callback<()>,

    #[builder(default = Callback::empty())]
    pub hover_change: Callback<bool>,
}

impl GestureArea {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultGestureAreaStyle::new(
                    DefaultGestureAreaStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default GestureArea Style
//

#[derive(TypedBuilder)]
pub struct DefaultGestureAreaStyleParams {}

pub struct DefaultGestureAreaStyle;

impl DefaultGestureAreaStyle {
    pub fn new(_params: DefaultGestureAreaStyleParams) -> Self {
        DefaultGestureAreaStyle {}
    }
}

impl Style<GestureArea> for DefaultGestureAreaStyle {
    fn setup(&mut self, _data: &mut GestureArea, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        data: &mut GestureArea,
        _control_context: &mut ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                data.tap_down.emit(());
            }

            ControlEvent::TapUp { .. } => {
                data.tap_up.emit(());
            }

            ControlEvent::HoverChange(value) => {
                data.hover_change.emit(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut GestureArea,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();
        match children.into_iter().next() {
            Some(child) => {
                child.borrow_mut().measure(drawing_context, size);
                let child_rect = child.borrow().get_rect();
                Size::new(child_rect.width, child_rect.height)
            }
            _ => Size::new(0.0f32, 0.0f32),
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut GestureArea,
        control_context: &mut ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(drawing_context, rect);
        }
    }

    fn hit_test(
        &self,
        _data: &GestureArea,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        let children = control_context.get_children();
        let rect = match children.into_iter().next() {
            Some(child) => child.borrow().get_rect(),
            _ => Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32),
        };

        if point.is_inside(&rect) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        _data: &GestureArea,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let children = control_context.get_children();
        match children.into_iter().next() {
            Some(child) => child.borrow_mut().draw(drawing_context),
            _ => (),
        }
    }
}
