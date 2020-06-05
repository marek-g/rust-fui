use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::{Key, TypeMap};

pub trait ObservableCollection<T: 'static + Clone> {
    fn iter1<'a>(&'a self) -> ::std::slice::Iter<'a, T>;
}

///
/// ObservableCollection for Vec.
/// 
impl ObservableCollection<Rc<RefCell<dyn ControlObject>>> for Vec<Rc<RefCell<dyn ControlObject>>> {
    fn iter1<'a>(&'a self) -> ::std::slice::Iter<'a, Rc<RefCell<dyn ControlObject>>> {
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
    children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
}

pub trait Style<D> {
    fn draw(&self, data: &mut D) -> String;
}

pub struct StyledControl<D> {
    pub data: D,
    pub style: Box<dyn Style<D>>,
    pub attached_values: TypeMap,
    pub children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
}

impl<D: 'static> StyledControl<D> {
    pub fn new(
        data: D,
        style: Box<dyn Style<D>>,
        attached_values: TypeMap,
        children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(StyledControl {
            data: data,
            attached_values: attached_values,
            style,
            children: children,
        }))
    }
}

impl<D: 'static> ControlObject for StyledControl<D> {
    fn draw(&mut self) -> String {
        let name = self.style.draw(&mut self.data);
        let mut attached_values = "".to_string();
        if let Some(row_attached_value) = self.attached_values.get::<Row>() {
            attached_values += &format!(".Row({})", row_attached_value);
        }

        let children = {
            let vec: Vec<String> = self
                .children
                .iter1()
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

impl Horizontal {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultHorizontalStyle::new(DefaultHorizontalStyleParams::builder().build()))
            }),
            context.attached_values,
            context.children,
        )
    }
}

#[derive(TypedBuilder)]
pub struct DefaultHorizontalStyleParams {}

pub struct DefaultHorizontalStyle {}

impl DefaultHorizontalStyle {
    pub fn new(_params: DefaultHorizontalStyleParams) -> Self {
        DefaultHorizontalStyle {}
    }
}

impl Style<Horizontal> for DefaultHorizontalStyle {
    fn draw(&self, data: &mut Horizontal) -> String {
        format!("Horizontal({})", data.spacing)
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Button {}

impl Button {
    pub fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultButtonStyle::new(DefaultButtonStyleParams::builder().build()))
            }),
            context.attached_values,
            context.children,
        )
    }
}

#[derive(TypedBuilder)]
pub struct DefaultButtonStyleParams {}

pub struct DefaultButtonStyle {}

impl DefaultButtonStyle {
    pub fn new(_params: DefaultButtonStyleParams) -> Self {
        DefaultButtonStyle {}
    }
}

impl Style<Button> for DefaultButtonStyle {
    fn draw(&self, _data: &mut Button) -> String {
        "Button".to_string()
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Text {
    pub text: String,
}

impl Text {
    fn to_view(self, style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultTextStyle::new(DefaultTextStyleParams::builder().build()))
            }),
            context.attached_values,
            context.children,
        )
    }
}

#[derive(TypedBuilder)]
pub struct DefaultTextStyleParams {}

pub struct DefaultTextStyle {}

impl DefaultTextStyle {
    pub fn new(_params: DefaultTextStyleParams) -> Self {
        DefaultTextStyle {}
    }
}

impl Style<Text> for DefaultTextStyle {
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

    let mut control: std::cell::RefMut<dyn ControlObject> = control.borrow_mut();
    assert_eq!(
        "Horizontal(4).Row(1){Button{Text(\"Button\"){}},Text(\"Label\"){}}",
        control.draw()
    );

    //println!("{}", control.draw());
    //println!("{:?}", control);
}
