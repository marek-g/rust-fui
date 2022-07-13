use fui_core::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::border::Border;
use crate::controls::scroll_area::{ScrollArea, ViewportInfo};
use crate::controls::scroll_bar::ScrollBar;

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

impl ScrollViewer {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Children {
        let offset_x_prop = Property::new(0.0f32);
        let offset_y_prop = Property::new(0.0f32);

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

        let horizontal_scrollbar_visible_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                info.content_width > info.viewport_width
            });

        let vertical_scrollbar_visible_prop =
            Property::binded_c_from(&viewport_info_prop_src, |info: ViewportInfo| {
                info.content_height > info.viewport_height
            });

        ui! {
            Grid {
                columns: 2,
                widths: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],
                heights: vec![(0, Length::Fill(1.0f32)), (1, Length::Auto)],

                Border {
                    ScrollArea {
                        offset_x: offset_x_prop.clone(),
                        offset_y: offset_y_prop.clone(),
                        viewport_info: viewport_info_prop_src,

                        context.children,
                    },
                },

                ScrollBar {
                    Visible: vertical_scrollbar_visible_prop,
                    orientation: Orientation::Vertical,
                    value: offset_y_prop,
                    max_value: max_offset_y_prop,
                    viewport_size: viewport_height_prop,
                },

                ScrollBar {
                    Visible: horizontal_scrollbar_visible_prop,
                    orientation: Orientation::Horizontal,
                    value: offset_x_prop,
                    max_value: max_offset_x_prop,
                    viewport_size: viewport_width_prop,
                },
            }
        }
    }
}
