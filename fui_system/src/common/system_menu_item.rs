use std::cell::RefCell;
use std::rc::Rc;

pub enum SystemMenuItem {
    Separator,
    Text {
        text: String,
        shortcut: Option<String>,
        //icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Option<Box<dyn 'static + FnMut()>>,
        sub_items: Vec<SystemMenuItem>,
    },
}

impl SystemMenuItem {
    pub fn folder(text: &str, sub_items: Vec<SystemMenuItem>) -> Self {
        SystemMenuItem::Text {
            text: text.into(),
            shortcut: None,
            //icon: None,
            callback: None,
            sub_items,
        }
    }

    pub fn simple(text: &str, callback: Option<Box<dyn 'static + FnMut()>>) -> Self {
        SystemMenuItem::Text {
            text: text.into(),
            shortcut: None,
            //icon: None,
            callback,
            sub_items: Vec::new(),
        }
    }

    pub fn full(
        text: &str,
        shortcut: Option<String>,
        //icon: Option<Rc<RefCell<dyn ControlObject>>>,
        callback: Option<Box<dyn 'static + FnMut()>>,
    ) -> Self {
        SystemMenuItem::Text {
            text: text.into(),
            shortcut,
            //icon,
            callback,
            sub_items: Vec::new(),
        }
    }
}
