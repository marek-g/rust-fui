#ifndef RUST_FUI_QWINDOW_H
#define RUST_FUI_QWINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*QFunctionPointer)();

void *QWindow_new(void *parent);
void QWindow_delete(void *self);

void QWindow_setTitle(void *self, const void *text);
void QWindow_setIcon(void *self, const void *icon);
void QWindow_setStayOnTop(void *self, const int stayOnTop);
void QWindow_setTransparentForInput(void *self, const int transparentForInput);
void QWindow_setFrameType(void *self, const int frameType);
void QWindow_setPopupWindow(void *self);
void QWindow_setTranslucentBackground(void *self, const int translucentEffect);
void QWindow_setVisible(void *self, int visible);

int QWindow_getPositionX(void *self);
int QWindow_getPositionY(void *self);
void QWindow_setPosition(void *self, int x, int y);
int QWindow_getFramePositionX(void *self);
int QWindow_getFramePositionY(void *self);
void QWindow_setFramePosition(void *self, int x, int y);
int QWindow_getWidth(void *self);
int QWindow_getHeight(void *self);
void QWindow_resize(void *self, int width, int height);
void QWindow_setMinimumSize(void *self, int width, int height);
void QWindow_setCursorShape(void *self, int cursorShape);
int QWindow_startSystemMove(void *self);
int QWindow_startSystemResize(void *self, int edges);

void QWindow_update(void *self);

void QWindow_setEventFunc(void *self, void* (*func)(void*, void*), void *data);
void QWindow_setInitializeGLFunc(void *self, void (*func)(void*), void *data);
void QWindow_setPaintGLFunc(void *self, void (*func)(void*), void *data);

void *QWindow_context(void *self);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QWINDOW_H
