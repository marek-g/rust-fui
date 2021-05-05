#ifndef RUST_FUI_QMENU_H
#define RUST_FUI_QMENU_H

#ifdef __cplusplus
extern "C" {
#endif

void *QMenu_new();
void QMenu_delete(void *self);

// QMenu doesn't take ownership of action
//void QMenu_addAction(void *self, void *action);

// QMenu takes ownership of result
void* QMenu_addAction_text(void *self, void *text);
void* QMenu_addSeparator(void *self);
void* QMenu_addMenu(void *self, void *text);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QMENU_H
