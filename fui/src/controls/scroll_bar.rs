use std::cell::RefCell;
use std::rc::Rc;

use children_source::*;
use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::primitive_extensions::PrimitiveTransformations;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use observable::*;
use style::*;
use typed_builder::TypedBuilder;
use view::*;

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

impl View for ScrollBar {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(self, ScrollBarDefaultStyle::new(), context)
    }
}

//
// ScrollBar Default Style
//

pub struct ScrollBarDefaultStyle {
    rect: Rect,
    is_hover: Property<bool>,
    is_pressed: Property<bool>,
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
            is_hover: Property::new(false),
            is_pressed: Property::new(false),
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ScrollBar> for ScrollBarDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut ScrollBar,
        control: &Rc<RefCell<Control<ScrollBar>>>,
    ) {
        self.event_subscriptions
            .push(self.is_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_pressed.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollBar,
        children: &Box<dyn ChildrenSource>,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                self.is_pressed.set(true);
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &children, *position) {
                    //data.clicked.emit(());
                }
                self.is_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &children, *position) {
                    self.is_pressed.set(true);
                } else {
                    self.is_pressed.set(false);
                }
            }

            ControlEvent::HoverEnter => {
                self.is_hover.set(true);
            }

            ControlEvent::HoverLeave => {
                self.is_hover.set(false);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &ScrollBar,
        _children: &Box<dyn ChildrenSource>,
        _drawing_context: &mut DrawingContext,
        _size: Size,
    ) {
        match data.orientation {
            Orientation::Horizontal => {
                self.rect = Rect::new(0.0f32, 0.0f32, 150.0f32, 20.0f32);
            }
            Orientation::Vertical => {
                self.rect = Rect::new(0.0f32, 0.0f32, 20.0f32, 150.0f32);
            }
        }
    }

    fn set_rect(&mut self, _data: &ScrollBar, _children: &Box<dyn ChildrenSource>, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ScrollBar,
        _children: &Box<dyn ChildrenSource>,
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
        _children: &Box<dyn ChildrenSource>,
        _drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - x,
            Orientation::Vertical => height - y,
        };
        let scroll_bar_size_f32 =
            data.max_value.get() - data.min_value.get() + data.viewport_size.get();

        let thumb_size_px =
            ((data.viewport_size.get() * scroll_bar_size_px) / scroll_bar_size_f32).max(20.0f32);

        let thumb_pos_px = (scroll_bar_size_px - thumb_size_px)
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get());

        let background = [0.1, 0.5, 0.0, 0.2];
        let foreground = [0.1, 1.0, 0.0, 0.4];
        let line_color1 = if !self.is_pressed.get() {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        };
        let line_color2 = if !self.is_pressed.get() {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        let mut vec = Vec::new();
        if thumb_pos_px > 0.0f32 {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: UserPixelRect::new(
                    UserPixelPoint::new(x, y),
                    UserPixelSize::new(thumb_pos_px, height),
                ),
            });
        }

        vec.push(Primitive::Rectangle {
            color: foreground,
            rect: UserPixelRect::new(
                UserPixelPoint::new(x + thumb_pos_px, y),
                UserPixelSize::new(thumb_size_px, height),
            ),
        });

        if thumb_pos_px + thumb_size_px < scroll_bar_size_px {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: UserPixelRect::new(
                    UserPixelPoint::new(x + thumb_pos_px + thumb_size_px, y),
                    UserPixelSize::new(scroll_bar_size_px - thumb_pos_px - thumb_size_px, height),
                ),
            });
        }

        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        });

        vec
    }
}
