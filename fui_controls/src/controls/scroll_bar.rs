use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
use fui_drawing::DisplayListBuilder;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct ScrollBar {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,

    #[builder(default = Property::new(0.0f32))]
    pub min_value: Property<f32>,

    #[builder(default = Property::new(1.0f32))]
    pub max_value: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub value: Property<f32>,

    /// How much of the range of value is visible on the screen
    /// (affects the length of thumb)
    #[builder(default = Property::new(0.0f32))]
    pub viewport_size: Property<f32>,

    /// How much to modify the value on mouse wheel
    #[builder(default = Property::new(0.05f32))]
    pub single_step_size: Property<f32>,
}

impl ScrollBar {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<dyn ControlObject> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultScrollBarStyle::new(
                    DefaultScrollBarStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default ScrollBar Style
//



#[derive(TypedBuilder)]
pub struct DefaultScrollBarStyleParams {}

pub struct DefaultScrollBarStyle {
    thumb_pos_px: f32,
    thumb_size_px: f32,

    is_thumb_hover: Property<bool>,
    is_thumb_pressed: Property<bool>,
    pressed_offset: f32,
}

impl DefaultScrollBarStyle {
    pub fn new(_params: DefaultScrollBarStyleParams) -> Self {
        DefaultScrollBarStyle {
            thumb_pos_px: 0f32,
            thumb_size_px: 0f32,
            is_thumb_hover: Property::new(false),
            is_thumb_pressed: Property::new(false),
            pressed_offset: 0.0f32,
        }
    }

    fn calc_sizes(&mut self, data: &ScrollBar, control_context: &ControlContext, rect: Rect) {
        let start_margin = control_context
            .get_inherited_value::<ScrollBarStartMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_START_MARGIN);
        let end_margin = control_context
            .get_inherited_value::<ScrollBarEndMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_END_MARGIN);
        let min_thumb_size = control_context
            .get_inherited_value::<ScrollBarMinThumbSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_MIN_THUMB_SIZE);

        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => rect.width - start_margin - end_margin,
            Orientation::Vertical => rect.height - start_margin - end_margin,
        };
        let scroll_bar_size_f32 =
            data.max_value.get() - data.min_value.get() + data.viewport_size.get();

        self.thumb_size_px = ((data.viewport_size.get() * scroll_bar_size_px)
            / scroll_bar_size_f32)
            .round()
            .max(min_thumb_size);

        self.thumb_pos_px = ((scroll_bar_size_px - self.thumb_size_px)
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get()))
        .round();
    }
}

impl Style<ScrollBar> for DefaultScrollBarStyle {
    fn setup(&mut self, data: &mut ScrollBar, control_context: &ControlContext) {
        control_context.dirty_watch_property(&self.is_thumb_hover);
        control_context.dirty_watch_property(&self.is_thumb_pressed);
        control_context.dirty_watch_property(&data.min_value);
        control_context.dirty_watch_property(&data.max_value);
        control_context.dirty_watch_property(&data.value);
        control_context.dirty_watch_property(&data.viewport_size);
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollBar,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { position } => {
                let rect = control_context.get_rect();
                let start_margin = control_context
                    .get_inherited_value::<ScrollBarStartMargin>()
                    .map(|p| p.get())
                    .unwrap_or(default_theme::SCROLL_BAR_START_MARGIN);
                let pos = match data.orientation {
                    Orientation::Horizontal => position.x - rect.x - start_margin,
                    Orientation::Vertical => position.y - rect.y - start_margin,
                };
                if pos >= self.thumb_pos_px && pos < self.thumb_pos_px + self.thumb_size_px {
                    self.is_thumb_pressed.set(true);
                    self.pressed_offset = pos - self.thumb_pos_px;
                }
            }

            ControlEvent::TapUp { .. } => {
                self.is_thumb_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if self.is_thumb_pressed.get() {
                    let rect = control_context.get_rect();
                    let start_margin = control_context
                        .get_inherited_value::<ScrollBarStartMargin>()
                        .map(|p| p.get())
                        .unwrap_or(default_theme::SCROLL_BAR_START_MARGIN);
                    let end_margin = control_context
                        .get_inherited_value::<ScrollBarEndMargin>()
                        .map(|p| p.get())
                        .unwrap_or(default_theme::SCROLL_BAR_END_MARGIN);

                    let scroll_bar_size_px = match data.orientation {
                        Orientation::Horizontal => rect.width - start_margin - end_margin,
                        Orientation::Vertical => rect.height - start_margin - end_margin,
                    };

                    let pos = match data.orientation {
                        Orientation::Horizontal => position.x - rect.x - start_margin,
                        Orientation::Vertical => position.y - rect.y - start_margin,
                    };

                    let new_thumb_pos_px = pos - self.pressed_offset;
                    let new_value = (data.min_value.get()
                        + new_thumb_pos_px * (data.max_value.get() - data.min_value.get())
                            / (scroll_bar_size_px - self.thumb_size_px))
                        .max(data.min_value.get())
                        .min(data.max_value.get());

                    if new_value != data.value.get() {
                        data.value.set(new_value);
                    }
                }
            }

            ControlEvent::ScrollWheel { delta } => {
                match delta {
                    ScrollDelta::LineDelta(x, y) => {
                        let single_step = data.single_step_size.get();
                        let min_value = data.min_value.get();
                        let max_value = data.max_value.get();
                        let steps = if let Orientation::Vertical = data.orientation {
                            y
                        } else {
                            if x != 0.0f32 {
                                x
                            } else {
                                y
                            }
                        };
                        data.value.change(move |v| {
                            (v - steps * single_step).min(max_value).max(min_value)
                        });
                    }
                    ScrollDelta::PixelDelta(_, _) => (),
                };
            }

            ControlEvent::HoverChange(value) => {
                self.is_thumb_hover.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut ScrollBar,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        size: Size,
    ) -> Size {
        let min_thumb_size = control_context
            .get_inherited_value::<ScrollBarMinThumbSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_MIN_THUMB_SIZE);
        let min_size = min_thumb_size * 2.0;

        match data.orientation {
            Orientation::Horizontal => {
                let space = if size.width.is_infinite() {
                    min_size
                } else {
                    size.width
                };
                Size::new(min_size.max(space), 20.0f32)
            }
            Orientation::Vertical => {
                let space = if size.height.is_infinite() {
                    min_size
                } else {
                    size.height
                };
                Size::new(20.0f32, min_size.max(space))
            }
        }
    }

