use crate::qt_wrapper::{QApplication, QString};

pub struct SystemApplication {
    qapp: QApplication,
}

impl SystemApplication {
    pub fn new(app_name: &str) -> Result<Self, ()> {
        let qapp = QApplication::new()?;

        let app_name = QString::from_str(app_name)?;
        QApplication::set_application_display_name(&app_name);

        Ok(Self { qapp })
    }

    pub fn message_loop() {
        QApplication::exec();
    }

    pub fn exit_message_loop() {
        QApplication::exit(0);
    }
}
