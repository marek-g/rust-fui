#ifndef RUST_FUI_QWINDOW_EXT_H
#define RUST_FUI_QWINDOW_EXT_H

#include <QOpenGLWindow>

class QWindowExt : public QOpenGLWindow {
    Q_OBJECT

public:
    QWindowExt(QWindow *parent = nullptr);

    void setInitializeGLFunc(void (*func)(void*), void *data);
    void setPaintGLFunc(void (*func)(void*), void *data);

protected:
    void initializeGL();
    void paintGL() Q_DECL_OVERRIDE;

private:
    void (*m_funcInitializeGL)(void*);
    void *m_dataInitializeGL;

    void (*m_funcPaintGL)(void*);
    void *m_dataPaintGL;
};


#endif //RUST_FUI_QWINDOW_EXT_H
