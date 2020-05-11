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
        let tabs_source = Rc::new(context.children);
        let selected_tab = Rc::new(RefCell::new(Property::new(tabs_source.get(0))));

        let mut tab_buttons = ObservableVec::new();
        let len = tabs_source.len();
        for i in 0..len {
            tab_buttons.push(Rc::new(RefCell::new(TabButtonViewModel {
                index: i,
                title: format!("Tab {}", i + 1),
                tabs_source: tabs_source.clone(),
                selected_tab: selected_tab.clone(),
            })));
        }

        ui! {
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto), (1, Length::Fill(1.0f32))],

                Horizontal {
                    &tab_buttons,
                },

                Border {
                    &selected_tab,
                },
            }
        }
    }
}

struct TabButtonViewModel {
    pub index: usize,
    pub title: String,
    pub tabs_source: Rc<Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>>,
    pub selected_tab: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
}

impl ViewModel for TabButtonViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Button {
                Text { text: view_model.borrow().title.clone() },
                clicked: Callback::new(view_model,
                    |vm, _| vm.selected_tab.borrow_mut().set(
                        vm.tabs_source.get(vm.index))),
            }
        }
    }
}
