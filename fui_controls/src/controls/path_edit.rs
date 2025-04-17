use crate::controls::*;
use fui_core::*;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

pub enum PathKind {
    OpenFile,
    SaveFile,
}

#[derive(TypedBuilder)]
pub struct PathEdit {
    #[builder(default = Property::new("".to_string()))]
    pub label: Property<String>,

    #[builder(default = Property::new("".to_string()))]
    pub path: Property<String>,

    #[builder(default = PathKind::OpenFile)]
    pub kind: PathKind,
}

impl PathEdit {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let choose_callback = Callback::new_sync({
            //let window = context..window.clone();
            move |_| {
                println!("Callback");

                //let window = window.clone();
                /*async move {
                    let file = FileDialog::new()
                        .with_title("Please select a file to save to!")
                        .with_initial_path("test.dat")
                        .with_filter("All files (*.*)", &["*.*"])
                        .with_filter("Markdown (*.md)", &["*.md"])
                        .pick_save_file()
                        .await;
                    MessageBox::new(format!("{:?}", file))
                    .with_button("Ok")
                    .show(&window)
                    .await;
                }*/
            }
        });

        ui! {
            Grid {
        Margin: Thickness::all(8.0f32),

        rows: 1,
        default_width: Length::Auto,
        widths: vec![(1, Length::Fill(1.0f32))],

                Text { text: self.label },
                TextBox { text: self.path },
                Button { Text { text: "..." }, clicked: choose_callback },
            }
        }
    }
}
