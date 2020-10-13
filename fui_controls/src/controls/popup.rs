use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::layout::RelativeLayout;
use crate::RelativePlacement;
use drawing::primitive::Primitive;
use fui_core::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

#[derive(Copy, Clone)]
pub enum PopupPlacement {
    FullSize,
    BelowOrAboveParent,
}

#[derive(Copy, Clone)]
pub enum PopupAutoHide {
    None,
    ClickedOutside,
}

#[derive(TypedBuilder)]
pub struct Popup {
    #[builder(default = Property::new(false))]
    pub is_open: Property<bool>,

    #[builder(default = PopupPlacement::FullSize)]
    pub placement: PopupPlacement,

    #[builder(default = PopupAutoHide::ClickedOutside)]
    pub auto_hide: PopupAutoHide,
}

impl Popup {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultPopupStyle::new(
                    DefaultPopupStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default Popup Style
//

#[derive(TypedBuilder)]
pub struct DefaultPopupStyleParams {}

pub struct DefaultPopupStyle {
    popup_content: Rc<Cell<Option<Rc<RefCell<dyn ControlObject>>>>>,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultPopupStyle {
    pub fn new(_params: DefaultPopupStyleParams) -> Self {
        DefaultPopupStyle {
            popup_content: Rc::new(Cell::new(None)),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<Popup> for DefaultPopupStyle {
    fn setup(&mut self, data: &mut Popup, control_context: &mut ControlContext) {
        let self_rc = control_context.get_self_rc();
        let popup_content_rc = self.popup_content.clone();
        let placement = data.placement;
        let auto_hide = data.auto_hide;

        let mut clicked_outside = Callback::empty();
        if let PopupAutoHide::ClickedOutside = auto_hide {
            let mut is_open_property2 = Property::binded_two_way(&mut data.is_open);
            clicked_outside.set(move |_| {
                is_open_property2.set(false);
            });
        }

        let is_open_handler = move |is_open| {
            let window_service = self_rc
                .borrow()
                .get_context()
                .get_services()
                .map(|services| services.upgrade())
                .unwrap_or(None)
                .map(|services| services.borrow_mut().get_window_service())
                .unwrap_or(None);

            if let Some(window_service) = window_service {
                let self_popup = self_rc.borrow_mut();
                if let Some(first_child) =
                    self_popup.get_context().get_children().into_iter().next()
                {
                    if is_open {
                        let relative_placement = match placement {
                            PopupPlacement::FullSize => RelativePlacement::FullSize,

                            PopupPlacement::BelowOrAboveParent => {
                                let parent_weak =
                                    Rc::downgrade(&self_popup.get_context().get_parent().unwrap());
                                RelativePlacement::BelowOrAboveControl(parent_weak)
                            }
                        };

                        let content = ui! {
                            RelativeLayout {
                                placement: relative_placement,
                                clicked_outside: clicked_outside.clone(),

                                first_child,
                            }
                        };

                        popup_content_rc.set(Some(content.clone()));
                        window_service.borrow_mut().add_layer(content);
                    } else {
                        let content = popup_content_rc.replace(None).unwrap();
                        window_service.borrow_mut().remove_layer(&content);
                    }
                }
            }
        };

        self.event_subscriptions
            .push(data.is_open.on_changed(is_open_handler));
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
    ) -> Size {
        Size::new(0.0f32, 0.0f32)
    }

    fn set_rect(&mut self, _data: &mut Popup, _control_context: &mut ControlContext, _rect: Rect) {}

    fn hit_test(
        &self,
        _data: &Popup,
        _control_context: &ControlContext,
        _point: Point,
    ) -> HitTestResult {
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
