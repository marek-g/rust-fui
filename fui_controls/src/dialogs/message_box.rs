use crate::*;
use fui_core::*;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

pub struct DialogButtonViewModel {
    pub text: Property<String>,
    pub callback: Callback<()>,
}

impl DialogButtonViewModel {
    pub fn new(text: String, callback: Callback<()>) -> Self {
        Self {
            text: Property::new(text),
            callback,
        }
    }
}

impl ViewModel for DialogButtonViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut DialogButtonViewModel = &mut view_model.borrow_mut();

        ui! {
            Button {
                clicked: vm.callback.clone(),
                Text { text: &vm.text }
            }
        }
    }
}

#[derive(TypedBuilder)]
pub struct MessageBox {
    #[builder(default = String::new())]
    message: String,

    #[builder(default = Vec::new())]
    buttons: Vec<(String, Callback<()>)>,
}

impl MessageBox {
    pub fn show(self, window: &Rc<RefCell<dyn WindowService>>) {
        let mut buttons = ObservableVec::<Rc<RefCell<DialogButtonViewModel>>>::new();

        let content = ui! {
            Border {
                border_type: BorderType::None,
                Style: Default { background_color: [1.0f32, 1.0f32, 1.0f32, 0.5f32], },
                HorizontalAlignment: Alignment::Stretch,
                VerticalAlignment: Alignment::Stretch,

                Shadow {
                    Style: Default { size: 12.0f32 },
                    HorizontalAlignment: Alignment::Center,
                    VerticalAlignment: Alignment::Center,

                    Border {
                        border_type: BorderType::None,
                        Style: Default { background_color: [0.0f32, 0.0f32, 0.0f32, 0.8f32], },

                        Vertical {
                            Margin: Thickness::all(10.0f32),

                            Text { text: self.message },

                            Grid {
                                Margin: Thickness::top(10.0f32),
                                rows: 1,
                                &buttons
                            }
                        }
                    }
                }
            }
        };

        for (text, callback) in self.buttons {
            // create new_callback that closes dialog layer
            // and calls button callback
            let window_clone = window.clone();
            let content_clone: Rc<RefCell<dyn ControlObject>> = content.clone();
            let new_callback = Callback::simple(move |_| {
                window_clone.borrow_mut().remove_layer(&content_clone);
                callback.emit(());
            });

            buttons.push(Rc::new(RefCell::new(DialogButtonViewModel::new(
                text,
                new_callback,
            ))));
        }

        window.borrow_mut().add_layer(content);
    }
}
