#ifndef RUST_FUI_QWINDOW_EXT_H
#define RUST_FUI_QWINDOW_EXT_H

#include <QOpenGLWindow>

class QWindowExt : public QOpenGLWindow {
    Q_OBJECT

public:
    QWindowExt(QWindow *parent = nullptr);

protected:
    void paintGL() Q_DECL_OVERRIDE;
};


#endif //RUST_FUI_QWINDOW_EXT_H
