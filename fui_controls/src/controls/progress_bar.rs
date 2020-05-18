use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
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

impl Control for ProgressBar {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self,
            style.unwrap_or_else(|| {
                Box::new(DefaultProgressBarStyle::new(DefaultProgressBarStyleParams::builder().build()))
            }),
            context)
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
    rect: Rect,
    event_subscriptions: Vec<EventSubscription>,
}

impl DefaultProgressBarStyle {
    pub fn new(_params: DefaultProgressBarStyleParams) -> Self {
        DefaultProgressBarStyle {
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

impl Style<ProgressBar> for DefaultProgressBarStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ProgressBar,
        control: &Rc<RefCell<StyledControl<ProgressBar>>>,
    ) {
        self.event_subscriptions
            .push(data.min_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.max_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.value.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ProgressBar,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut ProgressBar,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        size: Size,
    ) {
        match data.orientation {
            Orientation::Horizontal => {
                let space = if size.width.is_infinite() {
                    MIN_SIZE
                } else {
                    size.width
                };
                self.rect = Rect::new(0.0f32, 0.0f32, MIN_SIZE.max(space), 20.0f32);
            }
            Orientation::Vertical => {
                let space = if size.height.is_infinite() {
                    MIN_SIZE
                } else {
                    size.height
                };
                self.rect = Rect::new(0.0f32, 0.0f32, 20.0f32, MIN_SIZE.max(space));
            }
        }
    }

    fn set_rect(&mut self, data: &mut ProgressBar, _context: &mut ControlContext, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self, _context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ProgressBar,
        _context: &ControlContext,
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
        data: &ProgressBar,
        _context: &ControlContext,
        _resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

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

        vec
    }
}
