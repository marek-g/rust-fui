use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
use typed_builder::TypedBuilder;

use crate::style::*;

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
    ) -> Rc<dyn ControlObject> {
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
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
}

impl DefaultToggleButtonStyle {
    pub fn new(_params: DefaultToggleButtonStyleParams) -> Self {
        DefaultToggleButtonStyle {
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
        }
    }
}

impl Style<ToggleButton> for DefaultToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.is_checked);
        control_context.dirty_watch_property(&self.is_tapped);
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.is_checked.change(|val| !val);
                    } else {
                        self.is_tapped.set(false);
                    }
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(hit_control) => {
                        if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                            self.is_tapped.set(true);
                        } else {
                            self.is_tapped.set(false);
                        }
                    }
                    _ => {
                        self.is_tapped.set(false);
                    }
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
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                content.measure(drawing_context, size);
                let rect = content.get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let is_pressed = if self.is_tapped.get() {
            !data.is_checked.get()
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut drawing_context.display,
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
            if is_pressed {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.draw(drawing_context);
            if is_pressed {
                drawing_context.display.restore();
            }
        }
    }
}

//
// CheckBox ToggleButton Style
//



#[derive(TypedBuilder)]
pub struct CheckBoxToggleButtonStyleParams {}

pub struct CheckBoxToggleButtonStyle {
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
}

impl CheckBoxToggleButtonStyle {
    pub fn new(_params: CheckBoxToggleButtonStyleParams) -> Self {
        CheckBoxToggleButtonStyle {
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
        }
    }
}

impl Style<ToggleButton> for CheckBoxToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.is_checked);
        control_context.dirty_watch_property(&self.is_tapped);
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.is_checked.change(|val| !val);
                    } else {
                        self.is_tapped.set(false);
                    }
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(hit_control) => {
                        if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                            self.is_tapped.set(true);
                        } else {
                            self.is_tapped.set(false);
                        }
                    }
                    _ => {
                        self.is_tapped.set(false);
                    }
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
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let check_box_button_size = control_context
            .get_inherited_value::<CheckBoxButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::CHECK_BOX_BUTTON_SIZE);
        let check_box_margin = control_context
            .get_inherited_value::<CheckBoxMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::CHECK_BOX_MARGIN);

        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                let child_size = Size::new(
                    if size.width.is_finite() {
                        0f32.max(size.width - check_box_button_size - check_box_margin * 2.0f32)
                    } else {
                        size.width
                    },
                    if size.height.is_finite() {
                        check_box_button_size.max(size.height)
                    } else {
                        size.height
                    },
                );
                content.measure(drawing_context, child_size);
                let rect = content.get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(
            content_size.width + check_box_button_size + check_box_margin * 2.0f32,
            check_box_button_size.max(content_size.height),
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let check_box_button_size = control_context
            .get_inherited_value::<CheckBoxButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::CHECK_BOX_BUTTON_SIZE);
        let check_box_margin = control_context
            .get_inherited_value::<CheckBoxMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::CHECK_BOX_MARGIN);

        let content_rect = Rect::new(
            rect.x + check_box_button_size + check_box_margin,
            rect.y,
            rect.width - check_box_button_size - check_box_margin * 2.0f32,
            rect.height,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let height = rect.height;

        let check_box_button_size = control_context
            .get_inherited_value::<CheckBoxButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::CHECK_BOX_BUTTON_SIZE);

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button_rounded(
            &mut drawing_context.display,
            x,
            y,
            check_box_button_size,
            height,
            3.0f32,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        if is_pressed {
            let mut tick_path_builder = DrawingPathBuilder::default();
            tick_path_builder.move_to((
                x + check_box_button_size / 2.0f32 - 4.0f32,
                y + height / 2.0f32 - 1.0f32,
            ));
            tick_path_builder.line_to((
                x + check_box_button_size / 2.0f32 - 1.0f32,
                y + height / 2.0f32 + 5.0f32,
            ));
            tick_path_builder.line_to((
                x + check_box_button_size / 2.0f32 + 5.0f32,
                y + height / 2.0f32 - 7.0f32,
            ));

            let paint = DrawingPaint::stroke_color(Color::rgba(1.0, 1.0, 1.0, 1.0), 2.0);

            drawing_context
                .display
                .draw_path(&tick_path_builder.build(), paint);
        }

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            if is_pressed {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.draw(drawing_context);
            if is_pressed {
                drawing_context.display.restore();
            }
        }
    }
}

//
// Tab ToggleButton Style
// (cannot be unpressed).
//

#[derive(TypedBuilder)]
pub struct TabToggleButtonStyleParams {}

pub struct TabToggleButtonStyle {
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
}

impl TabToggleButtonStyle {
    pub fn new(_params: TabToggleButtonStyleParams) -> Self {
        TabToggleButtonStyle {
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
        }
    }
}

impl Style<ToggleButton> for TabToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.is_checked);
        control_context.dirty_watch_property(&self.is_tapped);
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.is_checked.set(true);
                    } else {
                        self.is_tapped.set(false);
                    }
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(_) => match self.hit_test(&data, &control_context, *position) {
                        Some(_) => {
                            self.is_tapped.set(true);
                        }
                        _ => {
                            self.is_tapped.set(false);
                        }
                    },
                    _ => {
                        self.is_tapped.set(false);
                    }
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
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                content.measure(drawing_context, size);
                let rect = content.get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut drawing_context.display,
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
            if is_pressed {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.draw(drawing_context);
            if is_pressed {
                drawing_context.display.restore();
            }
        }
    }
}

//
// Radio ToggleButton Style
// (cannot be unpressed).
//

