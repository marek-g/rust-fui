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
    type Value = Property<String>;
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
    fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let tabs_source = Rc::new(context.children);
        let selected_tab = Rc::new(RefCell::new(Property::new(tabs_source.get(0))));

        let selected_tab_clone = selected_tab.clone();
        let tab_buttons: Box<dyn ObservableCollection<Rc<RefCell<TabButtonViewModel>>>> =
            Box::new(tabs_source.map(move |c|
                TabButtonViewModel::new(&c, &selected_tab_clone)
        ));

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
    pub title: Property<String>,
    pub content: Rc<RefCell<dyn ControlObject>>,
    pub selected_tab: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
}

impl TabButtonViewModel {
    pub fn new(content: &Rc<RefCell<dyn ControlObject>>,
        selected_tab: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Rc<RefCell<Self>> {
        let title = content.borrow()
            .get_context().get_attached_values()
            .get::<Title>()
            .map(|t| Property::binded_from(t))
            .unwrap_or_else(|| Property::new("Tab"));
        Rc::new(RefCell::new(TabButtonViewModel {
            title,
            content: content.clone(),
            selected_tab: selected_tab.clone(),
        }))
    }
}

impl ViewModel for TabButtonViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Button {
                Text { text: &view_model.borrow().title },
                clicked: Callback::new(view_model,
                    |vm, _| vm.selected_tab.borrow_mut().set(
                        vm.content.clone())),
            }
        }
    }
}
