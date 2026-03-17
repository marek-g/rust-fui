use std::any::TypeId;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::common::type_map::TypeMapKey;

/// Global registry tracking which TypeMapKey types are inherited.
///
/// This is used by the ui! macro to determine whether an attached value
/// should be stored in the local `TypeMap` or the inherited `InheritedTypeMap`.
///
/// # Usage
///
/// When defining a new attached property that should be inherited,
/// call `register_inherited!()` in the same module:
///
/// ```rust
/// pub struct ColorScheme { /* ... */ }
///
/// impl TypeMapKey for ColorScheme {
///     type Value = ColorScheme;
/// }
///
/// register_inherited!(ColorScheme);
/// ```
static INHERITED_TYPE_IDS: Lazy<Mutex<HashSet<TypeId>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

/// Register a TypeMapKey type as inherited.
///
/// Call this once when defining your inherited property type.
/// This is typically done in the same module where you define the type.
///
/// # Example
///
/// ```rust
/// pub struct FontSize;
///
/// impl TypeMapKey for FontSize {
///     type Value = f32;
/// }
///
/// register_inherited!(FontSize);
/// ```
#[macro_export]
macro_rules! register_inherited {
    ($type:ty) => {
        $crate::attached_value_registry::register_inherited::<$type>();
    };
}

/// Register a TypeMapKey type as inherited.
///
/// This is the function called by the `register_inherited!()` macro.
/// In most cases, you should use the macro instead of calling this directly.
pub fn register_inherited<K: TypeMapKey + 'static>() {
    INHERITED_TYPE_IDS.lock().unwrap().insert(TypeId::of::<K>());
}

/// Check if a TypeMapKey type is registered as inherited.
///
/// This is used internally by the ui! macro to route values
/// to the correct storage (TypeMap vs InheritedTypeMap).
pub fn is_inherited_type<K: TypeMapKey + 'static>() -> bool {
    INHERITED_TYPE_IDS.lock().unwrap().contains(&TypeId::of::<K>())
}

/// Helper function used by the ui! macro to insert a value
/// into the correct map based on whether the type is registered as inherited.
///
/// This allows the macro to generate uniform code for all attached values
/// without knowing at compile time which are inherited.
pub fn insert_attached_value<K: TypeMapKey + 'static>(
    value: K::Value,
    attached: &mut crate::common::type_map::TypeMap,
    inherited: &mut crate::common::inherited_type_map::InheritedTypeMap,
) {
    if is_inherited_type::<K>() {
        inherited.insert::<K>(value);
    } else {
        attached.insert::<K>(value);
    }
}

/// Alternative version that always inserts into attached map.
/// Used for non-inherited properties.
pub fn insert_attached_value_local<K: TypeMapKey + 'static>(
    value: K::Value,
    attached: &mut crate::common::type_map::TypeMap,
) {
    attached.insert::<K>(value); // ignore return value
}

/// Alternative version that always inserts into inherited map.
/// Used for inherited properties.
pub fn insert_attached_value_inherited<K: TypeMapKey + 'static>(
    value: K::Value,
    inherited: &mut crate::common::inherited_type_map::InheritedTypeMap,
) {
    inherited.insert::<K>(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::type_map::TypeMapKey;

    struct TestInheritedKey;
    impl TypeMapKey for TestInheritedKey {
        type Value = String;
    }

    struct TestLocalKey;
    impl TypeMapKey for TestLocalKey {
        type Value = i32;
    }

    #[test]
    fn test_register_and_check() {
        register_inherited::<TestInheritedKey>();

        assert!(is_inherited_type::<TestInheritedKey>());
        assert!(!is_inherited_type::<TestLocalKey>());
    }

    #[test]
    fn test_macro_registration() {
        // This tests that the macro works correctly
        register_inherited!(TestInheritedKey);
        assert!(is_inherited_type::<TestInheritedKey>());
    }
}
