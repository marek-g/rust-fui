#include <QString>
#include "qstring.h"

void *QString_null()
{
    return static_cast<void *>(new (std::nothrow) QString());
}

void *QString_fromUtf8(const char *str, int size)
{
    return static_cast<void *>(new (std::nothrow) QString(QString::fromUtf8(str, size)));
}

void QString_delete(void *self)
{
    delete static_cast<QString *>(self);
}

void *QString_toUtf8(void *self)
{
    QString *qString = static_cast<QString *>(self);
    QByteArray array = qString->toUtf8();

    return static_cast<void *>(new QByteArray(std::move(array)));
}