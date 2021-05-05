#include "qslot.h"
#include "qslot_cpp.h"

void *QSlot_new()
{
    return static_cast<void *>(new (std::nothrow) QSlotCpp());
}

void QSlot_delete(void *self)
{
    delete static_cast<QSlotCpp *>(self);
}

void QSlot_setFunc(void *self, void (*func)(void*), void *data)
{
    QSlotCpp *qSlot = static_cast<QSlotCpp *>(self);
    qSlot->setFunc(func, data);
}
