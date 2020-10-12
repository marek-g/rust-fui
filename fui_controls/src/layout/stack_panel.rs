use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use crate::Alignment;
use drawing::primitive::Primitive;
use fui_core::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct StackPanel {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,
}

impl StackPanel {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultStackPanelStyle::new(
                    DefaultStackPanelStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }
}

//
// Default StackPanel Style
//

#[derive(TypedBuilder)]
pub struct DefaultStackPanelStyleParams {}

pub struct DefaultStackPanelStyle {
    rect: Rect,
}

impl DefaultStackPanelStyle {
    pub fn new(_params: DefaultStackPanelStyleParams) -> Self {
        DefaultStackPanelStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
        }
    }
}

impl Style<StackPanel> for DefaultStackPanelStyle {
    fn setup(&mut self, _data: &mut StackPanel, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        _data: &mut StackPanel,
        _control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut StackPanel,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) {
        let mut result = Rect::new(0.0f32, 0.0f32, 0f32, 0f32);

        let children = control_context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                let available_size = Size::new(f32::INFINITY, size.height);

                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width += child_size.width;
                    result.height = result.height.max(child_size.height);
                }
            }
            Orientation::Vertical => {
                let available_size = Size::new(size.width, f32::INFINITY);

                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child_size = child.borrow().get_rect();
                    result.width = result.width.max(child_size.width);
                    result.height += child_size.height;
                }
            }
        }

        self.rect = result;
    }

    fn set_rect(
        &mut self,
        data: &mut StackPanel,
        control_context: &mut ControlContext,
        rect: Rect,
    ) {
        let map = control_context.get_attached_values();
        Alignment::apply(
            &mut self.rect,
            rect,
            &map,
            Alignment::Start,
            Alignment::Start,
        );

        let mut child_rect = self.rect;

        let children = control_context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                for child in children.into_iter() {
                    let mut child = child.borrow_mut();

                    let child_size = child.get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;

                    let dest_rect = Rect::new(
                        child_rect.x,
                        child_rect.y,
                        child_rect.width,
                        self.rect.height,
                    );
                    child.set_rect(dest_rect);

                    child_rect.x += child_rect.width;
                }
            }
            Orientation::Vertical => {
                for child in children.into_iter() {
                    let mut child = child.borrow_mut();

                    let child_size = child.get_rect();
                    child_rect.width = child_size.width;
                    child_rect.height = child_size.height;

                    let dest_rect = Rect::new(
                        child_rect.x,
                        child_rect.y,
                        self.rect.width,
                        child_rect.height,
                    );
                    child.set_rect(dest_rect);

                    child_rect.y += child_rect.height;
                }
            }
        }
    }

    fn get_rect(&self, _control_context: &ControlContext) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &StackPanel,
        control_context: &ControlContext,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            let children = control_context.get_children();
            for child in children.into_iter() {
                let c = child.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let child_hit_test = c.hit_test(point);
                    match child_hit_test {
                        HitTestResult::Current => return HitTestResult::Child(child.clone()),
                        HitTestResult::Child(..) => return child_hit_test,
                        HitTestResult::Nothing => (),
                    }
                }
            }
            HitTestResult::Nothing
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        _data: &StackPanel,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let mut vec = Vec::new();
        let mut overlay = Vec::new();

        let children = control_context.get_children();
        for child in children.into_iter() {
            let (mut vec2, mut overlay2) = child.borrow().to_primitives(drawing_context);
            vec.append(&mut vec2);
            overlay.append(&mut overlay2);
        }

        (vec, overlay)
    }
}
