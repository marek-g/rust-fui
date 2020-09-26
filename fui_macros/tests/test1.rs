use fui_macros::ui;
use std::cell::RefCell;
use std::rc::Rc;
use typed_builder::TypedBuilder;
use typemap::{Key, TypeMap};

/// Children collection of a control.
///
/// The simplified version to handle only static children
/// for the test purposes.
pub enum Children {
    /// The collection has no items.
    None,

    /// The collection has a single child.
    SingleStatic(Rc<RefCell<dyn ControlObject>>),

    /// The collection is a list of controls.
    MultipleStatic(Vec<Rc<RefCell<dyn ControlObject>>>),
}

impl Children {
    /// Creates an empty children collection.
    pub fn empty() -> Self {
        Children::None
    }

    /// Constructs Children collection from
    /// vector of Children collections.
    pub fn from(children_vec: Vec<Children>) -> Self {
        let mut static_children: Vec<Rc<RefCell<dyn ControlObject>>> = Vec::new();

        for next in children_vec {
            match next {
                Children::None => (),
                Children::SingleStatic(item) => {
                    static_children.push(item);
                }
                Children::MultipleStatic(mut items) => {
                    static_children.append(&mut items);
                }
            }
        }

        if static_children.len() == 1 {
            Children::SingleStatic(static_children.into_iter().next().unwrap())
        } else if static_children.len() > 1 {
            Children::MultipleStatic(static_children)
        } else {
            Children::None
        }
    }

    /// Returns number of controls in the children collection.
    pub fn len(&self) -> usize {
        match self {
            Children::None => 0,
            Children::SingleStatic(_) => 1,
            Children::MultipleStatic(x) => x.len(),
        }
    }

    /// Tries to get Rc reference to the control at the `index` position.
    pub fn get(&self, index: usize) -> Option<Rc<RefCell<dyn ControlObject>>> {
        match self {
            Children::None => None,
            Children::SingleStatic(x) => {
                if index == 0 {
                    Some(x.clone())
                } else {
                    None
                }
            }
            Children::MultipleStatic(x) => x.get(index).map(|el| el.clone()),
        }
    }
}

/// Converts a single control to Children collection.
impl From<Rc<RefCell<dyn ControlObject>>> for Children {
    fn from(item: Rc<RefCell<dyn ControlObject>>) -> Children {
        Children::SingleStatic(item)
    }
}

/// Converts a single control to ChildEntry.
impl<T: 'static + ControlObject> From<Rc<RefCell<T>>> for Children {
    fn from(item: Rc<RefCell<T>>) -> Children {
        Children::SingleStatic(item)
    }
}

pub struct ChildrenIterator<'a> {
    source: &'a Children,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = Rc<RefCell<dyn ControlObject>>;

    fn next(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.pos < self.len {
            self.pos += 1;
            self.source.get(self.pos - 1)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ChildrenIterator<'a> {
    fn next_back(&mut self) -> Option<Rc<RefCell<dyn ControlObject>>> {
        if self.len > self.pos {
            self.len -= 1;
            self.source.get(self.len)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = Rc<RefCell<dyn ControlObject>>;
    type IntoIter = ChildrenIterator<'a>;

    fn into_iter(self) -> ChildrenIterator<'a> {
        ChildrenIterator {
            source: self,
            pos: 0,
            len: self.len(),
        }
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
    children: Children,
}

pub trait Style<D> {
    fn draw(&self, data: &mut D) -> String;
}

pub struct StyledControl<D> {
    pub data: D,
    pub style: Box<dyn Style<D>>,
    pub attached_values: TypeMap,
    pub children: Children,
}

impl<D: 'static> StyledControl<D> {
    pub fn new(
        data: D,
        style: Box<dyn Style<D>>,
        attached_values: TypeMap,
        children: Children,
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
            let vec: Vec<String> = (&self.children)
                .into_iter()
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
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultHorizontalStyle::new(
                    DefaultHorizontalStyleParams::builder().build(),
                ))
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
    pub fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultButtonStyle::new(
                    DefaultButtonStyleParams::builder().build(),
                ))
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
    fn to_view(
        self,
        style: Option<Box<dyn Style<Self>>>,
        context: ViewContext,
    ) -> Rc<RefCell<StyledControl<Self>>> {
        StyledControl::new(
            self,
            style.unwrap_or_else(|| {
                Box::new(DefaultTextStyle::new(
                    DefaultTextStyleParams::builder().build(),
                ))
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
