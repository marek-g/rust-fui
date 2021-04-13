#ifndef RUST_FUI_QSLOT_CPP_H
#define RUST_FUI_QSLOT_CPP_H

#include <QObject>

class QSlotCpp : public QObject {
    Q_OBJECT

public:
    QSlotCpp();
    virtual ~QSlotCpp();

    void setFunc(void (*func)(void*), void *data);

public slots:
    void method();

private:
    void (*m_func)(void*);
    void *m_data;
};

#endif //RUST_FUI_QSLOT_CPP_H
