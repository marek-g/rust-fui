# Layout

## Grid

Provides a way of arranging items in a grid (rectangle area with columns and rows).

_Note: The algorithm is based on WPF's implementation and has a very similar behavior._

### Example

```rust
ui! {
    Grid {
        columns: 2,

        // first row
        Text { text: "Label 1" },
        Button { Text { text: "Button 1" } },

        // second row
        Text { text: "Label 2" },
        Button { Text { text: "Button 2" } },
    }
}
```

### Details

You can specify constant number of `rows` or `columns`. If you specify `columns` the children will be auto-positioned horizontally (and vertically if you specify only number of `rows`).

If you specify none of the `columns` and `rows`, the grid will put all the children in the same cell (one over the other):

```rust
ui! {
    Grid {
        Image { ... },
        Text { text: "Text over the image!" },
    }
}
```

You can also position controls manually by using attached values: `Row` and `Column`. The other respected attached values are: `RowSpan` and `ColumnSpan`.

```rust
ui! {
    Grid {
        // first row
        Text {
            Row: 0, Column: 0, ColumnSpan: 2,
            text: "Label"
        },

        // second row
        Button {
            Row: 1, Column: 0,
            Text { text: "Button 1" }
        },
        Button {
            Row: 1, Column: 1,
            Text { text: "Button 2" }
        },
    }
}
```

There is a `Length` value assigned to every row and every column. `Length` is an enum with the following values:

* `Length::Auto` - the size of the column/row is the minimum size that fits all the children,
* `Length::Exact(value)` - user specified size
* `Length::Fill(value)` - fill the available space with a specified flex factor (flex factor express a weighted proportion of available space)

Default width and height of the cell is `Length::Fill(1.0f32)` but you can change this by specifing `default_width` and `default_height` attributes. Additionally you have control of width and height of individual columns and rows by specyfing `widths` and `heights` attributes.

You can also specify constraints for minimal and maximal row and column sizes with `default_min_width`, `default_max_width`,`default_min_height`, `default_max_height`, `min_widths`, `max_widths`, `min_heights` and `max_heights` attributes.

```rust
ui! {
    Grid {
        columns: 4,

        default_width: Length::Auto,
        default_height: Length::Exact(100.0f32),

        widths: vec![(0, Length::Exact(100.0f32)),
            (1, Length::Fill(1.0f32))],

        default_min_height: 200.0f32,
        min_heights: vec![(3, 100.0f32)],

        // ...
    }
}
```
