#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;

use std::rc::Rc;
use tokio::task::LocalSet;

use Property;

struct MainViewModel {
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<Self> {
        Rc::new(MainViewModel {
            counter: 10.into(),
            counter2: 0.into(),
        })
    }

    pub fn increase(self: &Rc<Self>) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(self: &Rc<Self>) {
        self.counter.change(|c| c - 1);
    }
}

impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<dyn ControlObject> {
        self.counter2.bind(&self.counter);
        self.counter.bind(&self.counter2);

        ui!(
            Horizontal {
                Margin: Thickness::sides(0.0, 5.0),
                Text {
                    Margin: Thickness::all(5.0),
                    text: format!("Counter {}", self.counter.get())
                },
                Button {
                    clicked => self.decrease(),
                    Text { text: "Decrease" }
                },
                Button {
                    clicked => self.increase(),
                    Text { text: "Increase" }
                },
                Text {
                    Margin: Thickness::all(5.0),
                    text: format!("Counter2 {}", self.counter2.get())
                },
            }
        )
    }
}

#[tokio::main(flavor = "current_thread")]
//#[tokio::main]
async fn main() -> Result<()> {
    LocalSet::new()
        .run_until(async {
            let app = Application::new("Example: multiwindow").await?;

            let mut window1 = Window::create(
                WindowOptions::new()
                    .with_title("Window 1")
                    .with_size(800, 600),
            )
            .await?;

            window1.set_vm(MainViewModel::new());

            let mut window2 = Window::create(
                WindowOptions::new()
                    .with_title("Window2")
                    .with_size(800, 600),
            )
            .await?;

            window2.set_vm(MainViewModel::new());

            app.run().await?;

            Ok(())
        })
        .await
}
