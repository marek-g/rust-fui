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
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultPopupStyle {
    pub fn new(_params: DefaultPopupStyleParams) -> Self {
        DefaultPopupStyle {
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
                    if let Some(first_child) = self_rc
                        .borrow().get_context().get_children().into_iter().next() {
                        if is_open {
                            window_service.borrow_mut().add_layer(first_child);
                        } else {
                            window_service.borrow_mut().remove_layer(&first_child);
                        }
                    }
                }
            })
        );
    }

    fn handle_event(
        &mut self,
        _data: &mut Popup,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        _data: &mut Popup,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _size: Size,
    ) {
    }

    fn set_rect(&mut self, _data: &mut Popup, _control_context: &mut ControlContext, _rect: Rect) {
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        Rect::new(0.0f32, 0.0f32, 0f32, 0f32)
    }

    fn hit_test(&self, _data: &Popup, _control_context: &ControlContext, _point: Point) -> HitTestResult {
        HitTestResult::Nothing
    }

    fn to_primitives(
        &self,
        _data: &Popup,
        _control_context: &ControlContext,
        _drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        (Vec::new(), Vec::new())
    }
}
