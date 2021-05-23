use crate::platform::qt::qt_wrapper::QApplicationAttribute;
use crate::platform::qt::qt_wrapper::{QApplication, QString, QSurfaceFormat};
use crate::platform::ApplicationOptions;
use crate::{FUISystemError, LoopProxy};

///
/// The application.
/// Manages application control flow.
///
pub struct Application {
    _qapp: QApplication,
}

impl Application {
    ///
    /// Creates the application object and
    /// sets the application display name.
    ///
    pub fn new(options: ApplicationOptions) -> Result<Self, FUISystemError> {
        let app_name = QString::from_str(&options.title)?;
        QApplication::set_application_display_name(&app_name);

        QApplication::set_attribute(
            QApplicationAttribute::ShareOpenGLContexts,
            options.opengl_share_contexts,
        );

        QSurfaceFormat::set_default(options.opengl_stencil_bits);

        let qapp = QApplication::new()?;
        Ok(Self { _qapp: qapp })
    }

    ///
    /// Gets LoopProxy that allows to communicate
    /// with main message loop from the same thread.
    ///
    pub fn get_loop_proxy(&self) -> LoopProxy {
        LoopProxy::new()
    }

    ///
    /// Enters the main event loop and waits until
    /// exit() is called, then returns the value that was set to exit().
    ///
    pub fn message_loop(&self) -> i32 {
        QApplication::exec()
    }

    ///
    /// Tells the message loop to exit with a return code.
    ///
    pub fn exit(return_code: i32) {
        QApplication::exit(return_code);
    }

    ///
    /// Posts function to be executed on the main event loop.
    /// Can be called from any thread.
    ///
    pub fn post_func<F>(func: F)
    where
        F: FnOnce() + 'static + Send,
    {
        QApplication::post_func(func);
    }
}
