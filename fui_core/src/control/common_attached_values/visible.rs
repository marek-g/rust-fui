//
// Attached values
//

use crate::Property;

pub struct Visible;
impl typemap::Key for Visible {
    type Value = Property<bool>;
}
