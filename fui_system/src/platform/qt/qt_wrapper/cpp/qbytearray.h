#ifndef RUST_FUI_QBYTEARRAY_H
#define RUST_FUI_QBYTEARRAY_H

#ifdef __cplusplus
extern "C" {
#endif

void *QByteArray_new();
void QByteArray_delete(void *self);

const char *QByteArray_constData(void *self);
int QByteArray_size(void *self);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QBYTEARRAY_H
