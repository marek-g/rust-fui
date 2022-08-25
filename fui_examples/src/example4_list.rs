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

#[derive(Clone)]
struct ItemViewModel {
    pub id: i32,
    pub name: String,
}

impl ItemViewModel {
    pub fn new(id: i32, name: String) -> Self {
        ItemViewModel { id, name }
    }
}

struct MainViewModel {
    pub items: ObservableVec<ItemViewModel>,
    counter: i32,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let main_vm = Rc::new(RefCell::new(MainViewModel {
            items: ObservableVec::new(),
            counter: 0,
        }));

        main_vm.add_n(4);

        main_vm
    }
}

trait MainViewModelMethods {
    fn add(&self);
    fn add_n(&self, n: i32);
    fn remove_all(&self);
    fn delete(&self, item_id: i32);
}

impl MainViewModelMethods for Rc<RefCell<MainViewModel>> {
    fn add(&self) {
        let new_item = ItemViewModel::new(
            self.borrow().counter,
            format!("Element {}", self.borrow().counter),
        );
        self.borrow_mut().counter += 1;

        println!("Add {}!", new_item.name);
        self.borrow_mut().items.push(new_item);
    }

    fn add_n(&self, n: i32) {
        for _ in 0..n {
            self.add();
        }
    }

    fn remove_all(&self) {
        println!("Remove all!");
        self.borrow_mut().items.clear();
    }

    fn delete(&self, item_id: i32) {
        println!("Delete {}!", item_id);
        self.borrow_mut().items.remove_filter(|i| i.id == item_id);
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Vertical {
                    Margin: Thickness::all(5.0f32),
                    Button {
                        clicked: Callback::new_vm_rc(view_model, |vm, _| vm.add()),
                        Text { text: "Add" },
                    },
                    Button {
                        clicked: Callback::new_vm_rc(view_model, |vm, _| vm.add_n(100)),
                        Text { text: "Add 100" },
                    },
                    Button {
                        clicked: Callback::new_vm_rc(view_model, |vm, _| vm.remove_all()),
                        Text { text: "Remove all" },
                    },
                },

                ScrollViewer {
                    Vertical {
                        Margin: Thickness::all(5.0f32),
                        Text { text: "The dynamic list can be mixed with static controls." },

                        Grid {
                            columns: 3,

                            vm.items.flat_map({
                                let view_model = view_model.clone();
                                move |item| {
                                vec![
                                    ui!(Text { text: "Flat map!" }),
                                    ui!(Text { text: &item.name }),
                                    ui!(Button {
                                        Margin: Thickness::new(5.0f32, 0.0f32, 0.0f32, 0.0f32),
                                        clicked: Callback::new_vm_rc(&view_model, {
                                            let item_id = item.id;
                                            move |vm, _| { vm.delete(item_id); }
                                        }),
                                        Text { text: "Delete" },
                                    }),
                                ]
                            }}),

                            vm.items.map(|item| {
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
