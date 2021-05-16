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
void QWindow_setVisible(void *self, int visible);

int QWindow_getWidth(void *self);
int QWindow_getHeight(void *self);
void QWindow_resize(void *self, int width, int height);

void QWindow_update(void *self);

void QWindow_setEventFunc(void *self, void* (*func)(void*, void*), void *data);
void QWindow_setInitializeGLFunc(void *self, void (*func)(void*), void *data);
void QWindow_setPaintGLFunc(void *self, void (*func)(void*), void *data);

void *QWindow_context(void *self);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QWINDOW_H
