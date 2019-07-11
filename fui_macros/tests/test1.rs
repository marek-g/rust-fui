use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;

pub trait ControlObject {
    fn draw(&mut self) -> String;
}

pub trait View {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>>;
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
pub struct Horizontal {
    #[builder(default = 0)]
    pub spacing: i32,
}

impl View for Horizontal {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>> {
        Control::new(self, HorizontalDefaultStyle::new(), children)
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
        format!("Horizontal({})", data.spacing)
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Button {}

impl View for Button {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>> {
        Control::new(self, ButtonDefaultStyle::new(), children)
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
pub struct Text {
    pub text: String,
}

impl View for Text {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>> {
        Control::new(self, TextDefaultStyle::new(), children)
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
        format!("Text(\"{}\")", data.text)
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