    fn set_rect(
        &mut self,
        data: &mut ScrollBar,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        rect: Rect,
    ) {
        self.calc_sizes(data, control_context, rect);
    }

    fn hit_test(
        &self,
        _data: &ScrollBar,
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
        data: &ScrollBar,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let r = control_context.get_rect();
        let x = r.x;
        let y = r.y;
        let width = r.width;
        let height = r.height;

        let start_margin = control_context
            .get_inherited_value::<ScrollBarStartMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_START_MARGIN);
        let end_margin = control_context
            .get_inherited_value::<ScrollBarEndMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_END_MARGIN);
        let side_margin = control_context
            .get_inherited_value::<ScrollBarSideMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::SCROLL_BAR_SIDE_MARGIN);

        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - start_margin - end_margin,
            Orientation::Vertical => height - start_margin - end_margin,
        };

        let background = default_theme::SCROLL_BAR_BACKGROUND;

        if self.thumb_pos_px > 0.0f32 {
            drawing_context.display.draw_rect(
                match data.orientation {
                    Orientation::Horizontal => rect(
                        x + start_margin,
                        y + side_margin,
                        self.thumb_pos_px,
                        height - side_margin - side_margin,
                    ),
                    Orientation::Vertical => rect(
                        x + side_margin,
                        y + start_margin,
                        width - side_margin - side_margin,
                        self.thumb_pos_px,
                    ),
                },
                background,
            );
        }

        match data.orientation {
            Orientation::Horizontal => default_theme::button(
                &mut drawing_context.display,
                x + self.thumb_pos_px + start_margin,
                y + side_margin,
                self.thumb_size_px,
                height - side_margin - side_margin,
                self.is_thumb_pressed.get(),
                self.is_thumb_hover.get(),
                false,
            ),
            Orientation::Vertical => default_theme::button(
                &mut drawing_context.display,
                x + side_margin,
                y + self.thumb_pos_px + start_margin,
                width - side_margin - side_margin,
                self.thumb_size_px,
                self.is_thumb_pressed.get(),
                self.is_thumb_hover.get(),
                false,
            ),
        };

        if self.thumb_pos_px + self.thumb_size_px < scroll_bar_size_px {
            drawing_context.display.draw_rect(
                match data.orientation {
                    Orientation::Horizontal => rect(
                        x + self.thumb_pos_px + self.thumb_size_px + start_margin,
                        y + side_margin,
                        scroll_bar_size_px - self.thumb_pos_px - self.thumb_size_px,
                        height - side_margin - side_margin,
                    ),
                    Orientation::Vertical => rect(
                        x + side_margin,
                        y + self.thumb_pos_px + self.thumb_size_px + start_margin,
                        width - side_margin - side_margin,
                        scroll_bar_size_px - self.thumb_pos_px - self.thumb_size_px,
                    ),
                },
                background,
            );
        }

        default_theme::border_3d_single(
            &mut drawing_context.display,
            x,
            y,
            width,
            height,
            true,
            false,
            false,
        );
    }
}