#[derive(TypedBuilder)]
pub struct RadioToggleButtonStyleParams {}

pub struct RadioToggleButtonStyle {
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
}

impl RadioToggleButtonStyle {
    pub fn new(_params: RadioToggleButtonStyleParams) -> Self {
        RadioToggleButtonStyle {
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
        }
    }
}

impl Style<ToggleButton> for RadioToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.is_checked);
        control_context.dirty_watch_property(&self.is_tapped);
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.is_checked.set(true);
                    } else {
                        self.is_tapped.set(false);
                    }
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(_) => match self.hit_test(&data, &control_context, *position) {
                        Some(_) => {
                            self.is_tapped.set(true);
                        }
                        _ => {
                            self.is_tapped.set(false);
                        }
                    },
                    _ => {
                        self.is_tapped.set(false);
                    }
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
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let radio_button_size = control_context
            .get_inherited_value::<RadioButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_BUTTON_SIZE);
        let radio_margin = control_context
            .get_inherited_value::<RadioMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_MARGIN);

        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                let child_size = Size::new(
                    if size.width.is_finite() {
                        0f32.max(size.width - radio_button_size - radio_margin * 2.0f32)
                    } else {
                        size.width
                    },
                    if size.height.is_finite() {
                        radio_button_size.max(size.height)
                    } else {
                        size.height
                    },
                );
                content.measure(drawing_context, child_size);
                let rect = content.get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(
            content_size.width + radio_button_size + radio_margin * 2.0f32,
            radio_button_size.max(content_size.height),
        )
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let radio_button_size = control_context
            .get_inherited_value::<RadioButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_BUTTON_SIZE);
        let radio_margin = control_context
            .get_inherited_value::<RadioMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_MARGIN);

        let content_rect = Rect::new(
            rect.x + radio_button_size + radio_margin,
            rect.y,
            rect.width - radio_button_size - radio_margin * 2.0f32,
            rect.height,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let r = control_context.get_rect();
        let x = r.x;
        let y = r.y;
        let height = r.height;

        let radio_button_size = control_context
            .get_inherited_value::<RadioButtonSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_BUTTON_SIZE);
        let radio_bullet_size = control_context
            .get_inherited_value::<RadioBulletSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::RADIO_BULLET_SIZE);

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button_rounded(
            &mut drawing_context.display,
            x,
            y,
            radio_button_size,
            height,
            3.0f32,
            is_pressed,
            self.is_hover.get(),
            self.is_focused.get(),
        );

        if is_pressed {
            drawing_context.display.draw_oval(
                rect(
                    x + (radio_button_size - radio_bullet_size) / 2.0,
                    y + (height - radio_bullet_size) / 2.0,
                    radio_bullet_size,
                    radio_bullet_size,
                ),
                Color::rgba(1.0, 1.0, 1.0, 0.8),
            );
        }

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            if is_pressed {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.draw(drawing_context);
            if is_pressed {
                drawing_context.display.restore();
            }
        }
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
    is_tapped: Property<bool>,
    is_hover: Property<bool>,
    is_focused: Property<bool>,
    _clicked: Callback<()>,
}

impl DropDownToggleButtonStyle {
    pub fn new(params: DropDownToggleButtonStyleParams) -> Self {
        DropDownToggleButtonStyle {
            is_tapped: Property::new(false),
            is_hover: Property::new(false),
            is_focused: Property::new(false),
            _clicked: params.clicked,
        }
    }
}

impl Style<ToggleButton> for DropDownToggleButtonStyle {
    fn setup(&mut self, data: &mut ToggleButton, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.is_checked);
        control_context.dirty_watch_property(&self.is_tapped);
        control_context.dirty_watch_property(&self.is_hover);
        control_context.dirty_watch_property(&self.is_focused);
    }

    fn handle_event(
        &mut self,
        data: &mut ToggleButton,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_tapped.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let Some(hit_control) = self.hit_test(&data, &control_context, *position) {
                    if Rc::ptr_eq(&hit_control, &control_context.get_self_rc()) {
                        data.is_checked.set(true);
                    } else {
                        self.is_tapped.set(false);
                    }
                }
                self.is_tapped.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                match self.hit_test(&data, &control_context, *position) {
                    Some(_) => match self.hit_test(&data, &control_context, *position) {
                        Some(_) => {
                            self.is_tapped.set(true);
                        }
                        _ => {
                            self.is_tapped.set(false);
                        }
                    },
                    _ => {
                        self.is_tapped.set(false);
                    }
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
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let children = control_context.get_children();
        let content_size = match children.into_iter().next() {
            Some(ref content) => {
                content.measure(drawing_context, size);
                let rect = content.get_rect();
                Size::new(rect.width, rect.height)
            }
            _ => Size::new(0f32, 0f32),
        };

        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(
        &mut self,
        _data: &mut ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        let content_rect = Rect::new(
            rect.x + 10.0f32,
            rect.y + 10.0f32,
            rect.width - 20.0f32,
            rect.height - 20.0f32,
        );

        let children = control_context.get_children();
        if let Some(ref content) = children.into_iter().next() {
            content.set_rect(drawing_context, content_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &ToggleButton,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ToggleButton,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let is_pressed = if self.is_tapped.get() {
            true
        } else {
            data.is_checked.get()
        };

        default_theme::button(
            &mut drawing_context.display,
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
            if is_pressed {
                drawing_context.display.save();
                drawing_context.display.translate(1.0, 1.0);
            }
            content.draw(drawing_context);
            if is_pressed {
                drawing_context.display.restore();
            }
        }
    }
}
