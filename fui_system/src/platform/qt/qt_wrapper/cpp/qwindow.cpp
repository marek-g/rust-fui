#include <QWindow>
#include <QOpenGLContext>
#include "qwindow_ext.h"
#include "qwindow.h"

#ifdef __unix__
#include <KWindowEffects>
#endif

#ifdef _WIN32
#include <windows.h>
#include "windows_transparency_win32.h"
#endif

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

void QWindow_setStayOnTop(void *self, const int stayOnTop)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setFlag(Qt::WindowStaysOnTopHint, stayOnTop != 0);
}

void QWindow_setTransparentForInput(void *self, const int transparentForInput)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setFlag(Qt::WindowTransparentForInput, transparentForInput != 0);
}

void QWindow_setFrameType(void *self, const int frameType)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setFlag(Qt::FramelessWindowHint, frameType == 0);
}

void QWindow_setPopupWindow(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setFlag(Qt::Popup, true);
}

void QWindow_setTranslucentBackground(void *self, const int translucentEffect)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    QSurfaceFormat format = QSurfaceFormat::defaultFormat();
    if (translucentEffect != 0) {
        format.setAlphaBufferSize(8);
    }
    window->setFormat(format);

    // blur effect
    #ifdef __unix__
    KWindowEffects::enableBlurBehind(window, translucentEffect == 2);
    #endif
    #ifdef _WIN32
    EnableBlurWin32((HWND)window->winId(), translucentEffect == 2);
    #endif
}

void QWindow_setVisible(void *self, int visible)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setVisible(visible != 0);
}

int QWindow_getPositionX(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->position().x();
}

int QWindow_getPositionY(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->position().y();
}

void QWindow_setPosition(void *self, int x, int y)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->setPosition(QPoint(x, y));
}

int QWindow_getFramePositionX(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->framePosition().x();
}

int QWindow_getFramePositionY(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->framePosition().y();
}

void QWindow_setFramePosition(void *self, int x, int y)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->setFramePosition(QPoint(x, y));
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

void QWindow_setCursorShape(void *self, int cursorShape)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    window->setCursor(QCursor((Qt::CursorShape)cursorShape));
}

int QWindow_startSystemMove(void *self)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->startSystemMove();
}

int QWindow_startSystemResize(void *self, int edges)
{
    QWindowExt *window = static_cast<QWindowExt *>(self);
    return window->startSystemResize((Qt::Edge)edges);
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
