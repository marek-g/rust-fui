#ifndef RUST_FUI_QSLOT_H
#define RUST_FUI_QSLOT_H

#ifdef __cplusplus
extern "C" {
#endif

void *QSlot_new();
void QSlot_delete(void *self);

void QSlot_setFunc(void *self, void (*func)(void*), void *data);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QSLOT_H
