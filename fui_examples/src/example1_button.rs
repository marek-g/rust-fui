#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;

use typed_builder::TypedBuilder;
use typemap::TypeMap;

struct MainViewModel {
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            counter: Property::new(10),
            counter2: Property::new(0),
        }))
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

#[derive(TypedBuilder)]
pub struct ButtonText {
    #[builder(default = Property::new("".to_string()))]
    pub text: Property<String>,
    #[builder(default = Callback::empty())]
    pub clicked: Callback<()>,
}

impl ButtonText {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Button {
                clicked: self.clicked,
                Text { text: self.text }
            }
        }
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        vm.counter2.bind(&mut vm.counter);
        vm.counter.bind(&mut vm.counter2);

        ui!(
            Horizontal {
                Margin: Thickness::sides(0.0f32, 5.0f32),
                Text {
                    Margin: Thickness::all(5.0f32),
                    text: (&vm.counter, |counter| format!("Counter {}", counter))
                },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.decrease()),
                    Text { text: "Decrease" }
                },
                ButtonText {
                    clicked: Callback::new(view_model, |vm, _| vm.increase()),
                    text: "Increase"
                },
                Text {
                    Margin: Thickness::all(5.0f32),
                    text: (&vm.counter2, |counter| format!("Counter2 {}", counter))
                },
            }
        )
    }
}

fn main() -> Result<()> {
    let mut app = Application::new("Example: button");

    app.add_window(
        WindowOptions::new()
            .with_title("Example: button")
            .with_size(800, 600),
        MainViewModel::new(),
    )?;

    app.run()?;

    Ok(())
}
