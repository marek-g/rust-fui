#![windows_subsystem = "windows"]

use anyhow::{Error, Result};
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use tokio::task::LocalSet;

use typemap::TypeMap;

#[derive(Clone)]
struct ItemViewModel {
    pub id: i32,
    pub name: String,
}

impl ItemViewModel {
    pub fn new(id: i32, name: String) -> Rc<Self> {
        Rc::new(ItemViewModel { id, name })
    }
}

struct MainViewModel {
    pub items: RefCell<ObservableVec<Rc<ItemViewModel>>>,
    counter: Cell<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<Self> {
        let main_vm = Rc::new(MainViewModel {
            items: RefCell::new(ObservableVec::new()),
            counter: Cell::new(0),
        });

        main_vm.add_n(4);

        main_vm
    }

    pub fn add(self: &Rc<Self>) {
        let new_item = ItemViewModel::new(
            self.counter.get(),
            format!("Element {}", self.counter.get()),
        );
        self.counter.set(self.counter.get() + 1);

        println!("Add {}!", new_item.name);
        self.items.borrow_mut().push(new_item);
    }

    pub fn add_n(self: &Rc<Self>, n: i32) {
        for _ in 0..n {
            self.add();
        }
    }

    pub fn remove_all(self: &Rc<Self>) {
        println!("Remove all!");
        self.items.borrow_mut().clear();
    }

    pub fn delete(self: &Rc<Self>, item: &Rc<ItemViewModel>) {
        println!("Delete {}!", item.id);
        self.items
            .borrow_mut()
            .remove_filter(|i| Rc::ptr_eq(i, item));
    }
}

impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Vertical {
                    Margin: Thickness::all(5.0f32),
                    Button {
                        clicked: Callback::new_rc(self, |vm, _| vm.add()),
                        Text { text: "Add" },
                    },
                    Button {
                        clicked: Callback::new_rc(self, |vm, _| vm.add_n(100)),
                        Text { text: "Add 100" },
                    },
                    Button {
                        clicked: Callback::new_rc(self, |vm, _| vm.remove_all()),
                        Text { text: "Remove all" },
                    },
                },

                ScrollViewer {
                    Vertical {
                        Margin: Thickness::all(5.0f32),
                        Text { text: "The dynamic list can be mixed with static controls." },

                        Grid {
                            columns: 3,

                            self.items.borrow().flat_map({
                                let view_model = self.clone();
                                move |item| {
                                vec![
                                    ui!(Text { text: "Flat map!" }),
                                    ui!(Text { text: &item.name }),
                                    ui!(Button {
                                        Margin: Thickness::new(5.0f32, 0.0f32, 0.0f32, 0.0f32),
                                        clicked: Callback::new_sync({
                        let vm = view_model.clone();
                        let item = item.clone();
                        move |_| vm.delete(&item)
                    }),
                                        Text { text: "Delete" },
                                    }),
                                ]
                            }}),

                            self.items.borrow().map(|item| {
                                ui!(Text { text: format!("Simple map! ({})", item.id) })
                            })
                        },

                        Text { text: "This is the end." },
                    },
                }
            }
        )
    }
}

#[tokio::main(flavor = "current_thread")]
//#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            let app = Application::new("Example: list").await?;

            let mut window = Window::create(
                WindowOptions::new()
                    .with_title("Example: list")
                    .with_size(800, 600),
            )
            .await?;

            window.set_vm(MainViewModel::new());

            app.run().await?;

            Ok::<(), Error>(())
        })
        .await
}
