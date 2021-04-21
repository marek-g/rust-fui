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

void QWindow_resize(void *self, int width, int height)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->resize(width, height);
}

void QWindow_update(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->update();
}

void QWindow_setInitializeGLFunc(void *self, void (*func)(void*), void *data)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setInitializeGLFunc(func, data);
}

void QWindow_setPaintGLFunc(void *self, void (*func)(void*), void *data)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setPaintGLFunc(func, data);
}

void *QWindow_context(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->context();
}
