use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::{Key, TypeMap};

pub trait ChildrenSource {
    fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>>;
}

impl<'a> IntoIterator for &'a ChildrenSource {
    type Item = &'a Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>>;

    fn into_iter(self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
        self.iter()
    }
}

///
/// ChildrenSource for Vec.
/// 
impl ChildrenSource for Vec<Rc<RefCell<dyn ControlObject>>> {
    fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
        self.iter()
    }
}

// attached value Row of type i32
struct Row;
impl Key for Row {
    type Value = i32;
}

pub trait ControlObject {
    fn draw(&mut self) -> String;
}

pub struct ViewContext {
    attached_values: TypeMap,
    children: Box<dyn ChildrenSource>,
}

pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}

pub trait Style<D> {
    fn draw(&self, data: &mut D) -> String;
}

pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,
    pub attached_values: TypeMap,
    pub children: Box<ChildrenSource>,
}

impl<D: 'static> Control<D> {
    pub fn new<S: 'static + Style<D>>(
        data: D,
        style: S,
        attached_values: TypeMap,
        children: Box<ChildrenSource>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Control {
            data: data,
            attached_values: attached_values,
            style: Box::new(style),
            children: children,
        }))
    }
}

impl<D: 'static> ControlObject for Control<D> {
    fn draw(&mut self) -> String {
        let name = self.style.draw(&mut self.data);
        let mut attached_values = "".to_string();
        if let Some(row_attached_value) = self.attached_values.get::<Row>() {
            attached_values += &format!(".Row({})", row_attached_value);
        }

        let children = {
            let vec: Vec<String> = self
                .children
                .iter()
                .map(|c| c.borrow_mut().draw())
                .collect();
            vec.join(",")
        };

        name + &attached_values + "{" + &children + "}"
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Horizontal {
    #[builder(default = 0)]
    pub spacing: i32,
}

impl View for Horizontal {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(
            self,
            HorizontalDefaultStyle::new(),
            context.attached_values,
            context.children,
        )
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
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(
            self,
            ButtonDefaultStyle::new(),
            context.attached_values,
            context.children,
        )
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
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(
            self,
            TextDefaultStyle::new(),
            context.attached_values,
            context.children,
        )
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
            Row: 1,
            spacing: 4,
            Button { Text { text: "Button".to_string() } },
            Text { text: "Label".to_string() }
        }
    );

    let mut control: std::cell::RefMut<ControlObject> = control.borrow_mut();
    assert_eq!(
        "Horizontal(4).Row(1){Button{Text(\"Button\"){}},Text(\"Label\"){}}",
        control.draw()
    );

    //println!("{}", control.draw());
    //println!("{:?}", control);
}
