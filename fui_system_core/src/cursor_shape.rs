/// This enum type defines the various cursors that can be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CursorShape {
    /// The standard arrow cursor.
    ArrowCursor,

    /// An arrow pointing upwards toward the top of the screen.
    UpArrowCursor,

    /// A crosshair cursor, typically used to help the user accurately select a point on the screen.
    CrossCursor,

    /// An hourglass or watch cursor, usually shown during operations that prevent the user from interacting with the application.
    WaitCursor,

    /// A caret or ibeam cursor, indicating that a widget can accept and display text input.
    IBeamCursor,

    /// A cursor used for elements that are used to vertically resize top-level windows.
    SizeVerCursor,

    /// A cursor used for elements that are used to horizontally resize top-level windows.
    SizeHorCursor,

    /// A cursor used for elements that are used to diagonally resize top-level windows at their top-right and bottom-left corners.
    SizeBDiagCursor,

    /// A cursor used for elements that are used to diagonally resize top-level windows at their top-left and bottom-right corners.
    SizeFDiagCursor,

    /// A cursor used for elements that are used to resize top-level windows in any direction.
    SizeAllCursor,

    /// A blank/invisible cursor, typically used when the cursor shape needs to be hidden.
    BlankCursor,

    /// A cursor used for vertical splitters, indicating that a handle can be dragged horizontally to adjust the use of available space.
    SplitVCursor,

    /// A cursor used for horizontal splitters, indicating that a handle can be dragged vertically to adjust the use of available space.
    SplitHCursor,

    /// A pointing hand cursor that is typically used for clickable elements such as hyperlinks.
    PointingHandCursor,

    /// A slashed circle cursor, typically used during drag and drop operations to indicate that dragged content cannot be dropped on particular widgets or inside certain regions.
    ForbiddenCursor,

    /// A cursor representing an open hand, typically used to indicate that the area under the cursor is the visible part of a canvas that the user can click and drag in order to scroll around.
    OpenHandCursor,

    /// A cursor representing a closed hand, typically used to indicate that a dragging operation is in progress that involves scrolling.
    ClosedHandCursor,

    /// An arrow with a question mark, typically used to indicate the presence of What's This? help for a widget.
    WhatsThisCursor,

    /// An hourglass or watch cursor, usually shown during operations that allow the user to interact with the application while they are performed in the background.
    BusyCursor,

    /// A cursor that is usually used when dragging an item.
    DragMoveCursor,

    /// A cursor that is usually used when dragging an item to copy it.
    DragCopyCursor,

    /// A cursor that is usually used when dragging an item to make a link to it.
    DragLinkCursor,
}
