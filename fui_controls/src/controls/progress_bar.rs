use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize};
use fui_core::*;
use typed_builder::TypedBuilder;

use crate::style::*;

#[derive(TypedBuilder)]
pub struct ProgressBar {
    #[builder(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    #[builder(default = Property::new(0.0f32))]
    pub min_value: Property<f32>,

    #[builder(default = Property::new(1.0f32))]
    pub max_value: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub value: Property<f32>,
}

impl ProgressBar {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultProgressBarStyle::new(
                    DefaultProgressBarStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default ProgressBar Style
//

const START_MARGIN: f32 = 1.0f32;
const END_MARGIN: f32 = 1.0f32;
const SIDE_MARGIN: f32 = 1.0f32;
const MIN_SIZE: f32 = 22.0f32;

#[derive(TypedBuilder)]
pub struct DefaultProgressBarStyleParams {}

pub struct DefaultProgressBarStyle {
    event_subscriptions: Vec<Subscription>,
}

impl DefaultProgressBarStyle {
    pub fn new(_params: DefaultProgressBarStyleParams) -> Self {
        DefaultProgressBarStyle {
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ProgressBar> for DefaultProgressBarStyle {
    fn setup(&mut self, data: &mut ProgressBar, control_context: &mut ControlContext) {
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
    }

    fn handle_event(
        &mut self,
        _data: &mut ProgressBar,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut ProgressBar,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _size: Size,
    ) -> Size {
        match data.orientation {
            Orientation::Horizontal => Size::new(MIN_SIZE, 20.0f32),
            Orientation::Vertical => Size::new(20.0f32, MIN_SIZE),
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut ProgressBar,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &ProgressBar,
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
        data: &ProgressBar,
        control_context: &ControlContext,
        _drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let rect = control_context.get_rect();
        let x = rect.x;
        let y = rect.y;
        let width = rect.width;
        let height = rect.height;

        let progress_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - START_MARGIN - END_MARGIN,
            Orientation::Vertical => height - START_MARGIN - END_MARGIN,
        };

        let progress_bar_pos_px = (progress_bar_size_px
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get()))
        .round();

        let foreground = [1.0, 1.0, 0.0, 0.7];
        let background = [0.1, 0.5, 0.0, 0.2];

        let mut vec = Vec::new();

        default_theme::border_3d_single(&mut vec, x, y, width, height, true, false, false);

        match data.orientation {
            Orientation::Horizontal => {
                let background_size = width - START_MARGIN - END_MARGIN - progress_bar_pos_px;

                if progress_bar_pos_px > 0.0f32 {
                    vec.push(Primitive::Rectangle {
                        color: foreground,
                        rect: PixelRect::new(
                            PixelPoint::new(x + START_MARGIN, y + SIDE_MARGIN),
                            PixelSize::new(progress_bar_pos_px, height - SIDE_MARGIN - SIDE_MARGIN),
                        ),
                    });
                }

                if background_size > 0.0f32 {
                    vec.push(Primitive::Rectangle {
                        color: background,
                        rect: PixelRect::new(
                            PixelPoint::new(
                                x + START_MARGIN + progress_bar_pos_px,
                                y + SIDE_MARGIN,
                            ),
                            PixelSize::new(background_size, height - SIDE_MARGIN - SIDE_MARGIN),
                        ),
                    });
                }
            }

            Orientation::Vertical => {
                let background_size = height - START_MARGIN - END_MARGIN - progress_bar_pos_px;

                if progress_bar_pos_px > 0.0f32 {
                    vec.push(Primitive::Rectangle {
                        color: foreground,
                        rect: PixelRect::new(
                            PixelPoint::new(x + SIDE_MARGIN, y + START_MARGIN + background_size),
                            PixelSize::new(width - SIDE_MARGIN - SIDE_MARGIN, progress_bar_pos_px),
                        ),
                    });
                }

                if background_size > 0.0f32 {
                    vec.push(Primitive::Rectangle {
                        color: background,
                        rect: PixelRect::new(
                            PixelPoint::new(x + SIDE_MARGIN, y + START_MARGIN),
                            PixelSize::new(width - SIDE_MARGIN - SIDE_MARGIN, background_size),
                        ),
                    });
                }
            }
        }

        (vec, Vec::new())
    }
}
