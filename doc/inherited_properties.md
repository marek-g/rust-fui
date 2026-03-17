# Inherited Attached Properties

Inherited attached properties allow you to define values on parent controls that are automatically inherited by all child controls, unless explicitly overridden. This is similar to [attached properties with inheritance in WPF](https://docs.microsoft.com/en-us/dotnet/desktop/wpf/properties/attached-properties-overview).

## Use Cases

Inherited properties are ideal for:

- **Theming/Styling** - Define a color scheme or style at the root level
- **Font settings** - Set font family, size, or weight for entire sections
- **Configuration** - Pass down configuration values without explicit binding
- **Contextual behavior** - Enable/disable features for entire control trees

## How It Works

1. **Registration** - Mark a property type as inherited using `register_inherited!()`
2. **Definition** - Implement `TypeMapKey` trait as usual
3. **Usage** - Use in `ui!` macro with the same syntax as regular attached properties
4. **Inheritance** - Values automatically propagate to children, with override support

## Defining an Inherited Property

```rust
use fui_core::{TypeMapKey, register_inherited};

// Define your property value type
#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub background: [f32; 4],
    pub foreground: [f32; 4],
}

// Implement TypeMapKey
pub struct ColorSchemeKey;

impl TypeMapKey for ColorSchemeKey {
    type Value = ColorScheme;
}

// Register as inherited - this is the key step!
register_inherited!(ColorSchemeKey);
```

## Using Inherited Properties

### Basic Inheritance

```rust
ui! {
    Vertical {
        // Set inherited color scheme at the root
        ColorSchemeKey: ColorScheme {
            primary: [1.0, 0.0, 0.0, 1.0],
            secondary: [0.5, 0.0, 0.0, 1.0],
            background: [1.0, 1.0, 1.0, 1.0],
            foreground: [0.0, 0.0, 0.0, 1.0],
        },
        
        // All children inherit the color scheme
        Button { Text { text: "Red Button" } },
        Button { Text { text: "Another Red Button" } },
        
        NestedVertical {
            // Still inherits the red color scheme
            Button { Text { text: "Also Red" } },
        }
    }
}
```

### Overriding Inherited Values

```rust
ui! {
    Vertical {
        ColorSchemeKey: ColorScheme::dark(),  // Default dark theme
        
        Button { Text { text: "Dark themed" } },
        
        Vertical {
            // Override for this subtree only
            ColorSchemeKey: ColorScheme::light(),
            
            Button { Text { text: "Light themed" } },
            Button { Text { text: "Also light themed" } },
        },
        
        // Back to dark theme (from root)
        Button { Text { text: "Dark themed again" } },
    }
}
```

## Accessing Inherited Properties in Styles

```rust
use fui_core::{ControlContext, TypeMapKey};

// In your style implementation
pub struct ButtonStyle {
    // ...
}

impl Style<Button> for ButtonStyle {
    fn draw(
        &self,
        data: &mut Button,
        context: &ControlContext,
        drawing_context: &mut FuiDrawingContext,
    ) {
        // Access inherited color scheme
        if let Some(color_scheme) = context.get_inherited_values().get::<ColorSchemeKey>() {
            // Use the inherited color scheme for drawing
            let brush = drawing_context.create_solid_brush(color_scheme.primary);
            // ... draw with brush
        }
        
        // Fallback to default if not inherited
        // ...
    }
}
```

## Technical Details

### Copy-on-Write Semantics

Inherited properties use `Rc` (reference counting) for efficient sharing:

- **No deep copy** when inheriting - only `Rc::clone()` which is cheap
- **Immutable values** - prevents accidental modification
- **Copy-on-write** - when a child overrides, only the HashMap is cloned if shared

```rust
// InheritedTypeMap internally uses:
Rc<HashMap<TypeId, Rc<dyn Any>>>
```

### Performance Characteristics

| Operation | Cost |
|-----------|------|
| Reading inherited value | O(1) - HashMap lookup |
| Inheriting (no override) | O(1) - Rc clone |
| Overriding value | O(n) - HashMap clone where n = number of inherited values |
| Memory per control | O(1) - shared Rc references |

### When Values Are Propagated

Inherited values are propagated to children:

1. **At creation time** - When `StyledControl::new()` is called
2. **When parent is set** - For dynamically added children via `set_parent()`
3. **Merge semantics** - Child's own values override parent's values

## Comparison: Local vs Inherited Attached Properties

| Feature | Local Attached | Inherited Attached |
|---------|---------------|-------------------|
| **Scope** | Single control | Control + all descendants |
| **Storage** | `TypeMap` | `InheritedTypeMap` |
| **Mutability** | Mutable | Immutable (via Rc) |
| **Registration** | None required | `register_inherited!()` |
| **Examples** | `Margin`, `Alignment`, `Visible` | `ColorScheme`, `FontSize`, `Theme` |
| **Performance** | Direct access | O(1) with Rc indirection |

## Complete Example

```rust
// ============================================
// Step 1: Define the inherited property
// ============================================
use fui_core::{TypeMapKey, register_inherited, ControlContext};
use fui_drawing::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub primary_color: Color,
    pub font_size: f32,
    pub is_dark: bool,
}

pub struct ThemeKey;

impl TypeMapKey for ThemeKey {
    type Value = Theme;
}

register_inherited!(ThemeKey);

// ============================================
// Step 2: Use in a style
// ============================================
use fui_core::{Style, FuiDrawingContext};

pub struct TextStyle {
    // ...
}

impl Style<Text> for TextStyle {
    fn draw(&self, data: &mut Text, context: &ControlContext, dc: &mut FuiDrawingContext) {
        // Try to get inherited theme
        let theme = context.get_inherited_values()
            .get::<ThemeKey>()
            .unwrap_or(&Theme::default());
        
        // Use theme settings
        let color = if theme.is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };
        
        // Draw text with theme color...
    }
}

// ============================================
// Step 3: Use in UI
// ============================================
use fui_macros::ui;

fn create_view() -> Rc<RefCell<dyn ControlObject>> {
    ui! {
        Vertical {
            // Set theme at root level
            ThemeKey: Theme {
                primary_color: Color::BLUE,
                font_size: 14.0,
                is_dark: true,
            },
            
            // All children inherit the dark theme
            Text { text: "Dark themed text" },
            
            Horizontal {
                Text { text: "Also dark themed" },
                
                // Override for this section
                ThemeKey: Theme {
                    is_dark: false,
                    ..Theme::default()
                },
                
                Text { text: "Light themed text" },
            }
        }
    }
}
```

## Best Practices

1. **Register early** - Call `register_inherited!()` immediately after defining the type
2. **Use descriptive names** - Consider suffixing with `Key` to distinguish from the value type
3. **Keep values small** - Large values increase memory usage when cloned
4. **Make values Clone** - Required for inheritance to work
5. **Document inheritance** - Make it clear in your style documentation which properties are inherited

## Limitations

- **No change notification** - Currently, changes to inherited properties don't automatically propagate. Set them before creating children.
- **Runtime registration** - The `register_inherited!()` call must happen before the property is used in `ui!` macro
- **No partial inheritance** - All inherited properties are merged; you can't selectively inherit

## Future Enhancements

Potential improvements for future versions:

- Change notifications for inherited properties
- Compile-time registration (via const fn or build script)
- Selective inheritance filters
- Debug tools to visualize inheritance tree
