#include <QAction>
#include "qaction.h"

void *QAction_new()
{
    return static_cast<void *>(new QAction());
}

void QAction_delete(void *self)
{
    delete static_cast<QAction *>(self);
}

void QAction_setText(void *self, const void *text)
{
    QAction *qAction = static_cast<QAction *>(self);
    const QString *qText = static_cast<const QString *>(text);

    qAction->setText(*qText);
}
