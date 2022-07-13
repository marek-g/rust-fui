use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use fui_core::*;
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

    #[builder(default = Property::new(0.0f32))]
    pub viewport_size: Property<f32>,
}

impl ScrollBar {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Children {
        Children::SingleStatic(StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultScrollBarStyle::new(
                    DefaultScrollBarStyleParams::builder().build(),
                ))
            }),
            context,
        ))
    }
}

//
// Default ScrollBar Style
//

const START_MARGIN: f32 = 1.0f32;
const END_MARGIN: f32 = 1.0f32;
const SIDE_MARGIN: f32 = 1.0f32;
const MIN_THUMB_SIZE: f32 = 20.0f32;
const MIN_SIZE: f32 = MIN_THUMB_SIZE * 2.0f32;

#[derive(TypedBuilder)]
pub struct DefaultScrollBarStyleParams {}

pub struct DefaultScrollBarStyle {
    thumb_pos_px: f32,
    thumb_size_px: f32,

    is_thumb_hover: Property<bool>,
    is_thumb_pressed: Property<bool>,
    pressed_offset: f32,

    event_subscriptions: Vec<Subscription>,
}

impl DefaultScrollBarStyle {
    pub fn new(_params: DefaultScrollBarStyleParams) -> Self {
        DefaultScrollBarStyle {
            thumb_pos_px: 0f32,
            thumb_size_px: 0f32,
            is_thumb_hover: Property::new(false),
            is_thumb_pressed: Property::new(false),
            pressed_offset: 0.0f32,
            event_subscriptions: Vec::new(),
        }
    }

    fn calc_sizes(&mut self, data: &ScrollBar, rect: Rect) {
        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => rect.width - START_MARGIN - END_MARGIN,
            Orientation::Vertical => rect.height - START_MARGIN - END_MARGIN,
        };
        let scroll_bar_size_f32 =
            data.max_value.get() - data.min_value.get() + data.viewport_size.get();

        self.thumb_size_px = ((data.viewport_size.get() * scroll_bar_size_px)
            / scroll_bar_size_f32)
            .round()
            .max(MIN_THUMB_SIZE);

        self.thumb_pos_px = ((scroll_bar_size_px - self.thumb_size_px)
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get()))
        .round();
    }
}

impl Style<ScrollBar> for DefaultScrollBarStyle {
    fn setup(&mut self, data: &mut ScrollBar, control_context: &mut ControlContext) {
        self.event_subscriptions.push(
            self.is_thumb_hover
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            self.is_thumb_pressed
                .dirty_watching(&control_context.get_self_rc()),
        );

        self.event_subscriptions.push(
            data.min_value
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions.push(
            data.max_value
                .dirty_watching(&control_context.get_self_rc()),
        );
        self.event_subscriptions
            .push(data.value.dirty_watching(&control_context.get_self_rc()));
        self.event_subscriptions.push(
            data.viewport_size
                .dirty_watching(&control_context.get_self_rc()),
        );
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollBar,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { position } => {
                let rect = control_context.get_rect();
                let pos = match data.orientation {
                    Orientation::Horizontal => position.x - rect.x - START_MARGIN,
                    Orientation::Vertical => position.y - rect.y - START_MARGIN,
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

                    let scroll_bar_size_px = match data.orientation {
                        Orientation::Horizontal => rect.width - START_MARGIN - END_MARGIN,
                        Orientation::Vertical => rect.height - START_MARGIN - END_MARGIN,
                    };

                    let pos = match data.orientation {
                        Orientation::Horizontal => position.x - rect.x - START_MARGIN,
                        Orientation::Vertical => position.y - rect.y - START_MARGIN,
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

            ControlEvent::HoverChange(value) => {
                self.is_thumb_hover.set(value);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut ScrollBar,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        match data.orientation {
            Orientation::Horizontal => {
                let space = if size.width.is_infinite() {
                    MIN_SIZE
                } else {
                    size.width
                };
                Size::new(MIN_SIZE.max(space), 20.0f32)
            }
            Orientation::Vertical => {
                let space = if size.height.is_infinite() {
                    MIN_SIZE
                } else {
                    size.height
                };
                Size::new(20.0f32, MIN_SIZE.max(space))
            }
        }
    }

    fn set_rect(
        &mut self,
        data: &mut ScrollBar,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        self.calc_sizes(data, rect);
    }

    fn hit_test(
        &self,
        _data: &ScrollBar,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn to_primitives(
        &self,
        data: &ScrollBar,
        control_context: &ControlContext,
        _drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - START_MARGIN - END_MARGIN,
            Orientation::Vertical => height - START_MARGIN - END_MARGIN,
        };

        let background = [0.1, 0.5, 0.0, 0.2];

        let mut vec = Vec::new();
        if self.thumb_pos_px > 0.0f32 {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: match data.orientation {
                    Orientation::Horizontal => PixelRect::new(
                        PixelPoint::new(x + START_MARGIN, y + SIDE_MARGIN),
                        PixelSize::new(self.thumb_pos_px, height - SIDE_MARGIN - SIDE_MARGIN),
                    ),
                    Orientation::Vertical => PixelRect::new(
                        PixelPoint::new(x + SIDE_MARGIN, y + START_MARGIN),
                        PixelSize::new(width - SIDE_MARGIN - SIDE_MARGIN, self.thumb_pos_px),
                    ),
                },
            });
        }

        match data.orientation {
            Orientation::Horizontal => default_theme::button(
                &mut vec,
                x + self.thumb_pos_px + START_MARGIN,
                y + SIDE_MARGIN,
                self.thumb_size_px,
                height - SIDE_MARGIN - SIDE_MARGIN,
                self.is_thumb_pressed.get(),
                self.is_thumb_hover.get(),
                false,
            ),
            Orientation::Vertical => default_theme::button(
                &mut vec,
                x + SIDE_MARGIN,
                y + self.thumb_pos_px + START_MARGIN,
                width - SIDE_MARGIN - SIDE_MARGIN,
                self.thumb_size_px,
                self.is_thumb_pressed.get(),
                self.is_thumb_hover.get(),
                false,
            ),
        };

        if self.thumb_pos_px + self.thumb_size_px < scroll_bar_size_px {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: match data.orientation {
                    Orientation::Horizontal => PixelRect::new(
                        PixelPoint::new(
                            x + self.thumb_pos_px + self.thumb_size_px + START_MARGIN,
                            y + SIDE_MARGIN,
                        ),
                        PixelSize::new(
                            scroll_bar_size_px - self.thumb_pos_px - self.thumb_size_px,
                            height - SIDE_MARGIN - SIDE_MARGIN,
                        ),
                    ),
                    Orientation::Vertical => PixelRect::new(
                        PixelPoint::new(
                            x + SIDE_MARGIN,
                            y + self.thumb_pos_px + self.thumb_size_px + START_MARGIN,
                        ),
                        PixelSize::new(
                            width - SIDE_MARGIN - SIDE_MARGIN,
                            scroll_bar_size_px - self.thumb_pos_px - self.thumb_size_px,
                        ),
                    ),
                },
            });
        }

        default_theme::border_3d_single(&mut vec, x, y, width, height, true, false, false);

        (vec, Vec::new())
    }
}
