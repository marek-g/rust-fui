use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use drawing::units::{PixelPoint, PixelRect, PixelSize, PixelThickness};
use fui::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::*;
use crate::controls::border::Border;
use crate::controls::scroll_area::{ScrollArea, ViewportInfo};
use crate::controls::scroll_bar::ScrollBar;
use crate::layout::*;

//
// Attached values.
//

pub struct Title;
impl typemap::Key for Title {
    type Value = String;
}

//
// TabControl.
//

#[derive(TypedBuilder)]
pub struct TabControl {
    #[builder(default = Property::new(0usize))]
    pub selected_index: Property<usize>,
}

impl Control for TabControl {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let tabs_source = context.children;
        let selected_tab = Rc::new(RefCell::new(Property::new(tabs_source.index(0))));

        let data_rc = Rc::new(RefCell::new(
            (tabs_source, selected_tab.clone())));

        ui! {
            Grid {
                rows: 2,
                heights: vec![(0, Length::Auto), (1, Length::Fill(1.0f32))],

                Horizontal {
                    Button {
                        Text { text: "Tab 1" },
                        clicked: Callback::new(&data_rc,
                            |data, _| data.1.borrow_mut().set(data.0.index(0))),
                    },
                    Button {
                        Text { text: "Tab 2" },
                        clicked: Callback::new(&data_rc,
                            |data, _| data.1.borrow_mut().set(data.0.index(1))),
                    }
                },

                Border {
                    &selected_tab,
                },
            }
        }
    }
}
