use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use drawing::primitive::Primitive;
use fui_core::RelativeLayout;
use fui_core::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

#[derive(Copy, Clone)]
pub enum PopupPlacement {
    /// The popup will fill full window area.
    FullSize,

    /// The popup will be placed above or below the parent
    /// (wherever is more space) and will be the size of the parent.
    BelowOrAboveParent,

    /// The popup will be placed to the left or to the right of the parent.
    LeftOrRightParent,
}

#[derive(Copy, Clone)]
pub enum PopupAutoHide {
    /// The popup will be not automatically hidden.
    None,

    /// The popup will be automatically closed when user clicks outside it.
    ClickedOutside,

    /// The popup will be automatically closed when user moves the cursor
    /// away the popup and it's parent or clicks outside it
    /// (the submenu like behavior).
    Menu,
}

#[derive(TypedBuilder)]
pub struct Popup {
    #[builder(default = Property::new(false))]
    pub is_open: Property<bool>,

    #[builder(default = PopupPlacement::FullSize)]
    pub placement: PopupPlacement,

    #[builder(default = PopupAutoHide::None)]
    pub auto_hide: PopupAutoHide,

    /// Called when auto hide has occured.
    #[builder(default = Callback::empty())]
    pub auto_hide_occured: Callback<()>,

    /// Popup does not pass through events to controls below
    /// except the area covered by this list of controls
    #[builder(default = Vec::new())]
    pub uncovered_controls: Vec<Weak<RefCell<dyn ControlObject>>>,
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
        let auto_hide_occured = data.auto_hide_occured.clone();
        let uncovered_controls = data.uncovered_controls.to_vec();

        let mut auto_hide_request_callback = Callback::empty();
        let mut is_open_property_clone = data.is_open.clone();
        auto_hide_request_callback.set(move |_| {
            is_open_property_clone.set(false);
            auto_hide_occured.emit(());
        });

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

                            PopupPlacement::LeftOrRightParent => {
                                let parent_weak =
                                    Rc::downgrade(&self_popup.get_context().get_parent().unwrap());
                                RelativePlacement::LeftOrRightControl(parent_weak)
                            }
                        };

                        let relative_auto_hide = match auto_hide {
                            PopupAutoHide::None => RelativeAutoHide::None,
                            PopupAutoHide::ClickedOutside => RelativeAutoHide::ClickedOutside,
                            PopupAutoHide::Menu => RelativeAutoHide::Menu,
                        };

                        let content = ui! {
                            RelativeLayout {
                                placement: relative_placement,
                                auto_hide: relative_auto_hide,
                                auto_hide_request: auto_hide_request_callback.clone(),
                                uncovered_controls: uncovered_controls.to_vec(),

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

    fn set_rect(
        &mut self,
        _data: &mut Popup,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &Popup,
        _control_context: &ControlContext,
        _point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        None
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
