#include <QPixmap>
#include "qpixmap.h"

void *QPixmap_new()
{
    return static_cast<void *>(new (std::nothrow) QPixmap());
}

void QPixmap_delete(void *self)
{
    delete static_cast<QPixmap *>(self);
}

int QPixmap_loadFromData(void *self, const unsigned char *data, int len)
{
    QPixmap *qPixmap = static_cast<QPixmap *>(self);
    return qPixmap->loadFromData((const uchar*)data, len);
}
