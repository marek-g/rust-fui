#ifndef RUST_FUI_QOPENGLCONTEXT_H
#define RUST_FUI_QOPENGLCONTEXT_H

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*QFunctionPointer)();

void *QOpenGLContext_new();
void QOpenGLContext_delete(void *self);

QFunctionPointer QOpenGLContext_getProcAddress(void *self, const char *procName);

#ifdef __cplusplus
}
#endif

#endif //RUST_FUI_QOPENGLCONTEXT_H
