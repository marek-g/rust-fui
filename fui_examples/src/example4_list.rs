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

struct MainViewModel {
    pub items: Vec<ItemViewModel>,
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel {
            items: vec![
                ItemViewModel {
                    name: Property::new("Element 1"),
                    number: Property::new(10),
                },
                ItemViewModel {
                    name: Property::new("Element 2"),
                    number: Property::new(11),
                },
                ItemViewModel {
                    name: Property::new("Element 3"),
                    number: Property::new(12),
                },
                ItemViewModel {
                    name: Property::new("Element 4"),
                    number: Property::new(13),
                },
            ],
        }
    }
}

impl View for MainViewModel {
    fn to_view(self, _context: ViewContext) -> Rc<RefCell<ControlObject>> {
        let view_model = &Rc::new(RefCell::new(self));
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        ui!(
            Vertical {
                Horizontal {
                    Text { text: &vm.items[0].name },
                    Text { text: (&vm.items[0].number, |n| format!(" - {}", n)) },
                },
                Horizontal {
                    Text { text: &vm.items[1].name },
                    Text { text: (&vm.items[1].number, |n| format!(" - {}", n)) },
                },
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
            .add_window_view_model(window_builder, app.get_events_loop(), main_view_model)
            .unwrap();
    }

    app.run();
}
