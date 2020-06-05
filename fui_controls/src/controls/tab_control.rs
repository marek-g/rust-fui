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
use crate::{DataHolder, layout::*, RadioController, RadioElement};

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

impl TabControl {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let tabs_source = Rc::new(context.children);
        let selected_tab = Rc::new(RefCell::new(Property::new(tabs_source.get(0))));

        let selected_tab_clone = selected_tab.clone();
        let tab_button_vms: Box<dyn ObservableCollection<Rc<RefCell<TabButtonViewModel>>>> =
            Box::new(tabs_source.map(move |c|
                TabButtonViewModel::new(&c, &selected_tab_clone)
        ));

        let tab_radio_buttons =
            Rc::new(RefCell::new(tab_button_vms.map(
                |c| {
                    let r: Rc<RefCell<dyn RadioElement>> = c.clone(); r
                })));
        let radio_controller = RadioController::new(tab_radio_buttons);

        let content = ui! {
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto), (1, Length::Fill(1.0f32))],

                Horizontal {
                    &tab_button_vms,
                },

                Border {
                    &selected_tab,
                },
            }
        };

        let data_holder = DataHolder {
            data: (tab_button_vms, selected_tab, radio_controller)
        };
        data_holder.to_view(None, ViewContext {
            attached_values: context.attached_values,
            children: Box::new(vec![content as Rc<RefCell<dyn ControlObject>>]),
        })
    }
}

struct TabButtonViewModel {
    pub title: Property<String>,
    pub is_checked: Property<bool>,
    pub content: Rc<RefCell<dyn ControlObject>>,
    pub selected_tab: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
    pub event_subscription: Option<EventSubscription>,
}

impl TabButtonViewModel {
    pub fn new(content: &Rc<RefCell<dyn ControlObject>>,
        selected_tab: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>) -> Rc<RefCell<Self>> {
        let title = content.borrow()
            .get_context().get_attached_values()
            .get::<Title>()
            .map(|t| Property::binded_from(t))
            .unwrap_or_else(|| Property::new("Tab"));

        let vm_rc = Rc::new(RefCell::new(TabButtonViewModel {
            title,
            is_checked: Property::new(false),
            content: content.clone(),
            selected_tab: selected_tab.clone(),
            event_subscription: None,
        }));

        {
            let weak_vm = Rc::downgrade(&vm_rc);
            let mut vm = vm_rc.borrow_mut();
            vm.event_subscription = Some(vm.is_checked.on_changed(
                move |is_checked| {
                    if is_checked {
                        weak_vm.upgrade().map(|vm| {
                            let vm = vm.borrow();
                            vm.selected_tab.borrow_mut().set(
                                vm.content.clone());
                        });
                    }
                }
            ));
        }

        vm_rc
    }
}

impl ViewModel for TabButtonViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let mut vm = view_model.borrow_mut();
        ui! {
            ToggleButton {
                Style: Tab {},

                is_checked: &mut vm.is_checked,

                Text { text: &vm.title },
            }
        }
    }
}

impl RadioElement for TabButtonViewModel {
    fn is_checked(&self) -> bool {
        self.is_checked.get()
    }

    fn set_is_checked(&mut self, is_checked: bool) {
        self.is_checked.set(is_checked)
    }

    fn on_checked(&self, f: Box<dyn Fn()>) -> EventSubscription {
        self.is_checked.on_changed(move |is_checked| if is_checked { f(); })
    }
}
