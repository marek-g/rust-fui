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
            counter: 0.into(),
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
                    text: format!("Count: {}", self.counter.get())
                },
                Button {
                    clicked => self.increase(),
                    Text { text: "Increase" }
                },
                Button {
                    clicked => self.decrease(),
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

### Property Binding in `ui!` Macro

The `ui!` macro supports several ways to bind properties:

#### 1. Automatic Tracking with `.get()` (Recommended)

When you use `.get()` on a property inside an expression, the macro automatically tracks changes and updates the UI:

```rust
Text {
    text: format!("Count: {}", self.counter.get())
}
```

This is the simplest and most readable approach. The macro detects `self.property.get()` and automatically subscribes to property changes.

#### 2. Simple Property Reference

For direct property display without transformation:

```rust
Text {
    text: &self.text  // One-way binding
}
```

#### 3. **Two-Way Binding**

For editable fields that sync with the view model:

```rust
TextBox {
    text: self.text.clone()  // Two-way binding
}
```

### Methods

View model methods are regular Rust methods that can:
- Modify properties using `.change()`
- Call business logic
- Trigger async operations

Methods are typically called from UI events using callback syntax in the `ui!` macro:

```rust
// Synchronous callback without argument
Button {
    clicked => self.increase(),
    Text { text: "Click me" }
}

// Synchronous callback with argument
Button {
    clicked(v) => self.handle_click(v),
    Text { text: "Click me" }
}

// Async callback without argument
Button {
    clicked async => self.save_data(),
    Text { text: "Save" }
}

// Async callback with argument
Button {
    clicked(v) async => self.process_item(v),
    Text { text: "Process" }
}
```

The callback syntax supports:
- **`name => expression`** - Synchronous callback without argument (`Callback<()>`)
- **`name(arg) => expression`** - Synchronous callback with argument (`Callback<T>`)
- **`name async => expression`** - Async callback without argument
- **`name(arg) async => expression`** - Async callback with argument

The macro automatically clones `self` and replaces it with `_self_cloned` inside the closure to prevent memory leaks.

For use outside the `ui!` macro, the `cb!` macro is still available:

```rust
MenuItem::simple("Save", cb!(self, async save_data))
```

### The `ui!` Macro

The `ui!` macro is used to declare the visual structure of your view. It supports:
- Layout controls (`Vertical`, `Horizontal`, `Grid`)
- Property bindings with automatic tracking
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
                    clicked => self.add_item(),
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
                // Automatic tracking with .get()
                Text {
                    text: format!("Counter: {}", self.counter.get())
                },
                Text {
                    text: format!("Mirror: {}", self.counter2.get())
                },
            }
        }
    }
}
```

### Manual Property Binding with `bind_from_expr`

For complex scenarios outside the `ui!` macro, you can use `Property::bind_from_expr`:

```rust
use fui_core::{Property, PropertySubscription};

let first_name = Property::new("John".to_string());
let last_name = Property::new("Doe".to_string());

let full_name = Property::<String>::bind_from_expr(
    {
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        move || format!("{} {}", first_name.get(), last_name.get())
    },
    vec![
        PropertySubscription::from_property(&first_name),
        PropertySubscription::from_property(&last_name),
    ]
);
```

This creates a property that automatically updates when any of the source properties change. However, using `.get()` inside the `ui!` macro is preferred for simplicity.

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
