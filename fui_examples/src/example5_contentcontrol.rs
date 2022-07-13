#![windows_subsystem = "windows"]

use anyhow::{Error, Result};
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;
use tokio::task::LocalSet;

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
        let content = Property::new(ViewModel::create_view(&item1).single());

        let main_vm = Rc::new(RefCell::new(MainViewModel {
            item1,
            item2,
            content,
        }));

        main_vm
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Children {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Horizontal {
                    Button {
                        Text { text: " - Content 1 - " },
                        clicked: Callback::new_vm(view_model, |vm, _| {
                            vm.content.set(ViewModel::create_view(&vm.item1).single());
                        }),
                    },
                    Button {
                        Text { text: " - Content 2 - " },
                        clicked: Callback::new_vm(view_model, |vm, _| {
                            vm.content.set(ViewModel::create_view(&vm.item2).single());
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
    fn create_view(_view_model: &Rc<RefCell<Self>>) -> Children {
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
    fn create_view(_view_model: &Rc<RefCell<Self>>) -> Children {
        ui!(
            Horizontal {
                Text { text: "Item 2" },
            }
        )
    }
}

#[tokio::main(flavor = "current_thread")]
//#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            let app = Application::new("Example: content control").await?;

            app.get_window_manager()
                .borrow_mut()
                .add_window(
                    WindowOptions::new()
                        .with_title("Example: content control")
                        .with_size(800, 600),
                    MainViewModel::new(),
                )
                .await?;

            app.run().await?;

            Ok::<(), Error>(())
        })
        .await
}
