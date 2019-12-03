use layout::Grid;
use std::cell::RefCell;
use std::rc::Rc;

use children_source::*;
use common::*;
use control::*;
use control_object::*;
use controls::scroll_bar::ScrollBar;
use drawing::primitive::Primitive;
use drawing::primitive_extensions::PrimitiveTransformations;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use fui_macros::ui;
use layout::*;
use observable::*;
use style::*;
use typed_builder::TypedBuilder;
use typemap::TypeMap;
use view::*;

pub enum ScrollBarVisibility {
    Disabled,
    Auto,
    Hidden,
    Visible,
}

#[derive(TypedBuilder)]
pub struct ScrollViewer {
    #[builder(default = ScrollBarVisibility::Auto)]
    pub horizontal_scroll_bar_visibility: ScrollBarVisibility,

    #[builder(default = ScrollBarVisibility::Auto)]
    pub vertical_scroll_bar_visibility: ScrollBarVisibility,

    #[builder(default = Property::new(0.0f32))]
    pub offset_x: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub offset_y: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub max_offset_x: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub max_offset_y: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub viewport_width: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub viewport_height: Property<f32>,
}

impl View for ScrollViewer {
    fn to_view(mut self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let offset_x_prop = Property::binded_two_way(&mut self.offset_x);
        let offset_y_prop = Property::binded_two_way(&mut self.offset_y);
        let max_offset_x_prop = Property::binded_from(&self.max_offset_x);
        let max_offset_y_prop = Property::binded_from(&self.max_offset_y);
        let viewport_width_prop = Property::binded_from(&self.viewport_width);
        let viewport_height_prop = Property::binded_from(&self.viewport_height);

        let content = Control::new(self, ScrollViewerDefaultStyle::new(), context);

        ui! {
            Grid {
                columns: 2,

                widths: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],
                heights: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],

                @content,

                ScrollBar {
                    orientation: Orientation::Vertical,
                    value: offset_y_prop,
                    max_value: max_offset_y_prop,
                    viewport_size: viewport_height_prop,
                },

                ScrollBar {
                    orientation: Orientation::Horizontal,
                    value: offset_x_prop,
                    max_value: max_offset_x_prop,
                    viewport_size: viewport_width_prop,
                },
            }
        }
    }
}

//
// ScrollViewer Default Style
//

pub struct ScrollViewerDefaultStyle {
    rect: Rect,
    content_size: Size,
    event_subscriptions: Vec<EventSubscription>,
}

impl ScrollViewerDefaultStyle {
    pub fn new() -> Self {
        ScrollViewerDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            content_size: Size::new(0.0f32, 0.0f32),
            event_subscriptions: Vec::new(),
        }
    }

    fn update_scrollbars(&self, data: &mut ScrollViewer) {
        data.viewport_width.set(self.rect.width);
        data.viewport_height.set(self.rect.height);
        data.max_offset_x
            .set((self.content_size.width - self.rect.width).max(0.0f32));
        data.max_offset_y
            .set((self.content_size.height - self.rect.height).max(0.0f32));
        if data.offset_x.get() > data.max_offset_x.get() {
            data.offset_x.set(data.max_offset_x.get())
        }
        if data.offset_y.get() > data.max_offset_y.get() {
            data.offset_y.set(data.max_offset_y.get())
        }
    }
}

impl Style<ScrollViewer> for ScrollViewerDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut ScrollViewer,
        _control: &Rc<RefCell<Control<ScrollViewer>>>,
    ) {
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollViewer,
        children: &Box<dyn ChildrenSource>,
        event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut ScrollViewer,
        children: &Box<dyn ChildrenSource>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) {
        self.content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        self.rect = Rect::new(
            0.0f32,
            0.0f32,
            self.content_size.width.min(size.width),
            self.content_size.height.min(size.height),
        );
    }

    fn set_rect(
        &mut self,
        data: &mut ScrollViewer,
        children: &Box<dyn ChildrenSource>,
        rect: Rect,
    ) {
        self.rect = rect;

        self.update_scrollbars(data);

        if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().set_rect(rect);
        }
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ScrollViewer,
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
        data: &ScrollViewer,
        children: &Box<dyn ChildrenSource>,
        drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        default_theme::button(&mut vec, x, y, width, height, true, false);

        if let Some(ref content) = children.into_iter().next() {
            let mut vec2 = content.borrow_mut().to_primitives(drawing_context);

            vec2.translate(UserPixelPoint::new(
                -data.offset_x.get().round(),
                -data.offset_y.get().round(),
            ));

            let mut vec2 = vec2.clip(UserPixelRect::new(
                UserPixelPoint::new(x, y),
                UserPixelSize::new(width, height),
            ));

            vec.append(&mut vec2);
        }

        vec
    }
}
