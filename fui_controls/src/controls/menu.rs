use fui_core::{ControlObject, Property, Style, ViewContext};
use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::TypeMap;

use crate::controls::*;
use crate::layout::*;
use crate::{DataHolder, GestureArea};
use fui_core::*;

pub enum MenuItem {
    Separator,
    Text {
        text: String,
        shortcut: Option<String>,
        icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Callback<()>,
        sub_items: Vec<MenuItem>,
    },
    Custom {
        content: Rc<RefCell<dyn ControlObject>>,
        callback: Callback<()>,
        sub_items: Vec<MenuItem>,
    },
}

impl MenuItem {
    pub fn folder(text: &str, sub_items: Vec<MenuItem>) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            icon: None,
            callback: Callback::empty(),
            sub_items,
        }
    }

    pub fn simple(text: &str, callback: Callback<()>) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            icon: None,
            callback,
            sub_items: Vec::new(),
        }
    }

    pub fn full(
        text: &str,
        shortcut: Option<String>,
        icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Callback<()>,
    ) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut,
            icon,
            callback,
            sub_items: Vec::new(),
        }
    }
}

#[derive(TypedBuilder)]
pub struct Menu {
    #[builder(default = Orientation::Horizontal)]
    pub orientation: Orientation,

    pub items: Vec<MenuItem>,
}

impl Menu {
    pub fn to_view(
        self,
        _style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let content: Vec<_> = self.items.into_iter().map(|item| item.to_view()).collect();

        let menu = ui! {
            StackPanel {
                orientation: self.orientation,

                content,
            }
        };

        let data_holder = DataHolder { data: () };
        data_holder.to_view(
            None,
            ViewContext {
                attached_values: context.attached_values,
                children: Children::SingleStatic(menu),
            },
        )
    }
}

impl MenuItem {
    pub fn to_view(self) -> Rc<RefCell<dyn ControlObject>> {
        match self {
            MenuItem::Separator => {
                let separator: Rc<RefCell<dyn ControlObject>> = ui! {
                    Text { text: "|" }
                };
                separator
            }

            MenuItem::Text {
                text,
                shortcut,
                icon,
                callback,
                sub_items,
            } => {
                let title = ui! {
                    Text { Style: Hover {}, text: text }
                };
                if sub_items.len() == 0 {
                    return title;
                }

                let mut is_open_prop = Property::new(false);
                let mut is_open_prop2 = Property::binded_two_way(&mut is_open_prop);
                let mut tap_down_callback: Callback<()> = Callback::empty();
                tap_down_callback.set(move |_| {
                    is_open_prop2.set(true);
                });

                ui! {
                    GestureArea {
                        tap_down: tap_down_callback,

                        title,

                        Popup {
                            is_open: is_open_prop,
                            placement: PopupPlacement::BelowOrAboveParent,

                            Text { text: "This is popup!" }
                        }
                    }
                }
            }

            MenuItem::Custom {
                content,
                callback,
                sub_items,
            } => content,
        }
    }
}
