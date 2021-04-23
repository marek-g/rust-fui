use crate::common::callback_helper::RawCallback;
use std::cell::RefCell;
use std::rc::Rc;

pub enum MenuItem {
    Separator,
    Text {
        text: String,
        shortcut: Option<String>,
        //icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Option<RawCallback>,
        sub_items: Vec<MenuItem>,
    },
}

impl MenuItem {
    pub fn folder(text: &str, sub_items: Vec<MenuItem>) -> Self {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            //icon: None,
            callback: None,
            sub_items,
        }
    }

    pub fn simple<F>(text: &str, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        MenuItem::Text {
            text: text.into(),
            shortcut: None,
            //icon: None,
            callback: Some(RawCallback::new(callback)),
            sub_items: Vec::new(),
        }
    }

    pub fn full<F>(
        text: &str,
        shortcut: Option<String>,
        //icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: F,
    ) -> Self
    where
        F: FnMut() + 'static,
    {
        MenuItem::Text {
            text: text.into(),
            shortcut,
            //icon,
            callback: Some(RawCallback::new(callback)),
            sub_items: Vec::new(),
        }
    }
}
