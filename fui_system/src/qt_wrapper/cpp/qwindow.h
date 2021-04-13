#ifndef RUST_FUI_QWINDOW_H
#define RUST_FUI_QWINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

void *QWindow_new(void *parent);
void QWindow_delete(void *self);

void QWindow_setTitle(void *self, const void *text);
void QWindow_setVisible(void *self, int visible);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QWINDOW_H
