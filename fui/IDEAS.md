HLists
-----------------------------

TODO:
Use HLists to represent children. Get rid of ControlObject :)

[DONE] One way / two way bindings
-----------------------------

```rust
Text { text: &vm.counter }, // one way binding
Text { text: &mut vm.counter }, // two way binding
Text { text: (&vm.counter, |c| format!("C={}", c) }, // one way binding with converter
Text { text: (&vm.counter, |c| c.to_string(), |s| s.parse().unwrap()) }, // two way binding with converter
```

[DONE] Attached values
-----------------------------

Using TypeMap crate.

Syntax: Attached values starts with upper case.

```rust
Text {
  Row: 5, // attached value
  grid::Column: 5,
  text: "Ala",
}
```

Styles / dependency properties
-----------------------------

I'm not sure we need to have dependency properties for value inheritance.
Maybe the better idea is some kind of style system. E.g.:

```rust
Text # textStyle1, buttonStyle2 {
  Row: 5,
  Column: 5,
  text: "Ala",
}
```

Where `textStyle1` points to `TextDefaultStyle::new(font_size: 10, font_nam="Arial")`.
Style can be inherited down. So maybe it should be a list of styles (not necessairly related with
the current view type).

Maybe style should be something more complex to allow inserting tree nodes like Margin {}. Probably can reuse views as templates for this purpose.


Data templates / Lists:
-----------------------------

Just use the same View trait for defining data templates. The List control will call its create_view method for every item.

```rust
impl View for ItemViewModel {
}

impl View for MainViewModel {
  fn create_view(...) {
    ui!(Vertical {
      List {
        items: Property::new(|vm| vm.items)
      }
    })
  }
}
```

Cons:

- ViewModel can only have one View defined (workaround: create_view to take `style` argument?)

```rust
pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,
    pub children: Vec<Rc<RefCell<ControlObject>>>,

    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

pub trait View {
    fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject>>;
}

impl View for Button {
  fn to_view(self, _children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject> {
    Control::new(self, <ButtonDefaultStyle>::new(), _children)
  }
}

struct MainViewModel {
  pub elements: Observable<BookViewModel>,
}

struct BookViewModel {
  pub name: String,
}

struct Header {
  pub title: String,
}

impl View for Header {
  fn to_view(self, _children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject> {
    ui! {
      Text { title: self.title }
    }
  }
}

impl View for MainViewModel {
  fn to_view(self, _children: Vec<Rc<RefCell<ControlObject>>>) -> Rc<RefCell<ControlObject> {
    let vm = Rc::new(Cell::new(self));
    ui!(
      // <Vertical>::builder().spacing(4).build().to_view(vec![
      //
      //   <Header>::builder().title("Hello!").build().to_view(Vec::new()).
      //   <Text>::builder().text("Hello!").build().to_view(Vec::new()).
      //   <List>::builder().source(...).template(...).build().to_view(Vec::new()),
      //
      // ])
      Vertical {
        spacing: 4,
        Header { title: "Hello!" },
        Text { text: "Text" },
        List {
          source: binding!(vm.elements),
          template: template!(BookViewModel),
        },
      }
    )
  }
}


let main_vm = MainViewModel { elements: ... };
let main_view = main_vm.to_view();

application.run(&main_view);


struct Vertical {
  pub spacing: i32,
}

impl View for Vertical {
  fn to_view(self, children: Vec<Rc<RefCell<ControlObject>>>) -> ViewData {
    Control::new(self, <VerticalDefaultStyle>::new(), children)
  }
}

```
