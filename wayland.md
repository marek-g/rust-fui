# Transparency glitches on wayland

As of 2025-04-08 Qt6 (6.8.2) on Wayland with NVidia driver (570.133.07)
opens OpenGL apps as OpenGL ES and when fui app is enabling stencil buffer
(QSurfaceFormat.setStencilBufferSize(8)) there are transparency glitches
that would be not present on wayland/OpenGL (tested with GLFW).
Forcing OpenGL with QSurfaceFormat.setRenderableType(QSurfaceFormat::OpenGL)
opens empty window.

The workaround is to run the app in XWayland. Qt apps can be run as:

``` sh
QT_QPA_PLATFORM=xbc app
app --platform=xbc
```

However, as this is inconvenient, the `fui_system` for now is modified
to run with `xbc` platform on all unix systems if no `--platform` parameter was provided.

So, for now fui apps run in XWayland, unless you run them as:

``` sh
app --platform=wayland
```
