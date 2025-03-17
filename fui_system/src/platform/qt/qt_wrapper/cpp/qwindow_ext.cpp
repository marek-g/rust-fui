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
        FFIEvent ffiEvent;
        if (convertEventToRust(event, ffiEvent)) {
            void *result = m_funcEvent(m_dataEvent, (void*)&ffiEvent);

            freeEvent(ffiEvent);

            if (result != 0) {
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

bool QWindowExt::convertEventToRust(QEvent *event, FFIEvent &ffiEvent)
{
    switch (event->type())
    {
        case QEvent::Enter: {
            ffiEvent.tag = FFIEvent::Tag::MouseEnter;
            return true;
        }

        case QEvent::Leave: {
            ffiEvent.tag = FFIEvent::Tag::MouseLeave;
            return true;
        }

        case QEvent::MouseMove: {
            ffiEvent.tag = FFIEvent::Tag::MouseMove;
            ffiEvent.mouse_move.position.x = ((QMouseEvent*)event)->position().x();
            ffiEvent.mouse_move.position.y = ((QMouseEvent*)event)->position().y();
            return true;
        }

        case QEvent::MouseButtonPress:
        case QEvent::MouseButtonRelease: {
            ffiEvent.tag = FFIEvent::Tag::MouseButton;
            ffiEvent.mouse_button.state = event->type() == QEvent::MouseButtonPress ?
                                          FFIElementState::Pressed : FFIElementState::Released;
            ffiEvent.mouse_button.button = convertMouseButton(((QMouseEvent*)event)->button());
            return true;
        }

        case QEvent::Wheel: {
            ffiEvent.tag = FFIEvent::Tag::ScrollWheel;

            QPoint degrees = ((QWheelEvent*)event)->angleDelta();
            if (!degrees.isNull()) {
                ffiEvent.scroll_wheel.delta.tag = FFIScrollDelta::Tag::LineDelta;
                ffiEvent.scroll_wheel.delta.line_delta._0 = degrees.x() / 120.0;
                ffiEvent.scroll_wheel.delta.line_delta._1 = degrees.y() / 120.0;
                return true;
            }

            QPoint pixels = ((QWheelEvent*)event)->pixelDelta();
            if (!pixels.isNull()) {
                ffiEvent.scroll_wheel.delta.tag = FFIScrollDelta::Tag::PixelDelta;
                ffiEvent.scroll_wheel.delta.pixel_delta._0 = pixels.x();
                ffiEvent.scroll_wheel.delta.pixel_delta._1 = pixels.y();
                return true;
            }

            return false;
        }

        case QEvent::KeyPress:
        case QEvent::KeyRelease: {
            ffiEvent.tag = FFIEvent::Tag::KeyEvent;
            ffiEvent.key_event.state = event->type() == QEvent::KeyPress ?
                    FFIElementState::Pressed : FFIElementState::Released;
            ffiEvent.key_event.keycode = ((QKeyEvent*)event)->key();
            ffiEvent.key_event.is_repeat = ((QKeyEvent*)event)->isAutoRepeat();
            Qt::KeyboardModifiers modifiers = ((QKeyEvent*)event)->modifiers();
            ffiEvent.key_event.modifiers.shift = (modifiers & Qt::ShiftModifier);
            ffiEvent.key_event.modifiers.ctrl = (modifiers & Qt::ControlModifier);
            ffiEvent.key_event.modifiers.alt = (modifiers & Qt::AltModifier);
            ffiEvent.key_event.modifiers.win = (modifiers & Qt::MetaModifier);
            ffiEvent.key_event.modifiers.keypad = (modifiers & Qt::KeypadModifier);
            if (((QKeyEvent*)event)->text().isEmpty()) {
                ffiEvent.key_event.text = 0;
            } else {
                QByteArray textUtf8 = ((QKeyEvent *) event)->text().toUtf8();
                char *text = new char[textUtf8.size() + 1];
                strcpy(text, textUtf8.data());
                ffiEvent.key_event.text = text;
            }
            return true;
        }

        case QEvent::Resize: {
            ffiEvent.tag = FFIEvent::Tag::Resize;
            ffiEvent.resize.width = ((QResizeEvent*)event)->size().width();
            ffiEvent.resize.height = ((QResizeEvent*)event)->size().height();
            return true;
        }

        default: return false;
    }
}

void QWindowExt::freeEvent(FFIEvent &ffiEvent)
{
    if (ffiEvent.tag == FFIEvent::Tag::KeyEvent) {
        delete[] ffiEvent.key_event.text;
        ffiEvent.key_event.text = 0;
    }
}

FFIMouseButton QWindowExt::convertMouseButton(Qt::MouseButton button)
{
    FFIMouseButton result;

    switch (button) {
        case Qt::LeftButton: {
            result.tag = FFIMouseButton::Tag::Left;
            return result;
        }

        case Qt::RightButton: {
            result.tag = FFIMouseButton::Tag::Right;
            return result;
        }

        case Qt::MiddleButton: {
            result.tag = FFIMouseButton::Tag::Middle;
            return result;
        }

        default: {
            result.tag = FFIMouseButton::Tag::Other;

            uint8_t pos = 4;
            int mask = 8;
            while (!(((int)button) & mask) && pos < 31) {
                mask <<= 1;
                pos++;
            }

            result.other._0 = pos;
            return result;
        }
    }
}
