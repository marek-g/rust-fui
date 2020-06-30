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
// DropDown.
//

#[derive(TypedBuilder)]
pub struct DropDown {
    #[builder(default = Property::new(0usize))]
    pub selected_index: Property<usize>,
}

impl DropDown {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let items_source = Rc::new(context.children);
        let selected_item = Rc::new(RefCell::new(Property::new(items_source.get(0))));

        /*let selected_item_clone = selected_item.clone();
        let tab_button_vms =
            items_source.map(move |c|
                TabButtonViewModel::new(&c, &selected_item_clone));*/

        let is_popup_open_property_rc = Rc::new(RefCell::new(Property::new(false)));
        let is_popup_open_property2 = Property::binded_from(&is_popup_open_property_rc.borrow_mut());

        let mut click_callback = Callback::empty();
        click_callback.set(move |_| {
            is_popup_open_property_rc.borrow_mut().change(|val: bool| !val);
        });

        let content = ui! {
            Button {
                clicked: click_callback,
                &selected_item,
            }
        };

        let popup = ui! {
            Popup {
                is_open: is_popup_open_property2,

                Vertical {
                    Text { text: "Ojej ale popup!!" },
                }
            }
        };

        let data_holder = DataHolder {
            data: (selected_item)
        };
        data_holder.to_view(None, ViewContext {
            attached_values: context.attached_values,
            children: Box::new(vec![content as Rc<RefCell<dyn ControlObject>>,
                popup as Rc<RefCell<dyn ControlObject>>]),
        })
    }
}

/*struct TabButtonViewModel {
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
}*/
