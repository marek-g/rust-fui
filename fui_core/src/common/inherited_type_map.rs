use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::type_map::TypeMapKey;

/// InheritedTypeMap stores immutable values shared via Rc for copy-on-write semantics.
///
/// Values stored in this map are:
/// - **Immutable** - no mutable access is provided
/// - **Shared** - multiple controls can share the same value via Rc
/// - **Inherited** - children receive parent's values merged with their own overrides
///
/// The internal HashMap is also wrapped in Rc, so cloning the map is cheap
/// (just Rc::clone on the HashMap) until a modification is needed.
pub struct InheritedTypeMap {
    map: Rc<HashMap<TypeId, Rc<dyn Any>>>,
}

impl InheritedTypeMap {
    /// Creates a new, empty InheritedTypeMap.
    pub fn new() -> Self {
        InheritedTypeMap {
            map: Rc::new(HashMap::new()),
        }
    }

    /// Inserts a value associated with the key `K`.
    ///
    /// The value is wrapped in `Rc` for efficient sharing.
    /// If a value with this key already exists, it is replaced.
    ///
    /// This may clone the internal HashMap if it's shared with other instances.
    pub fn insert<K: TypeMapKey + 'static>(&mut self, value: K::Value) {
        let id = TypeId::of::<K>();
        
        // Get mutable access to the HashMap, cloning it if shared
        let map_mut = Rc::make_mut(&mut self.map);
        map_mut.insert(id, Rc::new(value));
    }

    /// Gets an immutable reference to the value associated with the key `K`.
    pub fn get<K: TypeMapKey + 'static>(&self) -> Option<&K::Value> {
        self.map
            .get(&TypeId::of::<K>())
            .and_then(|rc| rc.downcast_ref::<K::Value>())
    }

    /// Gets an `Rc` reference to the value for efficient sharing with children.
    ///
    /// This is the preferred method when propagating values to child controls.
    pub fn get_rc<K: TypeMapKey + 'static>(&self) -> Option<Rc<K::Value>> {
        self.map
            .get(&TypeId::of::<K>())
            .and_then(|rc| rc.clone().downcast::<K::Value>().ok())
    }

    /// Merges two InheritedTypeMaps, with `overrides` taking precedence.
    ///
    /// This is used when creating child controls:
    /// - If `overrides` is empty, this is a cheap Rc::clone
    /// - If `overrides` has values, the HashMap is cloned only if it's shared
    /// - Child's values override parent's values for the same keys
    ///
    /// # Example
    /// ```rust
    /// let parent_values = /* ... */;
    /// let child_local_values = /* ... */;
    /// let child_inherited = parent_values.merge(&child_local_values);
    /// ```
    pub fn merge(&self, overrides: &InheritedTypeMap) -> InheritedTypeMap {
        // Fast path: if overrides is empty, just clone our Rc
        if overrides.map.is_empty() {
            return InheritedTypeMap {
                map: self.map.clone(),
            };
        }

        // Fast path: if self is empty, return clone of overrides
        if self.map.is_empty() {
            return InheritedTypeMap {
                map: overrides.map.clone(),
            };
        }

        // Slow path: need to merge
        // Start with a clone of self's map (clones HashMap only if Rc::strong_count > 1)
        let mut result_map = (*self.map).clone();

        // Override with child's values
        for (id, value) in overrides.map.iter() {
            result_map.insert(*id, value.clone());
        }

        InheritedTypeMap {
            map: Rc::new(result_map),
        }
    }

    /// Returns true if the map contains no values.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of values in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the internal HashMap is shared with other instances.
    ///
    /// This can be useful for debugging performance characteristics.
    pub fn is_shared(&self) -> bool {
        Rc::strong_count(&self.map) > 1
    }
}

impl Default for InheritedTypeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::type_map::TypeMapKey;

    struct TestKey;
    impl TypeMapKey for TestKey {
        type Value = String;
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = InheritedTypeMap::new();
        map.insert::<TestKey>("hello".to_string());

        assert_eq!(map.get::<TestKey>(), Some(&"hello".to_string()));
    }

    #[test]
    fn test_merge_override() {
        let mut parent = InheritedTypeMap::new();
        parent.insert::<TestKey>("parent".to_string());

        let mut child = InheritedTypeMap::new();
        child.insert::<TestKey>("child".to_string());

        let merged = parent.merge(&child);
        assert_eq!(merged.get::<TestKey>(), Some(&"child".to_string()));
    }

    #[test]
    fn test_merge_partial() {
        let mut parent = InheritedTypeMap::new();
        parent.insert::<TestKey>("parent_value".to_string());

        let child = InheritedTypeMap::new(); // Empty

        let merged = parent.merge(&child);
        assert_eq!(merged.get::<TestKey>(), Some(&"parent_value".to_string()));
        
        // Verify that merge with empty is cheap (Rc not cloned)
        assert!(merged.is_shared());
    }

    #[test]
    fn test_rc_sharing() {
        let mut map = InheritedTypeMap::new();
        map.insert::<TestKey>("shared".to_string());

        let rc1 = map.get_rc::<TestKey>().unwrap();
        let rc2 = map.get_rc::<TestKey>().unwrap();

        // Both Rc should point to the same allocation
        assert!(Rc::ptr_eq(&rc1, &rc2));
    }

    #[test]
    fn test_merge_empty_overrides_is_cheap() {
        let mut parent = InheritedTypeMap::new();
        parent.insert::<TestKey>("value".to_string());

        let child = InheritedTypeMap::new(); // Empty
        let merged = parent.merge(&child);

        // Should share the same HashMap (cheap clone)
        assert!(Rc::ptr_eq(&parent.map, &merged.map));
    }
}
