# Layout Alignment

There are two attached values that can be added to any control to precisely position control inside dedicated area: `HorizontalAlignment` and `VerticalAlignment`. Both can be set to one of the value:
- `Alignment::Start`
- `Alignment::Center`
- `Alignment::End`
- `Alignment::Stretch`

The default values are `Alignment::Stretch` except for `StackPanel`, `Vertical` and `Horizontal` for which it is `Alignment::Start`.

## Notes for control developers

The `ControlObject` trait implementation for `StyledControl<D>` takes care of respecting these attached values automatically for every control.

To change the default value, you can insert desired value to `ViewContext` during control creation - see`StackPanel::to_view(...)`.
