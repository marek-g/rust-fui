#ifndef RUST_FUI_QSYSTEMTRAY_H
#define RUST_FUI_QSYSTEMTRAY_H

#ifdef __cplusplus
extern "C" {
#endif

void *QSystemTrayIcon_new();
void QSystemTrayIcon_delete(void *self);

// QSystemTrayIcon doesn't take ownership over menu
void QSystemTrayIcon_setContextMenu(void *self, void *menu);
void QSystemTrayIcon_setIcon(void *self, const void *icon);
void QSystemTrayIcon_setToolTip(void *self, const void *tip);

void QSystemTrayIcon_setVisible(void *self, int visible);

void QSystemTrayIcon_showMessage(void *self, const void *title, const void *message, int icon, int timeoutMilliseconds);
void QSystemTrayIcon_showMessage2(void *self, const void *title, const void *message, const void *icon, int timeoutMilliseconds);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QSYSTEMTRAY_H
