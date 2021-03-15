#ifndef RUST_FUI_QACTION_H
#define RUST_FUI_QACTION_H

#ifdef __cplusplus
extern "C" {
#endif

void *QAction_new();
void QAction_delete(void *self);

void QAction_setText(void *self, const void *text);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QACTION_H
