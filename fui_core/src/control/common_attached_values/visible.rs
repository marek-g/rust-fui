//
// Attached values
//

use crate::{Property, TypeMapKey};

pub struct Visible;
impl TypeMapKey for Visible {
    type Value = Property<bool>;
}
