#ifndef RUST_FUI_QSTRINGLIST_H
#define RUST_FUI_QSTRINGLIST_H

#ifdef __cplusplus
extern "C" {
#endif

void QStringList_delete(void *self);

const void *QStringList_at(void *self, int pos);
int QStringList_size(void *self);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QSTRINGLIST_H
