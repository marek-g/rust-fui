#include <QApplication>
#include <QCoreApplication>
#include <QSurfaceFormat>
#include <QThread>
#include <Qt>
#include "qapplication.h"
#include <stdlib.h>

#ifdef Q_OS_UNIX
#include <locale.h>
#endif

int argc_copy;
char **argv_copy;

void *QApplication_new(int argc, const char** const argv)
{
    // copy argc * argv as QApplication requires them
    // to be available all the time
    argc_copy = argc;
    argv_copy = new char *[argc_copy];
    for (int i = 0; i < argc_copy; i++)
    {
        argv_copy[i] = new char[strlen(argv[i]) + 1];
        strcpy(argv_copy[i], argv[i]);
    }

    void *app = static_cast<void *>(new (std::nothrow) QApplication(argc_copy, argv_copy));

#if defined(Q_OS_UNIX)
    // On Unix/Linux Qt is configured to use the system locale settings by
    // default. This can cause a conflict when using POSIX functions, for
    // instance, when converting between data types such as floats and
    // strings, since the notation may differ between locales. To get
    // around this problem, call the POSIX function \c{setlocale(LC_NUMERIC,"C")}
    // right after initializing QApplication, QGuiApplication or QCoreApplication
    // to reset the locale that is used for number formatting to "C"-locale.
    setlocale(LC_ALL, "C");
#endif

    return app;
}

void QApplication_delete(void *self)
{
    delete static_cast<QApplication *>(self);

    for (int i = 0; i < argc_copy; i++)
    {
        delete argv_copy[i];
    }
    delete argv_copy;
}

void QApplication_setApplicationDisplayName(const void *text)
{
    const QString *qtext = static_cast<const QString *>(text);
    QApplication::setApplicationDisplayName(*qtext);
}

void QApplication_setAttribute(int attr, int enable)
{
    QApplication::setAttribute((Qt::ApplicationAttribute)attr, enable != 0);
}

int QApplication_exec()
{
    return QApplication::exec();
}

void QApplication_exit(int returnCode)
{
    QApplication::exit(returnCode);
}

int QApplication_isGuiThread()
{
    return QCoreApplication::instance() != nullptr &&
        QThread::currentThread() == QCoreApplication::instance()->thread();
}

void QApplication_postFunc(void (*callback_trampoline)(void*), void *callback_data)
{
    QCoreApplication *app = QApplication::instance();
    if (app)
    {
        QMetaObject::invokeMethod(app,
                                  [callback_trampoline, callback_data] { callback_trampoline(callback_data); },
                                  Qt::QueuedConnection);
    }
}

void QApplication_aboutQt()
{
    QApplication::aboutQt();
}
