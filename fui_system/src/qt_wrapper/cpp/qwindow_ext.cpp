#include "qwindow_ext.h"

QWindowExt::QWindowExt(QWindow *parent) : QOpenGLWindow(QOpenGLWindow::NoPartialUpdate, parent)
{
}

void QWindowExt::paintGL()
{
}
