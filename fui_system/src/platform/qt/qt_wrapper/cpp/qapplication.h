#ifndef RUST_FUI_QAPPLICATION_H
#define RUST_FUI_QAPPLICATION_H

#ifdef __cplusplus
extern "C" {
#endif

void *QApplication_new(int argc, const char** const argv);
void QApplication_delete(void *self);

void QApplication_setApplicationDisplayName(const void *text);
void QApplication_setAttribute(int attr, int enable);

int QApplication_exec();
void QApplication_exit(int returnCode);

int QApplication_isGuiThread();
void QApplication_postFunc(void (*callback_trampoline)(void*), void *callback_data);

void QApplication_aboutQt();

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QAPPLICATION_H
