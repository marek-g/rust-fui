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
    #[builder(default = Box::new(Vec::<Box<dyn ViewModelObject>>::new()))]
    pub items: Box<dyn ObservableCollection<Box<dyn ViewModelObject>>>,
}

impl DropDown {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let selected_item = Rc::new(RefCell::new(Property::new(self.items.get(0).create_view())));

        let is_popup_open_property_rc = Rc::new(RefCell::new(Property::new(false)));
        let is_popup_open_property2 =
            Property::binded_from(&is_popup_open_property_rc.borrow_mut());

        let is_popup_open_property_rc_clone = is_popup_open_property_rc.clone();
        let mut show_callback = Callback::empty();
        show_callback.set(move |_| {
            is_popup_open_property_rc_clone.borrow_mut().set(true);
        });

        let mut hide_callback = Callback::empty();
        hide_callback.set(move |_| {
            is_popup_open_property_rc.clone().borrow_mut().set(false);
        });

        let selected_item_clone = selected_item.clone();
        let hide_callback_clone = hide_callback.clone();
        let menu_item_vms = self.items.map(move |c| {
            MenuItemViewModel::new(
                c.clone(),
                selected_item_clone.clone(),
                hide_callback_clone.clone(),
            )
        });

        let content = ui! {
            Button {
                clicked: show_callback.clone(),
                &selected_item,

                Popup {
                    is_open: is_popup_open_property2,
                    placement: PopupPlacement::BelowOrAboveParent,
                    clicked_outside: hide_callback,

                    Grid {
                        columns: 1,

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
    pub clicked_callback: Callback<()>,
    pub source_vm: Box<dyn ViewModelObject>,
    pub selected_item: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
    pub event_subscription: Option<EventSubscription>,
}

impl MenuItemViewModel {
    pub fn new(
        source_vm: Box<dyn ViewModelObject>,
        selected_item: Rc<RefCell<Property<Rc<RefCell<dyn ControlObject>>>>>,
        clicked_callback: Callback<()>,
    ) -> Rc<RefCell<Self>> {
        let vm_rc = Rc::new(RefCell::new(MenuItemViewModel {
            is_checked: Property::new(false),
            clicked_callback: clicked_callback,
            source_vm: source_vm,
            selected_item: selected_item,
            event_subscription: None,
        }));

        {
            let weak_vm = Rc::downgrade(&vm_rc);
            let mut vm = vm_rc.borrow_mut();
            let clicked_callback = vm.clicked_callback.clone();
            vm.event_subscription = Some(vm.is_checked.on_changed(move |is_checked| {
                if is_checked {
                    weak_vm.upgrade().map(|vm| {
                        let vm = vm.borrow();
                        vm.selected_item
                            .borrow_mut()
                            .set(vm.source_vm.create_view());
                    });
                    clicked_callback.emit(());
                }
            }));
        }

        vm_rc
    }
}

impl ViewModel for MenuItemViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let mut vm = view_model.borrow_mut();
        let clicked_callback = vm.clicked_callback.clone();
        let content = vm.source_vm.create_view();
        ui! {
            ToggleButton {
                Style: DropDown {
                    clicked: clicked_callback,
                },
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
