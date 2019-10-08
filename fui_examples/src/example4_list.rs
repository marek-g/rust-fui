#![windows_subsystem = "windows"]

extern crate fui;
extern crate fui_macros;
extern crate typed_builder;
extern crate typemap;
extern crate winit;

use fui::application::*;
use fui::common::*;
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
                    clicked: Callback::new_rc(view_model, |vm, _| {
                        let parent = vm.borrow().parent.clone();
                        if let Some(parent) = parent.upgrade() {
                            parent.borrow_mut().delete(vm);
                        }
                    }),
                    Text { text: "Delete" },
                }
            }
        )
    }
}

struct MainViewModel {
    pub items: ObservableVec<Rc<RefCell<ItemViewModel>>>,

    self_weak: Weak<RefCell<MainViewModel>>,
    counter: i32,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let main_vm = Rc::new(RefCell::new(MainViewModel {
            items: ObservableVec::new(),
            self_weak: Weak::new(),
            counter: 0,
        }));

        main_vm.borrow_mut().self_weak = Rc::downgrade(&main_vm);
        main_vm.borrow_mut().add();
        main_vm.borrow_mut().add();
        main_vm.borrow_mut().add();
        main_vm.borrow_mut().add();

        main_vm
    }

    pub fn add(&mut self) {
        let new_item = ItemViewModel::new(
            self.self_weak.clone(),
            Property::new(format!("Element {}", self.counter)),
            Property::new(self.counter + 10),
        );
        self.counter += 1;

        println!("Add {}!", new_item.borrow().name.get());
        self.items.push(new_item);
    }

    pub fn add_100(&mut self) {
        for _ in 0..100 {
            self.add();
        }
    }

    pub fn remove_all(&mut self) {
        println!("Remove all!");
        self.items.remove_filter(|_i| true);
    }

    pub fn delete(&mut self, item: Rc<RefCell<ItemViewModel>>) {
        println!("Delete {}!", item.borrow().name.get());
        self.items
            .remove_filter(|i| std::ptr::eq(i.as_ref(), item.as_ref()));
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
                Horizontal {
                    Button {
                        clicked: Callback::new(view_model, |vm, _| vm.add()),
                        Text { text: "Add" },
                    },
                    Button {
                        clicked: Callback::new(view_model, |vm, _| vm.add_100()),
                        Text { text: "Add 100" },
                    },
                    Button {
                        clicked: Callback::new(view_model, |vm, _| vm.remove_all()),
                        Text { text: "Remove all" },
                    },
                },
                ScrollBar {
                    orientation: Orientation::Horizontal,
                    viewport_size: 0.5f32,
                    value: 0.3f32,
                },
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
