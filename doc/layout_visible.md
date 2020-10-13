# Layout Margin

The `Visble` attached value can be added to any control to control its visibility. The value type is `Property<bool>`. If `true`, the control is visible. If `false`, the control is not visible.

## Notes for control developers

The `ControlObject` trait implementation for `StyledControl<D>` takes care of respecting this attached value automatically for every control.
