#ifndef RUST_FUI_QWINDOW_EXT_H
#define RUST_FUI_QWINDOW_EXT_H

#include "rust_ffi.h"
#include <QOpenGLWindow>

class QWindowExt : public QOpenGLWindow {
    Q_OBJECT

public:
    QWindowExt(QWindow *parent = nullptr);

    void setEventFunc(void* (*func)(void*, void*), void *data);
    void setInitializeGLFunc(void (*func)(void*), void *data);
    void setPaintGLFunc(void (*func)(void*), void *data);

protected:
    bool event(QEvent *event) Q_DECL_OVERRIDE;
    void initializeGL() Q_DECL_OVERRIDE;
    void paintGL() Q_DECL_OVERRIDE;

private:
    void* (*m_funcEvent)(void*, void*);
    void *m_dataEvent;

    void (*m_funcInitializeGL)(void*);
    void *m_dataInitializeGL;

    void (*m_funcPaintGL)(void*);
    void *m_dataPaintGL;

    static bool convertEventToRust(QEvent *event, FFIEvent &ffiEvent);
    static void freeEvent(FFIEvent &ffiEvent);
    static FFIMouseButton convertMouseButton(Qt::MouseButton button);
};


#endif //RUST_FUI_QWINDOW_EXT_H
