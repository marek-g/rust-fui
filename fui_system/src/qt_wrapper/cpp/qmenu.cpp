#include <QMenu>
#include "qmenu.h"

void *QMenu_new()
{
    return static_cast<void *>(new QMenu());
}

void QMenu_delete(void *self)
{
    delete static_cast<QMenu *>(self);
}

void QMenu_addAction(void *self, void *action)
{
    QMenu *qMenu = static_cast<QMenu *>(self);
    QAction *qAction = static_cast<QAction *>(action);

    qMenu->addAction(qAction);
}

void* QMenu_addAction_text(void *self, void *text)
{
    QMenu *qMenu = static_cast<QMenu *>(self);
    QString *qText = static_cast<QString *>(text);

    return qMenu->addAction(*qText);
}
