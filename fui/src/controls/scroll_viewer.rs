use controls::scroll_bar::ScrollBar;
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
}

impl View for ScrollViewer {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        Control::new(self, ScrollViewerDefaultStyle::new(), context)
    }
}

//
// ScrollViewer Default Style
//

pub struct ScrollViewerDefaultStyle {
    rect: Rect,
    offset_x: f32,
    offset_y: f32,
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
            offset_x: 50.0f32,
            offset_y: 50.0f32,
            event_subscriptions: Vec::new(),
        }
    }
}

impl Style<ScrollViewer> for ScrollViewerDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        _data: &mut ScrollViewer,
        control: &Rc<RefCell<Control<ScrollViewer>>>,
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
        _data: &ScrollViewer,
        children: &Box<dyn ChildrenSource>,
        drawing_context: &mut DrawingContext,
        size: Size,
    ) {
        println!("measure: {:?}", size);

        let content_size = if let Some(ref content) = children.into_iter().next() {
            content.borrow_mut().measure(drawing_context, size);
            let rect = content.borrow().get_rect();
            Size::new(rect.width, rect.height)
        } else {
            Size::new(0f32, 0f32)
        };

        //self.rect = Rect::new(0.0f32, 0.0f32, content_size.width, content_size.height);
        self.rect = Rect::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    }

    fn set_rect(&mut self, _data: &ScrollViewer, children: &Box<dyn ChildrenSource>, rect: Rect) {
        self.rect = rect;

        println!("set_rect: {:?}", rect);

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
        _data: &ScrollViewer,
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

            vec2.translate(UserPixelPoint::new(self.offset_x, self.offset_y));

            vec.append(&mut vec2);
        }

        vec
    }
}
