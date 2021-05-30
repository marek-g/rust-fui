#include <QWindow>
#include <QOpenGLContext>
#include "qwindow_ext.h"
#include "qwindow.h"

void *QWindow_new(void *parent)
{
    QWindow *parent_window = static_cast<QWindow *>(parent);
    return static_cast<void *>(new (std::nothrow) QWindowExt(parent_window));
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

void QWindow_setIcon(void *self, const void *icon)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    const QIcon *qicon = static_cast<const QIcon *>(icon);
    window->setIcon(*qicon);
}

void QWindow_setVisible(void *self, int visible)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setVisible(visible != 0);
}

int QWindow_getWidth(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->size().width();
}

int QWindow_getHeight(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->size().height();
}

void QWindow_resize(void *self, int width, int height)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->resize(width, height);
}

void QWindow_setMinimumSize(void *self, int width, int height)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    QSize size(width, height);
    window->setMinimumSize(size);
}

void QWindow_update(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->update();
}

void QWindow_setEventFunc(void *self, void* (*func)(void*, void*), void *data)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setEventFunc(func, data);
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
