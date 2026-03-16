# View Models

View Models are the core of the MVVM (Model-View-ViewModel) pattern in FUI. They represent the state and business logic of your UI, separated from the visual representation.

## The ViewModel Trait

All view models must implement the `ViewModel` trait:

```rust
pub trait ViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>>;
}
```

The `create_view` method is responsible for creating the visual representation of your view model. It returns a tree of controls that will be rendered in the window.

## Basic Example

Here's a simple view model with a counter:

```rust
use fui_app::*;
use fui_controls::*;
use fui_core::*;
use fui_macros::ui;
use std::rc::Rc;

struct MainViewModel {
    pub counter: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<Self> {
        Rc::new(MainViewModel {
            counter: Property::new(0),
        })
    }

    pub fn increase(self: &Rc<Self>) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(self: &Rc<Self>) {
        self.counter.change(|c| c - 1);
    }
}

impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Vertical {
                Text { 
                    text: (&self.counter, |counter| format!("Count: {}", counter))
                },
                Button {
                    clicked: cb!(self, increase),
                    Text { text: "Increase" }
                },
                Button {
                    clicked: cb!(self, decrease),
                    Text { text: "Decrease" }
                },
            }
        }
    }
}
```

## Key Concepts

### Properties

`Property<T>` is a reactive wrapper around a value. When a property changes, all bindings to it are automatically updated:

```rust
pub counter: Property<i32>,
```

Properties support:
- **Binding**: Connect two properties to sync their values
- **Transformation**: Convert property values for display using closures
- **Change notification**: Automatically update UI when values change

### Methods

View model methods are regular Rust methods that can:
- Modify properties using `.change()`
- Call business logic
- Trigger async operations

Methods are typically called from UI events using the `cb!` macro:

```rust
Button {
    clicked: cb!(self, increase),
    Text { text: "Click me" }
}
```

### The `ui!` Macro

The `ui!` macro is used to declare the visual structure of your view. It supports:
- Layout controls (`Vertical`, `Horizontal`, `Grid`)
- Property bindings
- Event handlers
- Iteration over collections

## Observable Collections

For dynamic lists, use `ObservableVec<T>` which notifies the UI when items are added, removed, or modified:

```rust
struct MainViewModel {
    pub items: ObservableVec<Rc<ItemViewModel>>,
}

impl MainViewModel {
    pub fn add_item(self: &Rc<Self>) {
        self.items.push(Rc::new(ItemViewModel { 
            name: "New Item".to_string() 
        }));
    }
}

impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        ui! {
            Vertical {
                Button {
                    clicked: cb!(self, add_item),
                    Text { text: "Add Item" }
                },
                ScrollViewer {
                    Vertical {
                        for item in self.items {
                            Text { text: &item.name }
                        }
                    }
                }
            }
        }
    }
}
```

## Data Binding

Properties can be bound to each other for automatic synchronization:

```rust
impl ViewModel for MainViewModel {
    fn create_view(self: &Rc<Self>) -> Rc<RefCell<dyn ControlObject>> {
        // Bind two properties together
        self.counter2.bind(&self.counter);
        
        ui! {
            Vertical {
                Text { 
                    text: (&self.counter, |c| format!("Counter: {}", c))
                },
                Text { 
                    text: (&self.counter2, |c| format!("Mirror: {}", c))
                },
            }
        }
    }
}
```

## Setting the ViewModel

Attach your view model to a window using `set_vm`:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let app = Application::new("My App").await?;
    
    let mut window = Window::create(
        WindowOptions::new()
            .with_title("My Window")
            .with_size(800, 600),
    ).await?;
    
    window.set_vm(MainViewModel::new());
    
    app.run().await?;
    Ok(())
}
```

## Best Practices

1. **Use `Rc<Self>`**: View models should be wrapped in `Rc` and methods should take `&Rc<Self>` to allow shared ownership and easy callback creation.

2. **Separate concerns**: Keep business logic in the view model, not in the view. The view should only handle presentation.

3. **Use properties for UI state**: Any state that needs to be reflected in the UI should be wrapped in a `Property`.

4. **Keep views declarative**: The `create_view` method should declare the UI structure, not contain complex logic.

5. **Compose view models**: For complex UIs, compose multiple view models together:

```rust
struct MainViewModel {
    pub header: Rc<HeaderViewModel>,
    pub content: Rc<ContentViewModel>,
}
```

## StringViewModel

FUI provides a built-in `StringViewModel` for simple text representation:

```rust
use fui_controls::StringViewModel;

let text_vm = StringViewModel::new("Hello World");
```

This is useful for items in dropdowns, list views, and other controls that expect view model items.
