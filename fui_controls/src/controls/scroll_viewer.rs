use std::cell::RefCell;
use std::rc::Rc;

use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::border::Border;
use crate::controls::scroll_area::{ScrollArea, ViewportInfo};
use crate::controls::scroll_bar::ScrollBar;
use crate::layout::*;

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
        let mut offset_x_prop1 = Property::new(0.0f32);
        let offset_x_prop2 = Property::binded_two_way(&mut offset_x_prop1);

        let mut offset_y_prop1 = Property::new(0.0f32);
        let offset_y_prop2 = Property::binded_two_way(&mut offset_y_prop1);

        let viewport_info_prop_src = Property::new(ViewportInfo::default());

        let viewport_width_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                info.viewport_width
            });
        let viewport_height_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                info.viewport_height
            });
        let max_offset_x_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                (info.content_width - info.viewport_width).max(0.0f32)
            });
        let max_offset_y_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                (info.content_height - info.viewport_height).max(0.0f32)
            });

        let scroll_area = ScrollArea::builder()
            .offset_x(offset_x_prop1)
            .offset_y(offset_y_prop1)
            .viewport_info(viewport_info_prop_src)
            .build()
            .to_view(context);

        ui! {
            Grid {
                columns: 2,

                widths: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],
                heights: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],

                Border {
                    @scroll_area,
                },

                ScrollBar {
                    orientation: Orientation::Vertical,
                    value: offset_y_prop2,
                    max_value: max_offset_y_prop,
                    viewport_size: viewport_height_prop,
                },

                ScrollBar {
                    orientation: Orientation::Horizontal,
                    value: offset_x_prop2,
                    max_value: max_offset_x_prop,
                    viewport_size: viewport_width_prop,
                },
            }
        }
    }
}
