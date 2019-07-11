use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;

pub trait ControlObject {
    fn draw(&mut self) -> String;
}

pub trait Style<D> {
    fn draw(&self, data: &mut D) -> String;
}

pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,
    pub children: Vec<Rc<RefCell<ControlObject>>>,
}

impl<D: 'static> Control<D> {
    pub fn new<S: 'static + Style<D>>(
        data: D,
        style: S,
        children: Vec<Rc<RefCell<ControlObject>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Control {
            data: data,
            style: Box::new(style),
            children: children,
        }))
    }
}

impl<D: 'static> ControlObject for Control<D>
{
    fn draw(&mut self) -> String {
        let name = self.style.draw(&mut self.data);
        let children = if self.children.len() > 0 {
            let vec: Vec<String> = self.children.iter().map(|c| c.borrow_mut().draw()).collect();
            vec.join(",")
        } else {
            "".to_string()
        };

        name + "{" + &children + "}"
    }
}

#[derive(Debug, TypedBuilder)]
pub struct HorizontalProperties {
    #[builder(default = 0)]
    pub spacing: i32,
}

#[derive(Debug)]
pub struct Horizontal {
    pub properties: HorizontalProperties,
}

impl Horizontal {
    pub fn new(properties: HorizontalProperties) -> Self {
        Horizontal {
            properties: properties,
        }
    }
}

pub struct HorizontalDefaultStyle {}

impl HorizontalDefaultStyle {
    pub fn new() -> Self {
        HorizontalDefaultStyle {}
    }
}

impl Style<Horizontal> for HorizontalDefaultStyle {
    fn draw(&self, data: &mut Horizontal) -> String {
        format!("Horizontal({})", data.properties.spacing)
    }
}

#[derive(Debug, TypedBuilder)]
pub struct ButtonProperties {}

#[derive(Debug)]
pub struct Button {
    pub properties: ButtonProperties,
}

impl Button {
    pub fn new(properties: ButtonProperties) -> Self {
        Button {
            properties: properties,
        }
    }
}

pub struct ButtonDefaultStyle {}

impl ButtonDefaultStyle {
    pub fn new() -> Self {
        ButtonDefaultStyle {}
    }
}

impl Style<Button> for ButtonDefaultStyle {
    fn draw(&self, _data: &mut Button) -> String {
        "Button".to_string()
    }
}

#[derive(Debug, TypedBuilder)]
pub struct TextProperties {
    pub text: String,
}

#[derive(Debug)]
pub struct Text {
    pub properties: TextProperties,
}

impl Text {
    pub fn new(properties: TextProperties) -> Self {
        Text {
            properties: properties,
        }
    }
}

pub struct TextDefaultStyle {}

impl TextDefaultStyle {
    pub fn new() -> Self {
        TextDefaultStyle {}
    }
}

impl Style<Text> for TextDefaultStyle {
    fn draw(&self, data: &mut Text) -> String {
        format!("Text(\"{}\")", data.properties.text)
    }
}

#[test]
fn test1() {
    let control = ui!(
        Horizontal {
            spacing: 4,
            Button { Text { text: "Button".to_string() } },
            Text { text: "Label".to_string() }
        }
    );

    let mut control: std::cell::RefMut<ControlObject> = control.borrow_mut();
    assert_eq!("Horizontal(4){Button{Text(\"Button\"){}},Text(\"Label\"){}}", control.draw());

    //println!("{}", control.draw());
    //println!("{:?}", control);
}
