use std::cell::{Cell, RefCell};
use std::rc::Rc;

use typed_builder::TypedBuilder;
use typemap::TypeMap;

use fui_core::*;
use fui_macros::ui;

use crate::controls::*;
use crate::{DataHolder, RadioController};

//
// DropDown.
//

#[derive(TypedBuilder)]
pub struct DropDown<V>
where
    V: ViewModel + PartialEq + 'static,
{
    #[builder(default = Property::new(None))]
    pub selected_item: Property<Option<Rc<V>>>,
    #[builder(default = Box::new(Vec::<Rc<V>>::new()))]
    pub items: Box<dyn ObservableCollection<Rc<V>>>,
}

impl<V> DropDown<V>
where
    V: ViewModel + PartialEq + 'static,
{
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let is_popup_open_property = Property::new(false);

        let is_popup_open_property_clone = is_popup_open_property.clone();
        let mut show_callback = Callback::empty();
        show_callback.set_sync(move |_| {
            is_popup_open_property_clone.set(true);
        });

        let is_popup_open_property_clone = is_popup_open_property.clone();
        let mut hide_callback = Callback::empty();
        hide_callback.set_sync(move |_| {
            is_popup_open_property_clone.set(false);
        });

        let selected_item_prop_clone = self.selected_item.clone();
        let hide_callback_clone = hide_callback.clone();
        let menu_item_vms = self.items.map(move |v| {
            MenuItemViewModel::new(
                v.clone(),
                selected_item_prop_clone.clone(),
                hide_callback_clone.clone(),
            )
        });
        let menu_item_controls = (&menu_item_vms
            as &dyn ObservableCollection<Rc<MenuItemViewModel<V>>>)
            .map(|vm| vm.create_view());

        let content = ui! {
            Button {
                clicked: show_callback.clone(),

                &self.selected_item,

                Popup {
                    is_open: is_popup_open_property,
                    auto_hide: PopupAutoHide::ClickedOutside,
                    placement: PopupPlacement::BelowOrAboveParent,

                    Shadow {
                        ScrollViewer {
                            horizontal_scroll_bar_visibility: ScrollBarVisibility::Hidden,
                            vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,

                            Grid {
                                VerticalAlignment: Alignment::Start,
                                columns: 1,

                                &menu_item_controls,
                            }
                        }
                    }
                }
            }
        };

        let radio_controller =
            RadioController::<StyledControl<ToggleButton>>::new(menu_item_controls);

        let data_holder = DataHolder {
            data: (
                self.selected_item,
                self.items,
                radio_controller,
                menu_item_vms,
            ),
        };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: context.attached_values,
                children: Children::SingleStatic(content),
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
    pub source_vm: Rc<V>,
    pub selected_item: Property<Option<Rc<V>>>,
    pub event_subscription: Cell<Option<Subscription>>,
}

impl<V> MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
    pub fn new(
        source_vm: Rc<V>,
        selected_item: Property<Option<Rc<V>>>,
        clicked_callback: Callback<()>,
    ) -> Rc<Self> {
        let is_checked = match &selected_item.get() {
            None => false,
            Some(vm) => vm == &source_vm,
        };
        let vm = Rc::new(MenuItemViewModel {
            is_checked: Property::new(is_checked),
            clicked_callback,
            source_vm,
            selected_item,
            event_subscription: Cell::new(None),
        });

        {
            let weak_vm = Rc::downgrade(&vm);
            let clicked_callback = vm.clicked_callback.clone();
            vm.event_subscription
                .set(Some(vm.is_checked.on_changed(move |is_checked| {
                    if is_checked {
                        weak_vm.upgrade().map(|vm| {
                            let source_vm_clone = vm.source_vm.clone();
                            vm.selected_item.set(Some(source_vm_clone));
                        });
                        clicked_callback.emit(());
                    }
                })));
        }

        vm
    }
}

impl<V> ViewModel for MenuItemViewModel<V>
where
    V: ViewModel + PartialEq + 'static,
{
    fn create_view(vm: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        let clicked_callback = vm.clicked_callback.clone();
        let content = vm.source_vm.create_view();
        ui! {
            ToggleButton {
                Style: DropDown {
                    clicked: clicked_callback,
                },
                is_checked: vm.is_checked.clone(),
                content,
            }
        }
    }
}
