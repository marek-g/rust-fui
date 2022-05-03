# Async support

The FUI library supports async code using `Tokio` runtime.

## Threads

Both `async` runtime and `fui_system` have their own message loops. The `FUI` library creates a separate thread for `fui_system` message loop. The two threads are:

- `VM Thread` - (`VM` stands for `View Models`) - the main, `async` thread where the ViewModels are running
- `GUI Thread` - the thread where the window handling and drawing is done

Communication between the threads is done by posting closures using communication channels.

The GUI events are forwarded to the `VM Thread`. All the control's methods (including the the layout phase and `to_primitives()`) is done in the `VM Thread`.

The `DrawingContext` is designed that calling resource related methods is safe from the `VM Thread`.

> In the current implementation the font can be created and measured during the layout phase without communication with the `GUI Thread`. The font data will be loaded, but the texture creation will be deferred to to the drawing phase done on the `GUI Thread`. 
