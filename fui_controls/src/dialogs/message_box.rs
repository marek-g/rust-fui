use crate::*;
use fui_core::*;
use fui_macros::ui;
use futures_channel::oneshot;
use futures_channel::oneshot::Canceled;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
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
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Children {
        let vm: &mut DialogButtonViewModel = &mut view_model.borrow_mut();

        ui! {
            Button {
                clicked: vm.callback.clone(),
                Text { text: &vm.text }
            }
        }
    }
}

pub struct MessageBox {
    message: String,
    buttons: Vec<String>,
}

impl MessageBox {
    pub fn new<S: Into<String>>(message: S) -> Self {
        MessageBox {
            message: message.into(),
            buttons: Vec::new(),
        }
    }

    pub fn with_button<S: Into<String>>(mut self, text: S) -> Self {
        self.buttons.push(text.into());
        self
    }

    pub async fn show(self, window: &Rc<RefCell<dyn WindowService>>) -> i32 {
        self.show_internal(window).await.unwrap()
    }

    fn show_internal(
        self,
        window: &Rc<RefCell<dyn WindowService>>,
    ) -> impl Future<Output = Result<i32, Canceled>> {
        let mut buttons = ObservableVec::<Rc<RefCell<DialogButtonViewModel>>>::new();

        let (sender, receiver) = oneshot::channel::<i32>();
        let sender = Rc::new(RefCell::new(Some(sender)));

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
        }
        .single();

        for (button_index, text) in self.buttons.into_iter().enumerate() {
            // create new_callback that closes dialog layer
            // and calls button callback
            let window_clone = window.clone();
            let content_clone: Rc<RefCell<dyn ControlObject>> = content.clone();
            let new_callback = Callback::new_sync({
                let sender = sender.clone();
                move |_| {
                    window_clone.borrow_mut().remove_layer(&content_clone);
                    sender
                        .borrow_mut()
                        .take()
                        .unwrap()
                        .send(button_index as i32)
                        .unwrap();
                }
            });

            buttons.push(Rc::new(RefCell::new(DialogButtonViewModel::new(
                text,
                new_callback,
            ))));
        }

        window.borrow_mut().add_layer(content);

        receiver
    }
}
