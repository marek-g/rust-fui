#include <QSystemTrayIcon>
#include "qsystemtrayicon.h"

void *QSystemTrayIcon_new()
{
    return static_cast<void *>(new (std::nothrow) QSystemTrayIcon());
}

void QSystemTrayIcon_delete(void *self)
{
    delete static_cast<QSystemTrayIcon *>(self);
}

void QSystemTrayIcon_setContextMenu(void *self, void *menu)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    QMenu *qmenu = static_cast<QMenu *>(menu);
    tray->setContextMenu(qmenu);
}

void QSystemTrayIcon_setIcon(void *self, const void *icon)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    const QIcon *qicon = static_cast<const QIcon *>(icon);
    tray->setIcon(*qicon);
}

void QSystemTrayIcon_setToolTip(void *self, const void *tip)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    const QString *qtip = static_cast<const QString *>(tip);
    tray->setToolTip(*qtip);
}

void QSystemTrayIcon_setVisible(void *self, int visible)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    tray->setVisible(visible != 0);
}

void QSystemTrayIcon_showMessage(void *self, const void *title, const void *message, int icon, int timeoutMilliseconds)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    const QString *qtitle = static_cast<const QString *>(title);
    const QString *qmessage = static_cast<const QString *>(message);
    tray->showMessage(*qtitle, *qmessage, (QSystemTrayIcon::MessageIcon)icon, timeoutMilliseconds);
}

void QSystemTrayIcon_showMessage2(void *self, const void *title, const void *message, const void *icon, int timeoutMilliseconds)
{
    QSystemTrayIcon *tray = static_cast<QSystemTrayIcon *>(self);
    const QString *qtitle = static_cast<const QString *>(title);
    const QString *qmessage = static_cast<const QString *>(message);
    const QIcon *qicon = static_cast<const QIcon *>(icon);
    tray->showMessage(*qtitle, *qmessage, *qicon, timeoutMilliseconds);
}
