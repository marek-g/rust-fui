#include <QStringList>
#include "qstringlist.h"

void QStringList_delete(void *self)
{
    delete static_cast<QStringList *>(self);
}

const void *QStringList_at(void *self, int pos)
{
    QStringList *qStringList = static_cast<QStringList *>(self);
    return static_cast<const void *>(&qStringList->at(pos));
}

int QStringList_size(void *self)
{
    QStringList *qStringList = static_cast<QStringList *>(self);
    return qStringList->size();
}
