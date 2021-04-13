TEMPLATE = lib
CONFIG += staticlib

SOURCES += qaction.cpp qapplication.cpp qicon.cpp qmenu.cpp \
    qpixmap.cpp qslot.cpp qslot_cpp.cpp qstring.cpp qsystemtrayicon.cpp \
    qwindow.cpp
HEADERS += qaction.h qapplication.h qicon.h qmenu.h \
    qpixmap.h qslot.h qslot_cpp.h qstring.h qsystemtrayicon.h \
    qwindow.h

QT += widgets
