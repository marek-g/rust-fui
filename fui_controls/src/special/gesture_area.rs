use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
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
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                data.tap_down.emit(());
            }

            ControlEvent::TapUp { ref position } => {
                data.tap_up.emit(());
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut GestureArea,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().measure(drawing_context, size);
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut GestureArea,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(rect);
        }
    }

    fn get_rect(&self, control_context: &ControlContext) -> Rect {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().get_rect()
        } else {
            Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
        }
    }

    fn hit_test(
        &self,
        _data: &GestureArea,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        let children = control_context.get_children();
        let rect = if let Some(child) = children.into_iter().next() {
            child.borrow().get_rect()
        } else {
            Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
        };

        if point.is_inside(&rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &GestureArea,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().to_primitives(drawing_context)
        } else {
            (Vec::new(), Vec::new())
        }
    }
}
