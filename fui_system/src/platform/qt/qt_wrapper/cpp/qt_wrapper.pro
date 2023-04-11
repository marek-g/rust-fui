DESTDIR = ./

TEMPLATE = lib
CONFIG += staticlib

SOURCES += qaction.cpp qapplication.cpp \
    qbytearray.cpp \
    qfiledialog.cpp \
    qicon.cpp qmenu.cpp \
    qopenglcontext.cpp \
    qpixmap.cpp qslot.cpp qslot_cpp.cpp \
    qstring.cpp qstringlist.cpp qsurfaceformat.cpp \
    qsystemtrayicon.cpp \
    qwindow.cpp qwindow_ext.cpp
HEADERS += qaction.h qapplication.h \
    qbytearray.h \
    qfiledialog.h \
    qicon.h qmenu.h \
    qopenglcontext.h \
    qpixmap.h qslot.h qslot_cpp.h \
    qstring.h qstringlist.h qsurfaceformat.h \
    qsystemtrayicon.h \
    qwindow.h qwindow_ext.h

QT += widgets

unix {
    QT += KWindowSystem
}
