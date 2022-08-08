#include <QByteArray>
#include "qbytearray.h"

void *QByteArray_new()
{
    return static_cast<void *>(new (std::nothrow) QByteArray());
}

void QByteArray_delete(void *self)
{
    delete static_cast<QByteArray *>(self);
}

const char *QByteArray_constData(void *self)
{
    QByteArray *qByteArray = static_cast<QByteArray *>(self);
    return qByteArray->constData();
}

int QByteArray_size(void *self)
{
    QByteArray *qByteArray = static_cast<QByteArray *>(self);
    return qByteArray->size();
}
