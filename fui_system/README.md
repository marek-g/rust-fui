# fui_system

Cross-platform windowing library. Compared to winit / sdl / sfml it is better suited for desktop applications as it gives you abstractions not only for windows but also for other desktop related features like dialogs, menus, notification tray icons etc.


Supported platforms:

- Linux (X11/Wayland using Qt)
- Windows (using Qt, tested on msys2 / mingw)
- it should be relatively easy to add other platforms supported by Qt

Features:

- multiple windows
- OpenGL support
- system tray icons
