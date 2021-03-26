#include <QAction>
#include <QObject>
#include "qaction.h"
#include "qslot_cpp.h"
#include <stdio.h>

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

void QAction_connectTriggered(void *self, void *slot)
{
    QAction *qAction = static_cast<QAction *>(self);
    QSlotCpp *qSlot = static_cast<QSlotCpp *>(slot);

    QObject::connect(qAction, &QAction::triggered, qSlot, &QSlotCpp::method);
}
