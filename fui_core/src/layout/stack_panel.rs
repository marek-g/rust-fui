use std::cell::RefCell;
use std::f32;
use std::rc::Rc;

use crate::{
    Alignment, ControlContext, ControlEvent, ControlObject, DrawingContext, EventContext,
    HorizontalAlignment, Orientation, Point, Rect, Size, Style, StyledControl, VerticalAlignment,
    ViewContext,
};
use drawing::primitive::Primitive;
use typed_builder::TypedBuilder;

use super::Length;

//
// Attached values
//

pub struct Grow;
impl typemap::Key for Grow {
    type Value = Length;
}

//
// StackPanel
//

#[derive(TypedBuilder)]
pub struct StackPanel {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,
}

impl StackPanel {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        mut context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        // set default alignment to Start
        context
            .attached_values
            .entry::<HorizontalAlignment>()
            .or_insert(Alignment::Start);
        context
            .attached_values
            .entry::<VerticalAlignment>()
            .or_insert(Alignment::Start);

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

pub struct DefaultStackPanelStyle;

impl DefaultStackPanelStyle {
    pub fn new(_params: DefaultStackPanelStyleParams) -> Self {
        DefaultStackPanelStyle {}
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
    ) -> Size {
        let mut result = Size::new(0f32, 0f32);

        let children = control_context.get_children();

        match data.orientation {
            Orientation::Horizontal => {
                let available_size = Size::new(f32::INFINITY, size.height);

                // calculate min sizes
                let mut grow_fills_sum = 0.0f32;
                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child = child.borrow();
                    let mut child_size = child.get_rect();

                    let map = child.get_context().get_attached_values();
                    if let Some(grow) = map.get::<Grow>() {
                        match *grow {
                            Length::Exact(l) => {
                                child_size.width = child_size.width.max(l);
                            }
                            Length::Fill(f) => {
                                grow_fills_sum += f;
                            }
                            _ => (),
                        }
                    }

                    result.width += child_size.width;
                    result.height = result.height.max(child_size.height);
                }

                // distribute size that's left over Grow = Length::Fill children
                if size.width.is_finite() && size.width > result.width && grow_fills_sum > 0.0f32 {
                    result.width = size.width;
                }
            }
            Orientation::Vertical => {
                let available_size = Size::new(size.width, f32::INFINITY);

                // calculate min sizes
                let mut grow_fills_sum = 0.0f32;
                for child in children.into_iter() {
                    child.borrow_mut().measure(drawing_context, available_size);
                    let child = child.borrow();
                    let mut child_size = child.get_rect();

                    let map = child.get_context().get_attached_values();
                    if let Some(grow) = map.get::<Grow>() {
                        match *grow {
                            Length::Exact(l) => {
                                child_size.height = child_size.height.max(l);
                            }
                            Length::Fill(f) => {
                                grow_fills_sum += f;
                            }
                            _ => (),
                        }
                    }

                    result.width = result.width.max(child_size.width);
                    result.height += child_size.height;
                }

                // distribute size that's left over Grow = Length::Fill children
                if size.height.is_finite() && size.height > result.height && grow_fills_sum > 0.0f32
                {
                    result.height = size.height;
                }
            }
        }

        result
    }

    fn set_rect(
        &mut self,
        data: &mut StackPanel,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        let mut child_rect = rect;

        let children = control_context.get_children();

        let grow_fills_sum: f32 = children
            .into_iter()
            .map(|child| {
                if let Some(Length::Fill(fill)) = child
                    .borrow()
                    .get_context()
                    .get_attached_values()
                    .get::<Grow>()
                {
                    *fill
                } else {
                    0.0f32
                }
            })
            .sum();

        match data.orientation {
            Orientation::Horizontal => {
                let child_sizes_sum: f32 = children
                    .into_iter()
                    .map(|child| {
                        let child = child.borrow();
                        let child_width = child.get_rect().width;
                        let map = child.get_context().get_attached_values();
                        if let Some(grow) = map.get::<Grow>() {
                            match *grow {
                                Length::Exact(l) => child_width.max(l),
                                _ => child_width,
                            }
                        } else {
                            child_width
                        }
                    })
                    .sum();

                let fill_coefficient =
                    if grow_fills_sum > 0.0f32 && rect.width - child_sizes_sum > 0.0f32 {
                        (rect.width - child_sizes_sum) / grow_fills_sum
                    } else {
                        0.0f32
                    };

                for child in children.into_iter() {
                    {
                        let child = child.borrow();
                        let mut child_size = child.get_rect();

                        let map = child.get_context().get_attached_values();
                        if let Some(grow) = map.get::<Grow>() {
                            match *grow {
                                Length::Exact(l) => {
                                    child_size.width = child_size.width.max(l);
                                }
                                Length::Fill(f) => {
                                    if fill_coefficient > 0.0f32 {
                                        child_size.width += f * fill_coefficient;
                                    }
                                }
                                _ => (),
                            }
                        }

                        child_rect.width = child_size.width;
                        child_rect.height = child_size.height;
                    }

                    let dest_rect =
                        Rect::new(child_rect.x, child_rect.y, child_rect.width, rect.height);

                    let mut child = child.borrow_mut();
                    child.set_rect(drawing_context, dest_rect);

                    child_rect.x += child_rect.width;
                }
            }
            Orientation::Vertical => {
                let child_sizes_sum: f32 = children
                    .into_iter()
                    .map(|child| {
                        let child = child.borrow();
                        let child_height = child.get_rect().height;
                        let map = child.get_context().get_attached_values();
                        if let Some(grow) = map.get::<Grow>() {
                            match *grow {
                                Length::Exact(l) => child_height.max(l),
                                _ => child_height,
                            }
                        } else {
                            child_height
                        }
                    })
                    .sum();

                let fill_coefficient =
                    if grow_fills_sum > 0.0f32 && rect.height - child_sizes_sum > 0.0f32 {
                        (rect.height - child_sizes_sum) / grow_fills_sum
                    } else {
                        0.0f32
                    };

                for child in children.into_iter() {
                    {
                        let child = child.borrow();
                        let mut child_size = child.get_rect();

                        let map = child.get_context().get_attached_values();
                        if let Some(grow) = map.get::<Grow>() {
                            match *grow {
                                Length::Exact(l) => {
                                    child_size.height = child_size.height.max(l);
                                }
                                Length::Fill(f) => {
                                    if fill_coefficient > 0.0f32 {
                                        child_size.height += f * fill_coefficient;
                                    }
                                }
                                _ => (),
                            }
                        }

                        child_rect.width = child_size.width;
                        child_rect.height = child_size.height;
                    }

                    let dest_rect =
                        Rect::new(child_rect.x, child_rect.y, rect.width, child_rect.height);

                    let mut child = child.borrow_mut();
                    child.set_rect(drawing_context, dest_rect);

                    child_rect.y += child_rect.height;
                }
            }
        }
    }

    fn hit_test(
        &self,
        _data: &StackPanel,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if point.is_inside(&control_context.get_rect()) {
            let children = control_context.get_children();
            for child in children.into_iter() {
                let c = child.borrow();
                let rect = c.get_rect();
                if point.is_inside(&rect) {
                    let hit_control = c.hit_test(point);
                    if hit_control.is_some() {
                        return hit_control;
                    }
                }
            }
            None
        } else {
            None
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
