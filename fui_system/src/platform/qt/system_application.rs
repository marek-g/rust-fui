use crate::platform::qt::qt_wrapper::{QApplication, QString};

///
/// The application.
/// Manages application control flow.
///
pub struct SystemApplication {
    qapp: QApplication,
}

impl SystemApplication {
    ///
    /// Creates the application object and
    /// sets the application display name.
    ///
    pub fn new(app_name: &str) -> Result<Self, ()> {
        let qapp = QApplication::new()?;

        let app_name = QString::from_str(app_name)?;
        QApplication::set_application_display_name(&app_name);

        Ok(Self { qapp })
    }

    ///
    /// Enters the main event loop and waits until
    /// exit() is called, then returns the value that was set to exit().
    ///
    pub fn message_loop() -> i32 {
        QApplication::exec()
    }

    ///
    /// Tells the message loop to exit with a return code.
    ///
    pub fn exit(return_code: i32) {
        QApplication::exit(return_code);
    }
}
