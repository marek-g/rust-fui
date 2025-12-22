use crate::*;
use fui_core::*;
use fui_macros::ui;
use futures_channel::oneshot;
use futures_channel::oneshot::Canceled;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use typemap::TypeMap;

pub struct InputDialog {
    message: String,
}

impl InputDialog {
    pub fn new<S: Into<String>>(message: S) -> Self {
        InputDialog {
            message: message.into(),
        }
    }

    pub async fn get_text<S: Into<String>>(
        self,
        window: &Rc<dyn WindowService>,
        initial_text: S,
    ) -> Option<String> {
        self.get_text_internal(window, initial_text.into(), false)
            .await
            .unwrap()
    }

    pub async fn get_password(self, window: &Rc<dyn WindowService>) -> Option<String> {
        self.get_text_internal(window, String::new(), true)
            .await
            .unwrap()
    }

    fn get_text_internal(
        self,
        window: &Rc<dyn WindowService>,
        initial_text: String,
        is_password: bool,
    ) -> impl Future<Output = Result<Option<String>, Canceled>> + use<> {
        let (sender, receiver) = oneshot::channel::<Option<String>>();
        let sender = Rc::new(RefCell::new(Some(sender)));

        let mut cancel_callback = Callback::empty();
        let mut ok_callback = Callback::empty();
        let mut text_property = Property::<String>::new(initial_text);

        let content = ui! {
            Border {
                border_type: BorderType::None,
                Style: Default { background_color: [1.0f32, 1.0f32, 1.0f32, 0.5f32], },
                HorizontalAlignment: Alignment::Stretch,
                VerticalAlignment: Alignment::Stretch,

                Shadow {
                    Style: Default { size: 12.0f32 },
                    Margin: Thickness::all(20.0f32),
                    HorizontalAlignment: Alignment::Center,
                    VerticalAlignment: Alignment::Center,

                    Border {
                        border_type: BorderType::None,
                        Style: Default { background_color: [0.0f32, 0.0f32, 0.0f32, 0.8f32], },

                        Vertical {
                            Margin: Thickness::all(10.0f32),

                            Text { text: self.message },

                            TextBox {
                                Style: Default { password: is_password },
                                text: &mut text_property
                            },

                            Grid {
                                Margin: Thickness::top(10.0f32),
                                rows: 1,

                                Button {
                                    clicked: cancel_callback.clone(),
                                    Text { text: "Cancel" }
                                },

                                Button {
                                    clicked: ok_callback.clone(),
                                    Text { text: "OK" }
                                },
                            }
                        }
                    }
                }
            }
        };

        cancel_callback.set_sync({
            let sender = sender.clone();
            let window_clone = window.clone();
            let content_clone: Rc<RefCell<dyn ControlObject>> = content.clone();
            move |_| {
                window_clone.remove_layer(&content_clone);
                sender.borrow_mut().take().unwrap().send(None).unwrap();
            }
        });

        ok_callback.set_sync({
            let sender = sender.clone();
            let window_clone = window.clone();
            let content_clone: Rc<RefCell<dyn ControlObject>> = content.clone();
            move |_| {
                window_clone.remove_layer(&content_clone);
                sender
                    .borrow_mut()
                    .take()
                    .unwrap()
                    .send(Some(text_property.get()))
                    .unwrap();
            }
        });

        window.add_layer(content);

        receiver
    }
}
