TEMPLATE = lib
CONFIG += staticlib

SOURCES += qapplication.cpp qicon.cpp qmenu.cpp \
    qpixmap.cpp qstring.cpp qsystemtrayicon.cpp
HEADER += qapplication.h qicon.h qmenu.h \
    qpixmap.h qstring.h qsystemtrayicon.h

QT += widgets
