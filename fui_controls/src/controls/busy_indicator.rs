use crate::controls::*;
use crate::DataHolder;
use fui_core::*;
use fui_drawing::Color;
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

#[derive(TypedBuilder)]
pub struct BusyIndicator {
    #[builder(default = Property::new(false))]
    pub is_busy: Property<bool>,
}

impl BusyIndicator {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let content = ui! {
            Grid {
                Visible: self.is_busy,

                Border {
                    border_type: BorderType::None,
                    Style: Default { background_color: Color::rgba(0.0, 0.0, 0.0, 0.7), },

                    context.children,
                }
            }
        };

        let data_holder = DataHolder { data: () };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: context.attached_values,
                children: Children::SingleStatic(content),
            },
        )
    }
}
