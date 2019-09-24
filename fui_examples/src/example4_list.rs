#![windows_subsystem = "windows"]

extern crate fui;
extern crate fui_macros;
extern crate typed_builder;
extern crate typemap;
extern crate winit;

use fui::application::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use typemap::TypeMap;

struct ItemViewModel {
    pub parent: Weak<RefCell<MainViewModel>>,
    pub name: Property<String>,
    pub number: Property<i32>,
}

impl ItemViewModel {
    pub fn new(
        parent: Weak<RefCell<MainViewModel>>,
        name: Property<String>,
        number: Property<i32>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ItemViewModel {
            parent,
            name,
            number,
        }))
    }

    pub fn delete(&mut self) {
        println!("Delete!");
    }
}

impl RcView for ItemViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: &vm.name },
                Text { text: (&vm.number, |n| format!(" - {}", n)) },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.delete()),
                    Text { text: "Delete" },
                }
            }
        )
    }
}

struct MainViewModel {
    pub items: ObservableVec<Rc<RefCell<ItemViewModel>>>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let main_vm = Rc::new(RefCell::new(MainViewModel {
            items: ObservableVec::new(),
        }));

        {
            let main_vm_weak = Rc::downgrade(&main_vm);
            let mut main_vm_mut = main_vm.borrow_mut();
            main_vm_mut.items.push(ItemViewModel::new(
                main_vm_weak.clone(),
                Property::new("Element 1"),
                Property::new(10),
            ));
            main_vm_mut.items.push(ItemViewModel::new(
                main_vm_weak.clone(),
                Property::new("Element 2"),
                Property::new(11),
            ));
            main_vm_mut.items.push(ItemViewModel::new(
                main_vm_weak.clone(),
                Property::new("Element 3"),
                Property::new(12),
            ));
            main_vm_mut.items.push(ItemViewModel::new(
                main_vm_weak,
                Property::new("Element 4"),
                Property::new(13),
            ));
        }

        main_vm
    }
}

impl RcView for MainViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Vertical {
                Button { Text { text: "Add" } },
                &vm.items,
            }
        )
    }
}

fn main() {
    let mut app = Application::new("Example: list").unwrap();

    let main_view_model = MainViewModel::new();

    {
        let mut window_manager = app.get_window_manager().borrow_mut();
        let window_builder = winit::WindowBuilder::new().with_title("Example: list");
        window_manager
            .add_window_view_model(window_builder, app.get_events_loop(), &main_view_model)
            .unwrap();
    }

    app.run();
}
