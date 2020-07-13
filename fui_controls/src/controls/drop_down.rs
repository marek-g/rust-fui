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
pub struct DropDown<V>
where
    V: ViewModel + PartialEq + 'static,
{
    #[builder(default = Property::new(None))]
    pub selected_item: Property<Option<Rc<RefCell<V>>>>,
    #[builder(default = Box::new(Vec::<Rc<RefCell<V>>>::new()))]
    pub items: Box<dyn ObservableCollection<Rc<RefCell<V>>>>,
}

impl<V> DropDown<V>
where
    V: ViewModel + PartialEq + 'static,
{
    pub fn to_view(
        mut self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
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

        let selected_item_rc = Rc::new(RefCell::new(Property::binded_two_way(
            &mut self.selected_item,
        )));

        let hide_callback_clone = hide_callback.clone();
        let menu_item_vms = self.items.map(move |v| {
            MenuItemViewModel::new(
                v.clone(),
                selected_item_rc.clone(),
                hide_callback_clone.clone(),
            )
        });

        let content = ui! {
            Button {
                clicked: show_callback.clone(),
                &self.selected_item,

                Popup {
                    is_open: is_popup_open_property2,
                    placement: PopupPlacement::BelowOrAboveParent,
                    clicked_outside: hide_callback,

                    ScrollViewer {
                        horizontal_scroll_bar_visibility: ScrollBarVisibility::Hidden,
                        vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,

                        Grid {
                            columns: 1,
                            &menu_item_vms,
                        }
                    }
                }
            }
        };

        let radio_controller = RadioController::new(menu_item_vms);

        let data_holder = DataHolder {
            data: (self.selected_item, self.items, radio_controller),
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

struct MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
    pub is_checked: Property<bool>,
    pub clicked_callback: Callback<()>,
    pub source_vm: Rc<RefCell<V>>,
    pub selected_item: Rc<RefCell<Property<Option<Rc<RefCell<V>>>>>>,
    pub event_subscription: Option<EventSubscription>,
}

impl<V> MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
    pub fn new(
        source_vm: Rc<RefCell<V>>,
        selected_item: Rc<RefCell<Property<Option<Rc<RefCell<V>>>>>>,
        clicked_callback: Callback<()>,
    ) -> Rc<RefCell<Self>> {
        let is_checked = match &selected_item.borrow().get() {
            None => false,
            Some(vm) => vm == &source_vm,
        };
        let vm_rc = Rc::new(RefCell::new(MenuItemViewModel {
            is_checked: Property::new(is_checked),
            clicked_callback,
            source_vm,
            selected_item,
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
                            .set(Some(vm.source_vm.clone()));
                    });
                    clicked_callback.emit(());
                }
            }));
        }

        vm_rc
    }
}

impl<V> ViewModel for MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
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

impl<V> RadioElement for MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
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
