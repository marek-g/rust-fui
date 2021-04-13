#include <QString>
#include "qstring.h"

void *QString_fromUtf8(const char *str, int size)
{
    return static_cast<void *>(new QString(QString::fromUtf8(str, size)));
}

void QString_delete(void *self)
{
    delete static_cast<QString *>(self);
}
