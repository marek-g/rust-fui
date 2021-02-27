use crate::*;
use fui_core::*;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

pub struct DialogButtonViewModel {
    pub text: Property<String>,
}

impl DialogButtonViewModel {
    pub fn new(text: String) -> Self {
        Self {
            text: Property::new(text),
        }
    }
}

impl ViewModel for DialogButtonViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut DialogButtonViewModel = &mut view_model.borrow_mut();

        ui! {
            Button { Text { text: &vm.text }}
        }
    }
}

#[derive(TypedBuilder)]
pub struct MessageBoxParams {
    #[builder(default = String::new())]
    message: String,

    #[builder(default = ObservableVec::new())]
    buttons: ObservableVec<Rc<RefCell<DialogButtonViewModel>>>,
}

pub struct MessageBox;

impl MessageBox {
    pub fn show(window: &Rc<RefCell<dyn WindowService>>, params: MessageBoxParams) {
        let content = ui! {
            Shadow {
                Style: Default { size: 12.0f32 },

                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: [0.0f32, 0.0f32, 0.0f32, 0.8f32], },

                    Vertical {
                        Text { text: params.message },

                        Horizontal { &params.buttons }
                    }
                }
            }
        };

        window.borrow_mut().add_layer(content);
    }
}
