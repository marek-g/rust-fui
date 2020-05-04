#![windows_subsystem = "windows"]

use fui::*;
use fui_app::*;
use fui_controls::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use typemap::TypeMap;
use winit::window::WindowBuilder;
struct MainViewModel {
    pub item1: Rc<RefCell<Item1ViewModel>>,
    pub item2: Rc<RefCell<Item2ViewModel>>,
    //pub content: Property<Weak<RefCell<dyn RcView>>>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let item1 = Item1ViewModel::new();
        let item2 = Item2ViewModel::new();
        //let content = Property::new(item1.clone());

        let main_vm = Rc::new(RefCell::new(MainViewModel {
            item1,
            item2,
            //content,
        }));

        main_vm
    }
}

impl RcView for MainViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Horizontal {
                    Button {
                        clicked: Callback::new_rc(view_model, |vm, _| {}),
                        Text { text: " - Content 1 - " },
                    },
                    Button {
                        clicked: Callback::new_rc(view_model, |vm, _| {}),
                        Text { text: " - Content 2 - " },
                    },
                },

                /*ScrollViewer {
                    Vertical {
                        &vm.content,
                    },
                }*/
            }
        )
    }
}
struct Item1ViewModel;

impl Item1ViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Item1ViewModel {
        }))
    }
}

impl RcView for Item1ViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
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
        Rc::new(RefCell::new(Item2ViewModel {
        }))
    }
}

impl RcView for Item2ViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: "Item 2" },
            }
        )
    }
}
fn main() -> Result<()> {
    let mut app = Application::new("Example: content control").unwrap();

    app.add_window(
        WindowBuilder::new().with_title("Example: content control"),
        MainViewModel::new(),
    )?;

    app.run();

    Ok(())
}
