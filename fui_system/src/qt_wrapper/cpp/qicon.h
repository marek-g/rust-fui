#ifndef RUST_FUI_QICON_H
#define RUST_FUI_QICON_H

#ifdef __cplusplus
extern "C" {
#endif

void *QIcon_new();
void QIcon_delete(void *self);

void QIcon_addPixmap(void *self, const void *pixmap, int mode, int state);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QICON_H
