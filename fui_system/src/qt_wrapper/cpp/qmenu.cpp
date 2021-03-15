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
