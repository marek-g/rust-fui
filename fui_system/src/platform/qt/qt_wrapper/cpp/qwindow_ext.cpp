#include "qwindow_ext.h"
#include <QMouseEvent>

QWindowExt::QWindowExt(QWindow *parent)
    : QOpenGLWindow(QOpenGLWindow::NoPartialUpdate, parent),
    m_funcEvent(0),
    m_funcInitializeGL(0),
    m_funcPaintGL(0)
{
}

void QWindowExt::setEventFunc(void* (*func)(void*, void*), void *data)
{
    m_funcEvent = func;
    m_dataEvent = data;
}

void QWindowExt::setInitializeGLFunc(void (*func)(void*), void *data)
{
    m_funcInitializeGL = func;
    m_dataInitializeGL = data;
}

void QWindowExt::setPaintGLFunc(void (*func)(void*), void *data)
{
    m_funcPaintGL = func;
    m_dataPaintGL = data;
}

bool QWindowExt::event(QEvent *event)
{
    if (m_funcEvent)
    {
        Event rustEvent;
        if (convertEventToRust(event, rustEvent)) {
            if (m_funcEvent(m_dataEvent, (void*)&rustEvent) != 0) {
                return true;
            }
        }
    }

    return QOpenGLWindow::event(event);
}

void QWindowExt::initializeGL()
{
    if (m_funcInitializeGL)
    {
        m_funcInitializeGL(m_dataInitializeGL);
    }
}

void QWindowExt::paintGL()
{
    if (m_funcPaintGL)
    {
        m_funcPaintGL(m_dataPaintGL);
    }
}

bool QWindowExt::convertEventToRust(QEvent *event, Event &rustEvent)
{
    switch (event->type())
    {
        case QEvent::Enter: {
            rustEvent.tag = Event::Tag::MouseEnter;
            return true;
        }

        case QEvent::Leave: {
            rustEvent.tag = Event::Tag::MouseLeave;
            return true;
        }

        case QEvent::MouseMove: {
            rustEvent.tag = Event::Tag::MouseMove;
            rustEvent.mouse_move.position.x = ((QMouseEvent*)event)->x();
            rustEvent.mouse_move.position.y = ((QMouseEvent*)event)->y();
            return true;
        }

        default: return false;
    }
}
