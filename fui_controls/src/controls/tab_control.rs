use std::cell::{Cell, RefCell};
use std::rc::Rc;

use typed_builder::TypedBuilder;
use typemap::TypeMap;

use fui_core::*;
use fui_macros::ui;

use crate::controls::*;
use crate::{DataHolder, RadioController};

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
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let tabs_source =
            &context.children as &dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>;
        let selected_tab = Property::new(tabs_source.get(0).unwrap());

        let selected_tab_clone = selected_tab.clone();
        let tab_button_vms =
            tabs_source.map(move |c| TabButtonViewModel::new(&c, &selected_tab_clone));
        let tab_button_vms_controls = (&tab_button_vms
            as &dyn ObservableCollection<Rc<TabButtonViewModel>>)
            .map(|vm| vm.create_view());

        let content = ui! {
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto), (1, Length::Fill(1.0f32))],

                Horizontal {
                    &tab_button_vms_controls,
                },

                Shadow {
                    Border {
                        border_type: BorderType::Raisen,
                        Style: Default { background_color: [1.0f32, 1.0f32, 1.0f32, 0.05f32], },

                        &selected_tab,
                    }
                }
            }
        };

        let radio_controller =
            RadioController::<StyledControl<ToggleButton>>::new(tab_button_vms_controls);

        let data_holder = DataHolder {
            data: (selected_tab, radio_controller, tab_button_vms),
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

struct TabButtonViewModel {
    pub title: Property<String>,
    pub is_checked: Property<bool>,
    pub content: Rc<RefCell<dyn ControlObject>>,
    pub selected_tab: Property<Rc<RefCell<dyn ControlObject>>>,
    pub event_subscription: Cell<Option<Subscription>>,
}

impl TabButtonViewModel {
    pub fn new(
        content: &Rc<RefCell<dyn ControlObject>>,
        selected_tab: &Property<Rc<RefCell<dyn ControlObject>>>,
    ) -> Rc<Self> {
        let title = content
            .borrow()
            .get_context()
            .get_attached_values()
            .get::<Title>()
            .map(|t| t.clone())
            .unwrap_or_else(|| Property::new("Tab"));

        let vm = Rc::new(TabButtonViewModel {
            title,
            is_checked: Property::new(false),
            content: content.clone(),
            selected_tab: selected_tab.clone(),
            event_subscription: Cell::new(None),
        });

        {
            let weak_vm = Rc::downgrade(&vm);
            vm.event_subscription
                .set(Some(vm.is_checked.on_changed(move |is_checked| {
                    if is_checked {
                        weak_vm.upgrade().map(|vm| {
                            let content_clone = vm.content.clone();
                            vm.selected_tab.set(content_clone);
                        });
                    }
                })));
        }

        vm
    }
}

impl ViewModel for TabButtonViewModel {
    fn create_view(vm: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            ToggleButton {
                Style: Tab {},

                is_checked: vm.is_checked.clone(),

                Text {
                    Style: Dynamic {
                        color: (&vm.is_checked, |is_checked|
                            if is_checked { [1.0f32, 0.8f32, 0.0f32, 1.0f32] }
                            else { [1.0f32, 1.0f32, 1.0f32, 1.0f32] }),
                    },

                    text: &vm.title
                },
            }
        }
    }
}
