#include <QWindow>
#include <QOpenGLContext>
#include "qwindow_ext.h"
#include "qwindow.h"

void *QWindow_new(void *parent)
{
    QWindow *parent_window = static_cast<QWindow *>(parent);
    return static_cast<void *>(new QWindowExt(parent_window));
}

void QWindow_delete(void *self)
{
    delete static_cast<QWindowExt *>(self);
}

void QWindow_setTitle(void *self, const void *text)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    const QString *qtext = static_cast<const QString *>(text);
    window->setTitle(*qtext);
}

void QWindow_setVisible(void *self, int visible)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setVisible(visible != 0);
}

QFunctionPointer OpenGLCurrentContext_getProcAddress(const char *procName)
{
    QOpenGLContext *currentContext = QOpenGLContext::currentContext();
    if (currentContext)
    {
        return currentContext->getProcAddress(procName);
    }

    return nullptr;
}