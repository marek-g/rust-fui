use std::cell::RefCell;
use std::rc::Rc;

use drawing::primitive::Primitive;
use fui_core::*;
use fui_system_core::{CursorShape, Edge};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MoveResizeArea {
    /// resize border size (zero on any edge means no resize border on that edge)
    #[builder(default = Thickness::all(4.0f32))]
    pub border_size: Thickness,

    /// how wide in pixels is every corner (used for diagonal resizing)
    #[builder(default = 10.0f32)]
    pub corner_size: f32,

    /// is the whole area inside the border the move area
    #[builder(default = true)]
    pub has_move_area: bool,

    /// cursor shape to be used inside the move area
    #[builder(default = CursorShape::CrossCursor)]
    pub move_area_cursor: CursorShape,
}

impl MoveResizeArea {
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultMoveResizeAreaStyle::new(
                    DefaultMoveResizeAreaStyleParams::builder().build(),
                ))
            }),
            context,
        )
    }

    fn position_to_edge(&self, x: f32, y: f32, rect: Rect) -> Edge {
        let mut edge = 0i32;

        if x >= rect.x && x < rect.x + self.border_size.left {
            edge += Edge::Left.bits();
        } else if x >= rect.x + rect.width - self.border_size.right && x < rect.x + rect.width {
            edge += Edge::Right.bits();
        }

        if y >= rect.y && y < rect.y + self.border_size.top {
            edge += Edge::Top.bits();
        } else if y >= rect.y + rect.height - self.border_size.bottom && y < rect.y + rect.height {
            edge += Edge::Bottom.bits();
        }

        if (edge == Edge::Left.bits() || edge == Edge::Right.bits())
            && y < rect.y + self.corner_size
        {
            edge += Edge::Top.bits();
        }

        if (edge == Edge::Left.bits() || edge == Edge::Right.bits())
            && y > rect.y + rect.height - self.corner_size
        {
            edge += Edge::Bottom.bits();
        }

        if (edge == Edge::Top.bits() || edge == Edge::Bottom.bits())
            && x < rect.x + self.corner_size
        {
            edge += Edge::Left.bits();
        }

        if (edge == Edge::Top.bits() || edge == Edge::Bottom.bits())
            && x > rect.x + rect.width - self.corner_size
        {
            edge += Edge::Right.bits();
        }

        Edge::from_bits(edge).unwrap()
    }
}

//
// Default MoveResizeArea Style
//

#[derive(TypedBuilder)]
pub struct DefaultMoveResizeAreaStyleParams {}

pub struct DefaultMoveResizeAreaStyle {
    current_edge: Edge,
}

impl DefaultMoveResizeAreaStyle {
    pub fn new(_params: DefaultMoveResizeAreaStyleParams) -> Self {
        DefaultMoveResizeAreaStyle {
            current_edge: Edge::empty(),
        }
    }
}

impl Style<MoveResizeArea> for DefaultMoveResizeAreaStyle {
    fn setup(&mut self, _data: &mut MoveResizeArea, _control_context: &mut ControlContext) {}

    fn handle_event(
        &mut self,
        data: &mut MoveResizeArea,
        control_context: &mut ControlContext,
        _drawing_context: &mut dyn DrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { .. } => {
                // disable auto capturing this control as
                // start_sytem_move() and start_system_resize()
                // do not send TapUp events
                // (and otherwise this control stays captured forever)
                event_context.set_captured_control(None);

                let edge = self.current_edge;

                if edge.is_empty() && !data.has_move_area {
                    return;
                }

                // post start_system_move()
                // (cannot call it directly because services can be already borrowed)
                if let Some(services) = control_context.get_services() {
                    spawn_local_and_forget(async move {
                        let window_service = services.borrow_mut().get_window_service();
                        if let Some(window_service) = window_service {
                            if edge.is_empty() {
                                window_service.start_system_move();
                            } else {
                                window_service.start_system_resize(edge);
                            }
                        }
                    });
                }
            }

            ControlEvent::PointerMove { position } => {
                let rect = control_context.get_rect();
                self.current_edge = data.position_to_edge(position.x, position.y, rect);
                let cursor = match self.current_edge {
                    Edge::Left | Edge::Right => CursorShape::SizeHorCursor,
                    Edge::Top | Edge::Bottom => CursorShape::SizeVerCursor,
                    Edge::TopLeft | Edge::BottomRight => CursorShape::SizeFDiagCursor,
                    Edge::TopRight | Edge::BottomLeft => CursorShape::SizeBDiagCursor,
                    _ => {
                        if data.has_move_area {
                            data.move_area_cursor
                        } else {
                            CursorShape::ArrowCursor
                        }
                    }
                };

                // post set_cursor()
                // (cannot call it directly because services can be already borrowed)
                if let Some(services) = control_context.get_services() {
                    spawn_local_and_forget(async move {
                        let window_service = services.borrow_mut().get_window_service();
                        if let Some(window_service) = window_service {
                            window_service.set_cursor(cursor);
                        }
                    });
                }
            }

            ControlEvent::HitTestChange(is_hit_test) => {
                if !is_hit_test {
                    // post set_cursor()
                    // (cannot call it directly because services can be already borrowed)
                    if let Some(services) = control_context.get_services() {
                        spawn_local_and_forget(async move {
                            let window_service = services.borrow_mut().get_window_service();
                            if let Some(window_service) = window_service {
                                window_service.set_cursor(CursorShape::ArrowCursor);
                            }
                        });
                    }
                }
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &mut MoveResizeArea,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        size: Size,
    ) -> Size {
        let size = Margin::remove_thickness_from_size(size, data.border_size);

        let children = control_context.get_children();

        let size = if children.len() > 0 {
            let mut desired_size = Size::new(0.0f32, 0.0f32);

            for child in children.into_iter() {
                let mut child = child.borrow_mut();
                child.measure(drawing_context, size);
                let child_rc = child.get_rect();
                desired_size.width = desired_size.width.max(child_rc.width);
                desired_size.height = desired_size.height.max(child_rc.height);
            }

            desired_size
        } else {
            size
        };

        Margin::add_thickness_to_size(size, data.border_size)
    }

    fn set_rect(
        &mut self,
        data: &mut MoveResizeArea,
        control_context: &mut ControlContext,
        drawing_context: &mut dyn DrawingContext,
        rect: Rect,
    ) {
        let new_rect = Rect::new(
            rect.x + data.border_size.left,
            rect.y + data.border_size.top,
            rect.width - data.border_size.left - data.border_size.right,
            rect.height - data.border_size.top - data.border_size.bottom,
        );
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow_mut().set_rect(drawing_context, new_rect);
        }
    }

    fn hit_test(
        &self,
        _data: &MoveResizeArea,
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
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn to_primitives(
        &self,
        _data: &MoveResizeArea,
        control_context: &ControlContext,
        drawing_context: &mut dyn DrawingContext,
    ) -> (Vec<Primitive>, Vec<Primitive>) {
        let children = control_context.get_children();
        if let Some(child) = children.into_iter().next() {
            child.borrow().to_primitives(drawing_context)
        } else {
            (Vec::new(), Vec::new())
        }
    }
}
