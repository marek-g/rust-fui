# Events

## HoverChange(bool)

`HoverChange(true)` event is sent to the control when pointer (mouse, stylus, touch) goes into control's area.

`HoverChange(false)` event is sent to the control when pointer leaves control's area.

More than one control can be hovered at the same time if they overlap. The events go to the `hit` control and all its parents.

If there is a captured control, only the captured control can be hovered.
