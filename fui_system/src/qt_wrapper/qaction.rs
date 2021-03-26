use crate::qt_wrapper::{QSlot, QString};

pub struct QAction {
    pub this: *mut ::std::os::raw::c_void,
    pub is_owned: bool,
}

impl QAction {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let this = crate::qt_wrapper::QAction_new();
            if this.is_null() {
                return Err(());
            }

            Ok(Self {
                this,
                is_owned: true,
            })
        }
    }

    pub fn set_text(&mut self, text: &QString) -> Result<(), ()> {
        unsafe {
            crate::qt_wrapper::QAction_setText(self.this, text.this);
            Ok(())
        }
    }

    pub fn connect_triggered(&mut self) -> Result<QSlot, ()> {
        let mut slot = QSlot::new()?;
        unsafe {
            crate::qt_wrapper::QAction_connectTriggered(self.this, slot.this);
        }
        Ok(slot)
    }
}

impl Drop for QAction {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe {
                crate::qt_wrapper::QAction_delete(self.this);
            }
        }
    }
}
