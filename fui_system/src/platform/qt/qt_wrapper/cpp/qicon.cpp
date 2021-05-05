#include <QIcon>
#include "qicon.h"

void *QIcon_new()
{
    return static_cast<void *>(new (std::nothrow) QIcon());
}

void QIcon_delete(void *self)
{
    delete static_cast<QIcon *>(self);
}

void QIcon_addPixmap(void *self, const void *pixmap, int mode, int state)
{
    QIcon *qIcon = static_cast<QIcon *>(self);
    const QPixmap *qPixmap = static_cast<const QPixmap *>(pixmap);

    qIcon->addPixmap(*qPixmap, (QIcon::Mode)mode, (QIcon::State)state);
}
