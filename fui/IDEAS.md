HLists
=============================

TODO:
Use HLists to represent children. Get rid of ControlObject :)

Data templates / Lists:
=============================

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


SWIFT
=============================

```rust
struct ContentView;

impl View for ContentView {
  var body: some View {
    Text("Hello World")
  }
}

struct RoomCell {
  pub room: Room
}

impl View for RoomCell {
  fn create_view(room_cell: &Rc<RefCell<RoomCell>>) -> ViewData {
  }
}
```

Can reuse RoomCell in other View implementations!

TODO:

- ViewData returns View (what about bindings? reference counted bindings?)



What if use View instead of `Rc<RefCell<ControlObject>>` ?
=============================

? not possible ? how to call methods on ControlObject then?


What if use ViewData instead of `Rc<RefCell<ControlObject>>` ?
=============================

Where:

- as control's children
- returned by View::create_view() - as it is now

Pros:

- can create reusable components like in Swift

What about data templates?

- can have template controls like `List<TemplateType : View>`

What should be an argument for create_view()?

```rust
pub struct ViewData {
    pub control: Rc<RefCell<ControlObject>>,
    pub bindings: Vec<EventSubscription>,
    //pub attached_properties: Vec<(String, String)>,
}

pub trait View {
    fn to_view(self, children: Vec<ViewData>) -> ViewData;
}

impl View for Button {
  fn to_view(self, _children: Vec<ViewData>) -> ViewData {
    ViewData { control: Rc::new(Cell::new(self)), bindings: Vec::new() }
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
  fn to_view(self, _children: Vec<ViewData>) -> ViewData {
    ui! {
      Text { title: self.title }
    }
  }
}

impl View for MainViewModel {
  fn to_view(self, _children: Vec<ViewData>) -> ViewData {
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
  pub children: Vec<ViewData>,
}

impl View for Vertical {
  fn to_view(self, children: Vec<ViewData>) -> ViewData {
    let (control, bindings) = Control::new(self, <ButtonDefaultStyle>::new(), children);
    ViewData { control: Rc::new(Cell::new(control)), bindings }
  }
}

```


Next try:

```rust
pub struct Control<D> {
    pub data: D,
    pub style: Box<Style<D>>,
    pub children: Vec<Rc<RefCell<ControlObject>>>,

    pub bindings: Vec<EventSubscription>,

    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

pub trait View {
    fn to_view(self, children: Vec<ViewData>) -> Rc<RefCell<ControlObject>>;
}

impl View for ButtonProperties {
  fn to_view(self, _children: Vec<ViewData>) -> Rc<RefCell<ControlObject> {
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
  fn to_view(self, _children: Vec<ViewData>) -> Rc<RefCell<ControlObject> {
    ui! {
      Text { title: self.title }
    }
  }
}

impl View for MainViewModel {
  fn to_view(self, _children: Vec<ViewData>) -> Rc<RefCell<ControlObject> {
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
  pub children: Vec<ViewData>,
}

impl View for Vertical {
  fn to_view(self, children: Vec<ViewData>) -> ViewData {
    let (control, bindings) = Control::new(self, <ButtonDefaultStyle>::new(), children);
    ViewData { control: Rc::new(Cell::new(control)), bindings }
  }
}

```
