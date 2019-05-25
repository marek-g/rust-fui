use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ControlObject {
}

#[derive(Debug)]
pub struct HorizontalProperties {
    pub spacing: i32,
}

#[derive(Debug)]
pub struct Horizontal {
    pub properties: HorizontalProperties
}

impl Horizontal {
    pub fn new(properties: HorizontalProperties) -> Self {
        Horizontal {
            properties: properties,
        }
    }
}

#[derive(Debug)]
pub struct ButtonProperties {
}

#[derive(Debug)]
pub struct Button {
    pub properties: ButtonProperties
}

impl Button {
    pub fn new(properties: ButtonProperties) -> Self {
        Button {
            properties: properties,
        }
    }
}

#[derive(Debug)]
pub struct TextProperties {
    pub text: String,
}

#[derive(Debug)]
pub struct Text {
    pub properties: TextProperties
}

impl Text {
    pub fn new(properties: TextProperties) -> Self {
        Text {
            properties: properties,
        }
    }
}

#[test]
fn test1() {
    let a = ui!(
        Horizontal {
            spacing: 5,
            
            Button { Text { text: "Button".to_string() } },
            //Text { text: "Label".to_string() }
        }
    );
    /*let a = ui!(
        Horizontal {
            spacing: 5,
        }
    );*/
    //println!("{:?}", a);
}
