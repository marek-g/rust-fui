use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::transformation::*;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct Popup {
    #[builder(default = Property::new(false))]
    pub is_open: Property<bool>,
}

impl Popup {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultPopupStyle::new(DefaultPopupStyleParams::builder().build()))
            }),
            context)
    }
}

//
// Default Popup Style
//

#[derive(TypedBuilder)]
pub struct DefaultPopupStyleParams {}

pub struct DefaultPopupStyle {
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultPopupStyle {
    pub fn new(_params: DefaultPopupStyleParams) -> Self {
        DefaultPopupStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Popup> for DefaultPopupStyle {
    fn setup(&mut self, data: &mut Popup, control_context: &mut ControlContext) {
        let self_rc = control_context.get_self_rc();
        self.event_subscriptions.push(
            data.is_open.on_changed(move |is_open| {
                let window_service = self_rc
                    .borrow().get_context().get_services()
                    .map(|services| services.upgrade())
                    .unwrap_or(None)
                    .map(|services| services.borrow_mut().get_window_service())
                    .unwrap_or(None);

                if let Some(window_service) = window_service {
                    if is_open {
                        window_service.borrow_mut().add_layer(self_rc.clone());
                    } else {
                        window_service.borrow_mut().remove_layer(&self_rc);
                    }
                }
            })
        );
    }

    fn handle_event(
        &mut self,
        data: &mut Popup,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut Popup,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        if let Some(child) = control_context.get_children().into_iter().next() {
            child.borrow_mut().measure(drawing_context, size);
            self.rect = child.borrow().get_rect();
        } else {
            self.rect = Rect::new(0.0f32, 0.0f32, 0f32, 0f32);
        }
    }

    fn set_rect(&mut self, _data: &mut Popup, control_context: &mut ControlContext, rect: Rect) {
        self.rect = rect;

        if let Some(child) = control_context.get_children().into_iter().next() {
            child.borrow_mut().set_rect(rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &Popup, control_context: &ControlContext, point: Point) -> HitTestResult {
        if let Some(child) = control_context.get_children().into_iter().next() {
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
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &Popup,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        if let Some(child) = control_context.get_children().into_iter().next() {
            let (mut vec2, mut overlay2) = child.borrow().to_primitives(drawing_context);
            vec = vec2;
            overlay = overlay2;
        }

        (vec, overlay)
    }
}
