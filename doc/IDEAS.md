TODO:

- I have found two interesting projects to investigate:
  - https://github.com/Pauan/rust-signals
  - https://github.com/DioxusLabs/dioxus
  - https://github.com/ogoffart/propertybindings-rs (QML inspired macros)
  - https://github.com/d9n/pebl/tree/master/src (properties, bindings)
  - https://github.com/sixtyfpsui/sixtyfps
  - https://github.com/mrDIMAS/rg3d-ui

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

Creating new attached value:

```rust
struct NewValue;
impl typemap::Key for NewValue { type Value = i32; }
```

Properties
-----------------------------

- binding from many (at least two) properties with converter
- read only property

Layout
-----------------------------

Flutter: https://medium.com/saugo360/flutters-rendering-engine-a-tutorial-part-1-e9eff68b825d
Flutter Row src: https://github.com/flutter/flutter/blob/b712a172f9/packages/flutter/lib/src/widgets/basic.dart#L4015
Flutter containers: https://flutter.dev/docs/development/ui/widgets/layout
layout(): https://github.com/flutter/flutter/blob/b712a172f9/packages/flutter/lib/src/rendering/object.dart#L1534

WPF MSDN: https://docs.microsoft.com/en-us/dotnet/api/system.windows.frameworkelement.measureoverride?redirectedfrom=MSDN&view=netframework-4.8#System_Windows_FrameworkElement_MeasureOverride_System_Windows_Size_
WPF src: https://github.com/dotnet/wpf/tree/master/src/Microsoft.DotNet.Wpf/src/PresentationFramework/System/Windows/Controls

MeasureCore() / ArrangeCore(): https://github.com/dotnet/wpf/blob/master/src/Microsoft.DotNet.Wpf/src/PresentationFramework/System/Windows/FrameworkElement.cs

Flutter's child.layout(BoxConstraints) == WPF's MeasureOverride(Size).

Flutter:
performLayout() {
  child.layout(..); <- MeasureOverride <- GetDesiredSize()
  child.parentData.offset = ...; <- ArrangeOverride <- SetSize()
}


measure(size) {
  child.measure(...);
  child.set_offset(...)
}

measure(size) {
  self.preffered_size = self.get_preffered_size(size)
}

set_rect() {
  
}



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
    pub children: Box<dyn ObservableCollection<Rc<RefCell<dyn ControlObject>>>>,

    parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

pub trait View {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>>;
}

impl View for Button {
  fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject> {
    Control::new(self, <ButtonDefaultStyle>::new(), context.children)
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
  fn to_view(self, _context: ViewContext) -> Rc<RefCell<ControlObject> {
    ui! {
      Text { title: self.title }
    }
  }
}

impl View for MainViewModel {
  fn to_view(self, _context: ViewContext) -> Rc<RefCell<ControlObject> {
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
  fn to_view(self, context: ViewContext) -> ViewData {
    Control::new(self, <VerticalDefaultStyle>::new(), context.children)
  }
}

```

```rust
ui!(
    Vertical {
        // Collection<ItemViewModel>(&self.items),

        // Collection(&self.items), // take type from items collection

        &self.items, // simplest syntax

        /*for (index, item) in &self.items {
            Horizontal {
                Text { text: &item.name },
                Text { text: (&item.number, |n| format!(" - {}", n)) },
            }
        },*/
    }
)
```

```rust
// Vec<Rc<RefCell<ControlObject>>>

trait ChildrenCollection {
  fn len(&self) -> usize;
  fn create_view(&self, index: usize) -> Rc<RefCell<ControlObject>>;
}

struct DynamicCollection {
  pub items: Rc<RefCell<ObservableCollection>>,
}

impl ChildrenCollection for DynamicCollection {
  fn len(&self) -> { self.items }

  fn create_view(&self, index: usize) -> Rc<RefCell<ObservableCollection>> {
    let item = &items.borrow_mut().get(index);

    ui!(
      Horizontal {
          Text { text: &item.name },
          Text { text: (&item.number, |n| format!(" - {}", n)) },
      }
    )
  }
}

```


```rust

struct ItemViewModel {
  pub title: String,
}

impl View for ItemViewModel {
  fn to_view(self, _context: ViewContext) -> Rc<RefCell<ControlObject> {
    ui! {
      Text { title: self.title }
    }
  }
}

struct MainViewModel {
  pub elements: Rc<RefCell<Vec<Rc<RefCell<ItemViewModel>>>>>,

  pub elements: Collection<Rc<RefCell<ItemViewModel>>>,

  pub elements: Collection<ItemViewModel>,

  pub elements: Vec<ItemViewModel>,
}

pub struct ChildrenCollection<T: View> {
  pub sub_collection: Rc<RefCell<Vec<T>>>
}

pub enum ChildrenCollectionItem {
  Control(Rc<RefCell<dyn ControlObject>>),
  Views(Vec<Rc<RefCell<dyn View>>),
}

children: Vec<ChildrenCollectionItem>

```
