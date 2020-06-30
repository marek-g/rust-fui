use std::cell::RefCell;
use std::rc::Rc;

use fui::*;
use fui_macros::ui;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::*;
use crate::{DataHolder, layout::*};

//
// DropDown.
//

#[derive(TypedBuilder)]
pub struct DropDown {
    #[builder(default = Property::new(0usize))]
    pub selected_index: Property<usize>,
}

impl DropDown {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<dyn ControlObject>> {
        let items_source = Rc::new(context.children);
        let selected_item = Rc::new(RefCell::new(Property::new(items_source.get(0))));

        let is_popup_open_property_rc = Rc::new(RefCell::new(Property::new(false)));
        let is_popup_open_property2 = Property::binded_from(&is_popup_open_property_rc.borrow_mut());

        let mut click_callback = Callback::empty();
        click_callback.set(move |_| {
            is_popup_open_property_rc.borrow_mut().change(|val: bool| !val);
        });

        let content = ui! {
            Button {
                clicked: click_callback,
                &selected_item,

                Popup {
                    is_open: is_popup_open_property2,
    
                    Vertical {
                        Text { text: "Ojej ale popup!!" },
                    }
                }
            }
        };

        let data_holder = DataHolder {
            data: (selected_item)
        };
        data_holder.to_view(None, ViewContext {
            attached_values: context.attached_values,
            children: Box::new(vec![content as Rc<RefCell<dyn ControlObject>>]),
        })
    }
}
