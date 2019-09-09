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
use std::rc::Rc;

use typemap::TypeMap;

struct ItemViewModel {
    pub name: Property<String>,
    pub number: Property<i32>,
}

impl ItemViewModel {
    pub fn new(name: Property<String>, number: Property<i32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ItemViewModel {
            name,
            number,
        }))
    }
}

struct MainViewModel {
    pub items: ObservableVec<ItemViewModel>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            items: ObservableVec::new(vec![
                ItemViewModel::new(Property::new("Element 1"), Property::new(10)),
                ItemViewModel::new(Property::new("Element 2"), Property::new(11)),
                ItemViewModel::new(Property::new("Element 3"), Property::new(12)),
                ItemViewModel::new(Property::new("Element 4"), Property::new(13)),
            ]),
        }))
    }
}

impl RcView for MainViewModel {
    fn to_view(view_model: &Rc<RefCell<Self>>, _context: ViewContext) -> Rc<RefCell<ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Vertical {
                &vm.items
            }
        )
    }
}

impl RcView for ItemViewModel {
    fn to_view(view_model: &Rc<RefCell<Self>>, _context: ViewContext) -> Rc<RefCell<ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: &vm.name },
                Text { text: (&vm.number, |n| format!(" - {}", n)) },
                Button { Text { text: "Youpi!" } }
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
