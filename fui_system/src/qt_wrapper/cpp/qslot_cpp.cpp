#include "qslot_cpp.h"
#include <stdio.h>

QSlotCpp::QSlotCpp() {
}

QSlotCpp::~QSlotCpp() {
}

void QSlotCpp::setFunc(void (*func)(void *), void *data) {
    m_func = func;
    m_data = data;
}

void QSlotCpp::method() {
    m_func(m_data);
}
