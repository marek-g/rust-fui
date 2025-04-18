use crate::controls::*;
use fui_core::*;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

#[derive(Copy, Clone)]
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
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let mut choose_callback = Callback::empty();

        let control = ui! {
            Grid {
        Margin: Thickness::all(8.0f32),

        rows: 1,
        default_width: Length::Auto,
        widths: vec![(1, Length::Fill(1.0f32))],

                Text { text: self.label.clone() },
                TextBox { text: self.path.clone() },
                Button { Text { text: "..." }, clicked: choose_callback.clone() },
            }
        };

        let control_weak = Rc::downgrade(&control);

        choose_callback.set_async({
            move |_| {
                let control_weak = control_weak.clone();
                let path_prop = self.path.clone();
                let label_prop = self.label.clone();
                async move {
                    let mut file_dialog_service = None;
                    if let Some(control) = control_weak.upgrade() {
                        if let Some(services) = control.borrow().get_context().get_services() {
                            if let Some(services) = services.upgrade() {
                                file_dialog_service =
                                    Some(services.borrow().get_file_dialog_service());
                            }
                        }
                    }
                    if let Some(file_dialog_service) = file_dialog_service {
                        let dialog_data = FileDialogData::new().with_title(&label_prop.get());
                        let path = match self.kind {
                            PathKind::OpenFile => file_dialog_service.pick_file(dialog_data).await,

                            PathKind::SaveFile => {
                                file_dialog_service.pick_save_file(dialog_data).await
                            }
                        };

                        if let Some(path) = path {
                            path_prop.set(path.to_str().map(|s| s.to_string()).unwrap_or_default());
                        }
                    }
                }
            }
        });

        control
    }
}
