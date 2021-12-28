#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use typemap::TypeMap;

struct MainViewModel {
    pub item1: Rc<RefCell<Item1ViewModel>>,
    pub item2: Rc<RefCell<Item2ViewModel>>,

    pub content: Property<Rc<RefCell<dyn ControlObject>>>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let item1 = Item1ViewModel::new();
        let item2 = Item2ViewModel::new();
        let content = Property::new(ViewModel::create_view(&item1));

        let main_vm = Rc::new(RefCell::new(MainViewModel {
            item1,
            item2,
            content,
        }));

        main_vm
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Horizontal {
                    Button {
                        Text { text: " - Content 1 - " },
                        clicked: Callback::new(view_model, |vm, _| {
                            vm.content.set(ViewModel::create_view(&vm.item1));
                        }),
                    },
                    Button {
                        Text { text: " - Content 2 - " },
                        clicked: Callback::new(view_model, |vm, _| {
                            vm.content.set(ViewModel::create_view(&vm.item2));
                        }),
                    },
                },

                &vm.content,
            }
        )
    }
}
struct Item1ViewModel;

impl Item1ViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Item1ViewModel {}))
    }
}

impl ViewModel for Item1ViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: "Item 1" },
            }
        )
    }
}

struct Item2ViewModel;

impl Item2ViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Item2ViewModel {}))
    }
}

impl ViewModel for Item2ViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: "Item 2" },
            }
        )
    }
}
fn main() -> Result<()> {
    let mut app = Application::new("Example: content control")?;

    app.get_window_manager().borrow_mut().add_window(
        WindowOptions::new()
            .with_title("Example: content control")
            .with_size(800, 600),
        MainViewModel::new(),
    )?;

    app.run()?;

    Ok(())
}
