#ifndef RUST_FUI_QPIXMAP_H
#define RUST_FUI_QPIXMAP_H

#ifdef __cplusplus
extern "C" {
#endif

void *QPixmap_new();
void QPixmap_delete(void *self);

int QPixmap_loadFromData(void *self, const unsigned char *data, int len);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QPIXMAP_H
