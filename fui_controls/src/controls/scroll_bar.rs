use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
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

impl Control for ScrollBar {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(self, Box::new(ScrollBarDefaultStyle::new()), context)
    }
}

//
// ScrollBar Default Style
//

const START_MARGIN: f32 = 1.0f32;
const END_MARGIN: f32 = 1.0f32;
const SIDE_MARGIN: f32 = 1.0f32;
const MIN_THUMB_SIZE: f32 = 20.0f32;
const MIN_SIZE: f32 = MIN_THUMB_SIZE * 2.0f32;

pub struct ScrollBarDefaultStyle {
    rect: Rect,
    thumb_pos_px: f32,
    thumb_size_px: f32,

    is_thumb_hover: Property<bool>,
    is_thumb_pressed: Property<bool>,
    pressed_offset: f32,

    event_subscriptions: Vec<EventSubscription>,
}

impl ScrollBarDefaultStyle {
    pub fn new() -> Self {
        ScrollBarDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            thumb_pos_px: 0f32,
            thumb_size_px: 0f32,
            is_thumb_hover: Property::new(false),
            is_thumb_pressed: Property::new(false),
            pressed_offset: 0.0f32,
            event_subscriptions: Vec::new(),
        }
    }

    fn calc_sizes(&mut self, data: &ScrollBar) {
        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => self.rect.width - START_MARGIN - END_MARGIN,
            Orientation::Vertical => self.rect.height - START_MARGIN - END_MARGIN,
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

impl Style<ScrollBar> for ScrollBarDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ScrollBar,
        control: &Rc<RefCell<StyledControl<ScrollBar>>>,
    ) {
        self.event_subscriptions
            .push(self.is_thumb_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_thumb_pressed.dirty_watching(control));

        self.event_subscriptions
            .push(data.min_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.max_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.value.dirty_watching(control));
        self.event_subscriptions
            .push(data.viewport_size.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollBar,
        _context: &mut ControlContext,
        _resources: &mut dyn Resources,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { position } => {
                let pos = match data.orientation {
                    Orientation::Horizontal => position.x - self.rect.x - START_MARGIN,
                    Orientation::Vertical => position.y - self.rect.y - START_MARGIN,
                };
                if pos >= self.thumb_pos_px && pos < self.thumb_pos_px + self.thumb_size_px {
                    self.is_thumb_pressed.set(true);
                    self.pressed_offset = pos - self.thumb_pos_px;
                }
            }

            ControlEvent::TapUp { ref position } => {
                self.is_thumb_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if self.is_thumb_pressed.get() {
                    let scroll_bar_size_px = match data.orientation {
                        Orientation::Horizontal => self.rect.width - START_MARGIN - END_MARGIN,
                        Orientation::Vertical => self.rect.height - START_MARGIN - END_MARGIN,
                    };

                    let pos = match data.orientation {
                        Orientation::Horizontal => position.x - self.rect.x - START_MARGIN,
                        Orientation::Vertical => position.y - self.rect.y - START_MARGIN,
                    };

                    let new_thumb_pos_px = pos - self.pressed_offset;
                    let new_value = (data.min_value.get()
                        + new_thumb_pos_px * (data.max_value.get() - data.min_value.get())
                            / (scroll_bar_size_px - self.thumb_size_px))
                        .max(data.min_value.get())
                        .min(data.max_value.get());

                    if new_value != data.value.get() {
                        //println!("New value: {}", new_value);
                        data.value.set(new_value);
                    }
                }
            }

            ControlEvent::HoverEnter => {
                self.is_thumb_hover.set(true);
            }

            ControlEvent::HoverLeave => {
                self.is_thumb_hover.set(false);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut ScrollBar,
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

    fn set_rect(&mut self, data: &mut ScrollBar, _context: &mut ControlContext, rect: Rect) {
        self.rect = rect;
        self.calc_sizes(data);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ScrollBar,
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
        data: &ScrollBar,
        _context: &ControlContext,
        _resources: &mut dyn Resources,
    ) -> Vec<Primitive> {
        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

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

        vec
    }
}
