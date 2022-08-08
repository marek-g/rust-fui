#include "qfiledialog.h"
#include <QWidget>
#include <QString>
#include <QFileDialog>

void *QFileDialog_getOpenFileName(void *parent,
                              void *caption, void *dir,
                              void *filter, void *selected_filter,
                              int options)
{
    QWidget *qparent = static_cast<QWidget *>(parent);
    const QString *qcaption = static_cast<const QString *>(caption);
    const QString *qdir = static_cast<const QString *>(dir);
    const QString *qfilter = static_cast<const QString *>(filter);
    QString *qselectedfilter = static_cast<QString *>(selected_filter);
    const QFileDialog::Options qoptions = static_cast<const QFileDialog::Options>(options);
    QString result = QFileDialog::getOpenFileName(qparent,
                                        *qcaption, *qdir,
                                        *qfilter, qselectedfilter,
                                        qoptions);
    if (result.isNull()) {
        return nullptr;
    }
    return static_cast<void *>(new QString(std::move(result)));
}

void *QFileDialog_getOpenFileNames(void *parent,
                                   void *caption, void *dir,
                                   void *filter, void *selected_filter,
                                   int options)
{
    QWidget *qparent = static_cast<QWidget *>(parent);
    const QString *qcaption = static_cast<const QString *>(caption);
    const QString *qdir = static_cast<const QString *>(dir);
    const QString *qfilter = static_cast<const QString *>(filter);
    QString *qselectedfilter = static_cast<QString *>(selected_filter);
    const QFileDialog::Options qoptions = static_cast<const QFileDialog::Options>(options);
    QStringList result = QFileDialog::getOpenFileNames(qparent,
                                                   *qcaption, *qdir,
                                                   *qfilter, qselectedfilter,
                                                   qoptions);
    return static_cast<void *>(new QStringList(std::move(result)));
}

void *QFileDialog_getExistingDirectory(void *parent,
                                       void *caption, void *dir,
                                       int options)
{
    QWidget *qparent = static_cast<QWidget *>(parent);
    const QString *qcaption = static_cast<const QString *>(caption);
    const QString *qdir = static_cast<const QString *>(dir);
    const QFileDialog::Options qoptions = static_cast<const QFileDialog::Options>(options);
    QString result = QFileDialog::getExistingDirectory(qparent,
                                                       *qcaption, *qdir,
                                                       qoptions);
    if (result.isNull()) {
        return nullptr;
    }
    return static_cast<void *>(new QString(std::move(result)));
}

void *QFileDialog_getSaveFileName(void *parent,
                                  void *caption, void *dir,
                                  void *filter, void *selected_filter,
                                  int options)
{
    QWidget *qparent = static_cast<QWidget *>(parent);
    const QString *qcaption = static_cast<const QString *>(caption);
    const QString *qdir = static_cast<const QString *>(dir);
    const QString *qfilter = static_cast<const QString *>(filter);
    QString *qselectedfilter = static_cast<QString *>(selected_filter);
    const QFileDialog::Options qoptions = static_cast<const QFileDialog::Options>(options);
    QString result = QFileDialog::getSaveFileName(qparent,
                                                  *qcaption, *qdir,
                                                  *qfilter, qselectedfilter,
                                                  qoptions);
    if (result.isNull()) {
        return nullptr;
    }
    return static_cast<void *>(new QString(std::move(result)));
}
