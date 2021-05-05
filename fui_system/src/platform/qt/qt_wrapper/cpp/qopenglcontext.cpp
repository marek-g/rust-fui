#include <QOpenGLContext>
#include "qopenglcontext.h"

void *QOpenGLContext_new()
{
    return static_cast<void *>(new (std::nothrow) QOpenGLContext());
}

void QOpenGLContext_delete(void *self)
{
    delete static_cast<QOpenGLContext *>(self);
}

QFunctionPointer QOpenGLContext_getProcAddress(void *self, const char *procName)
{
    QOpenGLContext *qOpenGlContext = static_cast<QOpenGLContext *>(self);
    return qOpenGlContext->getProcAddress(procName);
}
