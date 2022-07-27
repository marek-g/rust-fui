# Fonts

Fonts are identified by its `name` (string) and `size` (u8). For example, there are `font_name` and `font_size` style attributes for `Text` control:

Example:

TODO:
```rust
Text {
    Style: Default { font_name: "Arial", font_size: 20 },
}
```

Font name can be:

- a string ended with `.ttf` like `"assets/OpenSans-Regular.ttf"` which points to the file in the file system
- `sans-serif` for default sans-serif font, `sans-serif bold` for default sans-serif bold font, `monospace` for default monospace font or `monospace bold` for default monospace bold font
- TODO: a list of font families separated with `,` like `Times New Roman, Arial, serif` that the system will be searched for in provided order; the `serif`, `sans-serif`, `monospace`, `cursive`, `fantasy` are fallback names
