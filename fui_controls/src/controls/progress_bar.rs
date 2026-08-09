use std::rc::Rc;

use fui_core::*;
use fui_drawing::prelude::*;
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
    ) -> Rc<dyn ControlObject> {
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



#[derive(TypedBuilder)]
pub struct DefaultProgressBarStyleParams {}

pub struct DefaultProgressBarStyle;

impl DefaultProgressBarStyle {
    pub fn new(_params: DefaultProgressBarStyleParams) -> Self {
        DefaultProgressBarStyle {}
    }
}

impl Style<ProgressBar> for DefaultProgressBarStyle {
    fn setup(&mut self, data: &mut ProgressBar, control_context: &ControlContext) {
        control_context.dirty_watch_property(&data.min_value);
        control_context.dirty_watch_property(&data.max_value);
        control_context.dirty_watch_property(&data.value);
    }

    fn handle_event(
        &mut self,
        _data: &mut ProgressBar,
        _control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _event_context: &mut dyn EventContext,
        _event: ControlEvent,
    ) {
    }

    fn measure(
        &mut self,
        data: &mut ProgressBar,
        control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _size: Size,
    ) -> Size {
        let min_size = control_context
            .get_inherited_value::<ProgressBarMinSize>()
            .map(|p| p.get())
            .unwrap_or(default_theme::PROGRESS_BAR_MIN_SIZE);
        match data.orientation {
            Orientation::Horizontal => Size::new(min_size, 20.0f32),
            Orientation::Vertical => Size::new(20.0f32, min_size),
        }
    }

    fn set_rect(
        &mut self,
        _data: &mut ProgressBar,
        _control_context: &ControlContext,
        _drawing_context: &mut FuiDrawingContext,
        _rect: Rect,
    ) {
    }

    fn hit_test(
        &self,
        _data: &ProgressBar,
        control_context: &ControlContext,
        point: Point,
    ) -> Option<Rc<dyn ControlObject>> {
        if point.is_inside(&control_context.get_rect()) {
            Some(control_context.get_self_rc())
        } else {
            None
        }
    }

    fn draw(
        &mut self,
        data: &ProgressBar,
        control_context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        let r = control_context.get_rect();
        let x = r.x;
        let y = r.y;
        let width = r.width;
        let height = r.height;

        let start_margin = control_context
            .get_inherited_value::<ProgressBarStartMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::PROGRESS_BAR_START_MARGIN);
        let end_margin = control_context
            .get_inherited_value::<ProgressBarEndMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::PROGRESS_BAR_END_MARGIN);
        let side_margin = control_context
            .get_inherited_value::<ProgressBarSideMargin>()
            .map(|p| p.get())
            .unwrap_or(default_theme::PROGRESS_BAR_SIDE_MARGIN);

        let progress_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - start_margin - end_margin,
            Orientation::Vertical => height - start_margin - end_margin,
        };

        let progress_bar_pos_px = (progress_bar_size_px
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get()))
        .round();

        let foreground = default_theme::PROGRESS_BAR_FOREGROUND;
        let background = default_theme::PROGRESS_BAR_BACKGROUND;

        default_theme::border_3d_single(
            &mut drawing_context.display,
            x,
            y,
            width,
            height,
            true,
            false,
            false,
        );

        match data.orientation {
            Orientation::Horizontal => {
                let background_size = width - start_margin - end_margin - progress_bar_pos_px;

                if progress_bar_pos_px > 0.0f32 {
                    drawing_context.display.draw_rect(
                        rect(
                            x + start_margin,
                            y + side_margin,
                            progress_bar_pos_px,
                            height - side_margin - side_margin,
                        ),
                        foreground,
                    );
                }

                if background_size > 0.0f32 {
                    drawing_context.display.draw_rect(
                        rect(
                            x + start_margin + progress_bar_pos_px,
                            y + side_margin,
                            background_size,
                            height - side_margin - side_margin,
                        ),
                        background,
                    );
                }
            }

            Orientation::Vertical => {
                let background_size = height - start_margin - end_margin - progress_bar_pos_px;

                if progress_bar_pos_px > 0.0f32 {
                    drawing_context.display.draw_rect(
                        rect(
                            x + side_margin,
                            y + start_margin + background_size,
                            width - side_margin - side_margin,
                            progress_bar_pos_px,
                        ),
                        foreground,
                    );
                }

                if background_size > 0.0f32 {
                    drawing_context.display.draw_rect(
                        rect(
                            x + side_margin,
                            y + start_margin,
                            width - side_margin - side_margin,
                            background_size,
                        ),
                        background,
                    );
                }
            }
        }
    }
}
