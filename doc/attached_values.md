# Attached Values

Attached values are a mechanism to extend controls with additional properties that can be read by layout containers or other controls. They allow you to customize the behavior of parent containers without modifying the control's core implementation.

## What are Attached Values

Attached values are key-value pairs that can be attached to any control. They are primarily used by layout containers (like `Grid`, `StackPanel`) to control the layout behavior of their children.

## Using Attached Values

### Grid Layout

The `Grid` layout uses several attached values to position and size child controls:

- `Row` - specifies the row index for the child control
- `Column` - specifies the column index for the child control
- `RowSpan` - specifies how many rows the child should span
- `ColumnSpan` - specifies how many columns the child should span

```rust
ui! {
    Grid {
        columns: 3,

        Text {
            Row: 0, Column: 0,
            text: "Header spanning 3 columns"
        },

        Button {
            Row: 1, Column: 0,
            Text { text: "Button 1" }
        },

        Button {
            Row: 1, Column: 1, ColumnSpan: 2,
            Text { text: "Button 2 (spans 2 columns)" }
        },

        Text {
            Row: 2, Column: 2,
            text: "Bottom right"
        },
    }
}
```

### StackPanel Layout

The `StackPanel` uses the `Grow` attached value to control how space is distributed among children:

- `Grow` - controls how much space a child should receive when extra space is available. Can be:
  - `Length::Auto` - child keeps its natural size
  - `Length::Exact(value)` - child gets the exact specified size
  - `Length::Fill(factor)` - child fills available space proportionally to its factor

```rust
ui! {
    Horizontal {
        Text {
            Grow: Length::Auto,
            text: "Fixed size"
        },

        Text {
            Grow: Length::Fill(1.0),
            text: "Fills 1 part"
        },

        Text {
            Grow: Length::Fill(2.0),
            text: "Fills 2 parts (twice the space)"
        },
    }
}
```

## Creating Custom Attached Values

Control developers can define custom attached values by implementing the `TypeMapKey` trait:

```rust
use fui_core::{TypeMap, TypeMapKey};

pub struct MyAttachedValue;
impl TypeMapKey for MyAttachedValue {
    type Value = i32;
}
```

Then use it in a control:

```rust
ui! {
    SomeLayout {
        Text {
            MyAttachedValue: 42,
            text: "Text with custom attached value"
        },
    }
}
```

To read the attached value in a layout container:

```rust
let value = ctx.get_attached_value::<MyAttachedValue>();
if let Some(val) = value.as_deref() {
    // Use the value
    println!("Attached value: {}", *val);
}
```

## Notes for Control Developers

The `ControlObject` trait implementation for `StyledControl<D>` automatically handles most attached values like `Margin`, `HorizontalAlignment`, `VerticalAlignment`, and `Visible`. Layout containers like `Grid` and `StackPanel` implement their own logic to read and respect their specific attached values.

To change the default value of an attached value, you can insert the desired value into the `ViewContext` during control creation - see `StackPanel::to_view(...)`.

---

# Attached Inherited Values

Attached inherited values allow child controls to access values from their parent controls in the control hierarchy. This is useful for sharing state or configuration across a subtree of controls.

## How Inheritance Works

When a control requests an attached value that doesn't exist locally, the system automatically searches parent controls in the hierarchy until it find the value or reaches the root.

For example, consider a menu system where all menu items need access to the parent menu's state:

```rust
// Parent menu sets some state as an attached value
let menu_data = ActiveMenu::new(...);
context.attached_values.insert::<ActiveMenu>(menu_data);

// Child menu items can access it without explicit passing
let menu_data = ctx.get_inherited_value::<ActiveMenu>();
```

## Using get_inherited_value

The `ControlContext` provides the `get_inherited_value<K>()` method to search for values in the parent hierarchy:

```rust
// Try to get the value from local attached values first
// If not found, search parents automatically
if let Some(value) = ctx.get_inherited_value::<MyAttachedValue>() {
    // Use the inherited value
}
```

The method:
1. First checks local attached values
2. If not found, traverses parent controls upward
3. Caches found values for performance
4. Returns `None` if the value is not found in the hierarchy

## Example: Menu System

The `Menu` control uses inherited values to share menu state:

```rust
impl Menu {
    pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<dyn ControlObject> {
        CompositeControl::new(context, move |ctx: &ControlContext| {
            // Get the ActiveMenu from parent (MenuBar or parent Menu)
            let menu_data = ctx.get_inherited_value::<ActiveMenu>();
            
            // Pass menu_data to child menu items
            menu_impl(ctx.get_children(), menu_data, true, TypeMap::new())
        })
    }
}
```

## Notes for Control Developers

- Inherited values are read-only in child controls. To share mutable state, use `Property<T>` or similar reactive types.
- The inheritance is automatic - no special setup required. Just ensure your controls are properly parented in the control hierarchy.
- Inherited values are cached for performance, so changes to parent values won't automatically propagate to children. Re-read when needed.
