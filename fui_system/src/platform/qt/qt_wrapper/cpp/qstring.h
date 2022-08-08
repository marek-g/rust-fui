#ifndef RUST_FUI_QSTRING_H
#define RUST_FUI_QSTRING_H

#ifdef __cplusplus
extern "C" {
#endif

void *QString_null();
void *QString_fromUtf8(const char *str, int size);
void QString_delete(void *self);

void *QString_toUtf8(void *self);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QSTRING_H
