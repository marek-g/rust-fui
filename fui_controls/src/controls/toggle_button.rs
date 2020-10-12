use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::{Brush, PathElement, Primitive};
use drawing::transformation::*;
use drawing::{
    primitive_extensions::circle_path,
    units::{PixelPoint, PixelThickness},
};
use fui_core::*;
use typed_builder::TypedBuilder;

use crate::style::*;
use crate::Alignment;

#[derive(TypedBuilder)]
pub struct ToggleButton {
    #[builder(default = Property::new(false))]
    pub is_checked: Property<bool>,
}

impl ToggleButton {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultToggleButtonStyle::new(
                    DefaultToggleButtonStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default ToggleButton Style
//

#[derive(TypedBuilder)]
pub struct DefaultToggleButtonStyleParams {}

pub struct DefaultToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultToggleButtonStyle {
    pub fn new(_params: DefaultToggleButtonStyleParams) -> Self {
        DefaultToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for DefaultToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.is_checked
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_tapped
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.is_checked.change(|val| !val);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
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
            content_size.width + 20.0f32,
            content_size.height + 20.0f32,
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            !data.is_checked.get()
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut vec,
            x,
            y,
            width,
            height,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}

//
// CheckBox ToggleButton Style
//

const CHECKBOX_BUTTON_SIZE: f32 = 24.0f32;
const CHECKBOX_MARGIN: f32 = 6.0f32;

#[derive(TypedBuilder)]
pub struct CheckBoxToggleButtonStyleParams {}

pub struct CheckBoxToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl CheckBoxToggleButtonStyle {
    pub fn new(_params: CheckBoxToggleButtonStyleParams) -> Self {
        CheckBoxToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for CheckBoxToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.is_checked
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_tapped
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.is_checked.change(|val| !val);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let children = control_context.get_children();
        let content_size = if let Some(ref content) = children.into_iter().next() {
            let child_size = Size::new(
                if size.width.is_finite() {
                    0f32.max(size.width - CHECKBOX_BUTTON_SIZE - CHECKBOX_MARGIN * 2.0f32)
                } else {
                    size.width
                },
                if size.height.is_finite() {
                    CHECKBOX_BUTTON_SIZE.max(size.height)
                } else {
                    size.height
                },
            );
            content.borrow_mut().measure(drawing_context, child_size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };
        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + CHECKBOX_BUTTON_SIZE + CHECKBOX_MARGIN * 2.0f32,
            CHECKBOX_BUTTON_SIZE.max(content_size.height),
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + CHECKBOX_BUTTON_SIZE + CHECKBOX_MARGIN,
            rect.y,
            rect.width - CHECKBOX_BUTTON_SIZE - CHECKBOX_MARGIN * 2.0f32,
            rect.height,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button_rounded(
            &mut vec,
            x,
            y,
            CHECKBOX_BUTTON_SIZE,
            height,
            3.0f32,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        if is_pressed {
            let mut tick_path = Vec::with_capacity(3);
            tick_path.push(PathElement::MoveTo(PixelPoint::new(
                x + CHECKBOX_BUTTON_SIZE / 2.0f32 - 4.0f32,
                y + height / 2.0f32 - 1.0f32,
            )));
            tick_path.push(PathElement::LineTo(PixelPoint::new(
                x + CHECKBOX_BUTTON_SIZE / 2.0f32 - 1.0f32,
                y + height / 2.0f32 + 5.0f32,
            )));
            tick_path.push(PathElement::LineTo(PixelPoint::new(
                x + CHECKBOX_BUTTON_SIZE / 2.0f32 + 5.0f32,
                y + height / 2.0f32 - 7.0f32,
            )));

            vec.push(Primitive::Stroke {
                path: tick_path,
                thickness: PixelThickness::new(2.0f32),
                brush: Brush::Color {
                    color: [1.0f32, 1.0f32, 1.0f32, 0.80f32],
                },
            });
        }

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}

//
// Tab ToggleButton Style
// (cannot be unpressed).
//

#[derive(TypedBuilder)]
pub struct TabToggleButtonStyleParams {}

pub struct TabToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl TabToggleButtonStyle {
    pub fn new(_params: TabToggleButtonStyleParams) -> Self {
        TabToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for TabToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.is_checked
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_tapped
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.is_checked.set(true);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
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
            content_size.width + 20.0f32,
            content_size.height + 20.0f32,
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut vec,
            x,
            y,
            width,
            height,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}

//
// Radio ToggleButton Style
// (cannot be unpressed).
//

const RADIO_BUTTON_SIZE: f32 = 24.0f32;
const RADIO_BULLET_SIZE: f32 = 14.0f32;
const RADIO_MARGIN: f32 = 6.0f32;

#[derive(TypedBuilder)]
pub struct RadioToggleButtonStyleParams {}

pub struct RadioToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    event_subscriptions: Vec<EventSubscription>,
}

impl RadioToggleButtonStyle {
    pub fn new(_params: RadioToggleButtonStyleParams) -> Self {
        RadioToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for RadioToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.is_checked
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_tapped
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.is_checked.set(true);
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let children = control_context.get_children();
        let content_size = if let Some(ref content) = children.into_iter().next() {
            let child_size = Size::new(
                if size.width.is_finite() {
                    0f32.max(size.width - RADIO_BUTTON_SIZE - RADIO_MARGIN * 2.0f32)
                } else {
                    size.width
                },
                if size.height.is_finite() {
                    RADIO_BUTTON_SIZE.max(size.height)
                } else {
                    size.height
                },
            );
            content.borrow_mut().measure(drawing_context, child_size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };
        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            content_size.width + RADIO_BUTTON_SIZE + RADIO_MARGIN * 2.0f32,
            RADIO_BUTTON_SIZE.max(content_size.height),
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        self.rect = rect;

        let content_rect = Rect::new(
            rect.x + RADIO_BUTTON_SIZE + RADIO_MARGIN,
            rect.y,
            rect.width - RADIO_BUTTON_SIZE - RADIO_MARGIN * 2.0f32,
            rect.height,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button_rounded(
            &mut vec,
            x,
            y,
            RADIO_BUTTON_SIZE,
            height,
            3.0f32,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        if is_pressed {
            vec.push(Primitive::Fill {
                path: circle_path(
                    PixelPoint::new(x + RADIO_BUTTON_SIZE / 2.0f32, y + height / 2.0f32),
                    RADIO_BULLET_SIZE / 2.0f32,
                ),
                brush: Brush::Color {
                    color: [1.0f32, 1.0f32, 1.0f32, 0.80f32],
                },
            });
        }

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}

//
// DropDown ToggleButton Style
// (cannot be unpressed,
// emit clicked event).
//

#[derive(TypedBuilder)]
pub struct DropDownToggleButtonStyleParams {
    #[builder(default = Callback::empty())]
    pub clicked: Callback<()>,
}

pub struct DropDownToggleButtonStyle {
    rect: Rect,
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    clicked: Callback<()>,
    event_subscriptions: Vec<EventSubscription>,
}

impl DropDownToggleButtonStyle {
    pub fn new(params: DropDownToggleButtonStyleParams) -> Self {
        DropDownToggleButtonStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            clicked: params.clicked,
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ToggleButton> for DropDownToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            data.is_checked
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_tapped
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            self.is_focused
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    data.is_checked.set(true);
                    self.clicked.emit(());
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &control_context, *position) {
                    self.is_tapped.set(true);
                } else {
                    self.is_tapped.set(false);
                }
            }

            ControlEvent::HoverChange(value) => {
                self.is_hover.set(value);
            }

            ControlEvent::FocusChange(value) => {
                self.is_focused.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
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
            content_size.width + 20.0f32,
            content_size.height + 20.0f32,
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        let map = control_context.get_attached_values();
        Alignment::apply(
            &mut self.rect,
            rect,
            &map,
            Alignment::Stretch,
            Alignment::Start,
        );

        let content_rect = Rect::new(
            self.rect.x + 10.0f32,
            self.rect.y + 10.0f32,
            self.rect.width - 20.0f32,
            self.rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(content_rect);
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        _control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut vec,
            x,
            y,
            width,
            height,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            let (mut vec2, mut overlay2) = content.borrow_mut().to_primitives(drawing_context);
            if is_pressed {
                vec2.translate(PixelPoint::new(1.0f32, 1.0f32));
            }
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
