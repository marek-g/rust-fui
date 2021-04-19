#include "qwindow_ext.h"

QWindowExt::QWindowExt(QWindow *parent)
    : QOpenGLWindow(QOpenGLWindow::NoPartialUpdate, parent),
    m_funcInitializeGL(0),
    m_funcPaintGL(0)
{
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
