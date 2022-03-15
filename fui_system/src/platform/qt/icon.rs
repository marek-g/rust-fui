use crate::platform::qt::qt_wrapper::{QIcon, QPixmap};
use crate::FUISystemError;

pub struct Icon {
    pub(crate) qicon: QIcon,
}

impl Icon {
    pub fn from_data(data: &[u8]) -> Result<Self, FUISystemError> {
        let pixmap = QPixmap::from_data(data)?;

        let mut icon = QIcon::new()?;
        icon.add_pixmap(&pixmap);

        Ok(Self { qicon: icon })
    }
}
