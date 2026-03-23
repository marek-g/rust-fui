#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;
use windowing_api::WindowFrameType;

use std::rc::Rc;
use tokio::task::LocalSet;

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
            Grid {
        Border { border_type: BorderType::Frame3D },

                MoveResizeArea {
            border_size: Thickness::all(3.0),

            Horizontal {
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
            let app = Application::new("Example: button").await?;

            let mut window = Window::create(
                WindowOptions::new()
                    .with_title("Example: button")
                    .with_translucent_background(windowing_api::TranslucentEffect::Transparent)
                    .with_frame_type(WindowFrameType::Frameless)
                    .with_size(800, 600),
            )
            .await?;

            window.set_vm(MainViewModel::new());

            app.run().await?;

            Ok(())
        })
        .await
}
