use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData; // ← Dodane

/// Trait defining a key for the TypeMap.
///
/// Each type implementing this trait represents a unique slot in the map.
/// The associated type `Value` determines what type of data is stored under this key.
pub trait TypeMapKey {
    /// The type of the value stored under this key.
    type Value: 'static;
}

/// A map storing values indexed by their type key.
///
/// This implementation is designed for single-threaded use.
/// It does not require stored values to implement `Send` or `Sync`.
pub struct TypeMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    /// Creates a new, empty TypeMap.
    pub fn new() -> Self {
        TypeMap {
            map: HashMap::new(),
        }
    }

    /// Inserts a value associated with the key `K`.
    ///
    /// Returns the previous value associated with this key, if any.
    pub fn insert<K: TypeMapKey + 'static>(&mut self, value: K::Value) -> Option<K::Value> {
        let id = TypeId::of::<K>();
        let old = self.map.insert(id, Box::new(value));
        old.and_then(|b| b.downcast::<K::Value>().ok().map(|b| *b))
    }

    /// Gets an immutable reference to the value associated with the key `K`.
    pub fn get<K: TypeMapKey + 'static>(&self) -> Option<&K::Value> {
        self.map
            .get(&TypeId::of::<K>())
            .and_then(|boxed| boxed.downcast_ref::<K::Value>())
    }

    /// Gets a mutable reference to the value associated with the key `K`.
    pub fn get_mut<K: TypeMapKey + 'static>(&mut self) -> Option<&mut K::Value> {
        self.map
            .get_mut(&TypeId::of::<K>())
            .and_then(|boxed| boxed.downcast_mut::<K::Value>())
    }

    /// Removes the value associated with the key `K` from the map.
    pub fn remove<K: TypeMapKey + 'static>(&mut self) -> Option<K::Value> {
        self.map
            .remove(&TypeId::of::<K>())
            .and_then(|b| b.downcast::<K::Value>().ok().map(|b| *b))
    }

    /// Checks if the key `K` exists in the map.
    pub fn contains<K: TypeMapKey + 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<K>())
    }

    /// Returns an entry for the key `K` for in-place manipulation.
    ///
    /// This is analogous to `HashMap::entry` and allows for lazy initialization.
    pub fn entry<K: TypeMapKey + 'static>(&mut self) -> Entry<'_, K> {
        let id = TypeId::of::<K>();
        if self.map.contains_key(&id) {
            Entry::Occupied(OccupiedEntry {
                map: &mut self.map,
                id,
                _phantom: PhantomData,
            })
        } else {
            Entry::Vacant(VacantEntry {
                map: &mut self.map,
                id,
                _phantom: PhantomData,
            })
        }
    }
}

impl Default for TypeMap {
    fn default() -> Self {
        Self::new()
    }
}

/// An entry in the TypeMap, which can be either occupied or vacant.
pub enum Entry<'a, K: TypeMapKey + 'static> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K>),
}

impl<'a, K: TypeMapKey + 'static> Entry<'a, K> {
    /// Ensures a value is in the entry by inserting the default if empty.
    ///
    /// Returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: K::Value) -> &'a mut K::Value {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of a function if empty.
    ///
    /// This is useful for lazy initialization where creating the value is expensive.
    pub fn or_insert_with<F: FnOnce() -> K::Value>(self, f: F) -> &'a mut K::Value {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(f()),
        }
    }

    /// Returns a reference to the value in the entry, or inserts a default.
    pub fn or_default(self) -> &'a mut K::Value
    where
        K::Value: Default,
    {
        self.or_insert_with(Default::default)
    }

    /// Modifies the value if the entry is occupied, or does nothing if vacant.
    ///
    /// Returns the entry for further chaining.
    pub fn and_modify<F: FnOnce(&mut K::Value)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                map: entry.map,
                id: entry.id,
                _phantom: PhantomData,
            }),
        }
    }
}

/// An occupied entry in the TypeMap.
pub struct OccupiedEntry<'a, K: TypeMapKey + 'static> {
    map: &'a mut HashMap<TypeId, Box<dyn Any>>,
    id: TypeId,
    _phantom: PhantomData<K>,
}

impl<'a, K: TypeMapKey + 'static> OccupiedEntry<'a, K> {
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &K::Value {
        self.map
            .get(&self.id)
            .and_then(|b| b.downcast_ref::<K::Value>())
            .expect("Type safety violated in OccupiedEntry")
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut K::Value {
        self.map
            .get_mut(&self.id)
            .and_then(|b| b.downcast_mut::<K::Value>())
            .expect("Type safety violated in OccupiedEntry")
    }

    /// Converts the entry into a mutable reference to the value.
    pub fn into_mut(self) -> &'a mut K::Value {
        self.map
            .get_mut(&self.id)
            .and_then(|b| b.downcast_mut::<K::Value>())
            .expect("Type safety violated in OccupiedEntry")
    }

    /// Removes the entry from the map and returns the value.
    pub fn remove(self) -> K::Value {
        self.map
            .remove(&self.id)
            .and_then(|b| b.downcast::<K::Value>().ok().map(|b| *b))
            .expect("Type safety violated in OccupiedEntry")
    }
}

/// A vacant entry in the TypeMap.
pub struct VacantEntry<'a, K: TypeMapKey + 'static> {
    map: &'a mut HashMap<TypeId, Box<dyn Any>>,
    id: TypeId,
    _phantom: PhantomData<K>,
}

impl<'a, K: TypeMapKey + 'static> VacantEntry<'a, K> {
    /// Inserts a value into the vacant entry and returns a mutable reference to it.
    pub fn insert(self, value: K::Value) -> &'a mut K::Value {
        self.map.insert(self.id, Box::new(value));

        self.map
            .get_mut(&self.id)
            .and_then(|b| b.downcast_mut::<K::Value>())
            .expect("Just inserted value should exist")
    }
}
