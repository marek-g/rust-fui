#include <QApplication>
#include <QCoreApplication>
#include <QSurfaceFormat>
#include <Qt>
#include "qapplication.h"
#include <stdlib.h>

int argc_copy;
char **argv_copy;

void *QApplication_new(int argc, const char** const argv) {
    // copy argc * argv as QApplication requires them
    // to be available all the time
    argc_copy = argc;
    argv_copy = new char *[argc_copy];
    for (int i = 0; i < argc_copy; i++)
    {
        argv_copy[i] = new char[strlen(argv[i]) + 1];
        strcpy(argv_copy[i], argv[i]);
    }

    return static_cast<void *>(new (std::nothrow) QApplication(argc_copy, argv_copy));
}

void QApplication_delete(void *self) {
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

int QApplication_exec() {
    return QApplication::exec();
}

void QApplication_exit(int returnCode) {
    QApplication::exit(returnCode);
}

void QApplication_aboutQt() {
    QApplication::aboutQt();
}
