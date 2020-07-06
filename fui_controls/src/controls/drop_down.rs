use std::cell::RefCell;
use std::rc::Rc;

use fui::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::*;
use crate::{layout::*, DataHolder, RadioController, RadioElement};

//
// DropDown.
//

#[derive(TypedBuilder)]
pub struct DropDown {
    #[builder(default = Property::new(0usize))]
    pub selected_index: Property<usize>,
    #[builder(default = Box::new(Vec::<Box<dyn WeakViewModelObject + 'static>>::new()))]
    pub items: Box<dyn ObservableCollection<Box<dyn WeakViewModelObject>>>,
}

impl DropDown {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let items_source = Rc::new(context.children);
        let selected_item = Rc::new(RefCell::new(Property::new(items_source.get(0))));

        let selected_item_clone = selected_item.clone();
        let menu_item_vms =
            items_source.map(move |c| MenuItemViewModel::new(&c, &selected_item_clone));

        let is_popup_open_property_rc = Rc::new(RefCell::new(Property::new(false)));
        let is_popup_open_property2 =
            Property::binded_from(&is_popup_open_property_rc.borrow_mut());

        let mut click_callback = Callback::empty();
        click_callback.set(move |_| {
            is_popup_open_property_rc
                .borrow_mut()
                .change(|val: bool| !val);
        });

        let content = ui! {
            Button {
                clicked: click_callback,
                &selected_item,

                Popup {
                    is_open: is_popup_open_property2,

                    Vertical {
                        &menu_item_vms,
                    }
                }
            }
        };

        let radio_controller = RadioController::new(menu_item_vms);

        let data_holder = DataHolder {
            data: (selected_item, radio_controller),
        };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: context.attached_values,
                children: Box::new(vec![content as Rc<RefCell<dyn ControlObject>>]),
            },
        )
    }
}

struct MenuItemViewModel {
    pub is_checked: Property<bool>,
    pub content: Rc<RefCell<dyn ControlObject>>,
    pub selected_tab: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
    pub event_subscription: Option<EventSubscription>,
}

impl MenuItemViewModel {
    pub fn new(
        content: &Rc<RefCell<dyn ControlObject>>,
        selected_tab: &Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
    ) -> Rc<RefCell<Self>> {
        let vm_rc = Rc::new(RefCell::new(MenuItemViewModel {
            is_checked: Property::new(false),
            content: content.clone(),
            selected_tab: selected_tab.clone(),
            event_subscription: None,
        }));

        {
            let weak_vm = Rc::downgrade(&vm_rc);
            let mut vm = vm_rc.borrow_mut();
            vm.event_subscription = Some(vm.is_checked.on_changed(move |is_checked| {
                if is_checked {
                    weak_vm.upgrade().map(|vm| {
                        let vm = vm.borrow();
                        vm.selected_tab.borrow_mut().set(vm.content.clone());
                    });
                }
            }));
        }

        vm_rc
    }
}

impl ViewModel for MenuItemViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let mut vm = view_model.borrow_mut();
        let content = vm.content.clone();
        ui! {
            ToggleButton {
                Style: Tab {},

                is_checked: &mut vm.is_checked,

                @content,
            }
        }
    }
}

impl RadioElement for MenuItemViewModel {
    fn is_checked(&self) -> bool {
        self.is_checked.get()
    }

    fn set_is_checked(&mut self, is_checked: bool) {
        self.is_checked.set(is_checked)
    }

    fn on_checked(&self, f: Box<dyn Fn()>) -> EventSubscription {
        self.is_checked.on_changed(move |is_checked| {
            if is_checked {
                f();
            }
        })
    }
}
