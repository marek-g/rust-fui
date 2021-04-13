#include <QWindow>
#include "qwindow.h"

void *QWindow_new(void *parent)
{
    QWindow *parent_window = static_cast<QWindow *>(parent);
    return static_cast<void *>(new QWindow(parent_window));
}

void QWindow_delete(void *self)
{
    delete static_cast<QWindow *>(self);
}

void QWindow_setTitle(void *self, const void *text)
{
    QWindow *window = static_cast<QWindow *>(self);
    const QString *qtext = static_cast<const QString *>(text);
    window->setTitle(*qtext);
}

void QWindow_setVisible(void *self, int visible)
{
    QWindow *window = static_cast<QWindow *>(self);
    window->setVisible(visible != 0);
}
