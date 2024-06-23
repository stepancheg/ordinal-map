/// Reference to the occupied entry in [`OrdinalMap`](crate::map::OrdinalMap)
/// or [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct OccupiedEntry<'a, K, V> {
    key: K,
    /// We know that the entry is `Some`.
    entry: &'a mut Option<V>,
}

/// Reference to the vacant entry in [`OrdinalMap`](crate::map::OrdinalMap)
/// or [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct VacantEntry<'a, K, V> {
    key: K,
    /// We know that the entry is `None`.
    entry: &'a mut Option<V>,
}

/// Entry API.
///
/// Operations with [`OrdinalMap`](crate::map::OrdinalMap)
/// and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap) are
/// constant time, and this API is provided for convenience.
pub enum Entry<'a, K, V> {
    /// Occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
    /// Vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    /// Get the key of the entry.
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Get a reference to the value in the entry.
    #[inline]
    pub fn get(&self) -> &V {
        self.entry.as_ref().unwrap()
    }

    /// Get a mutable reference to the value in the entry.
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        self.entry.as_mut().unwrap()
    }
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    /// Get the key of the entry.
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Insert a value into the map.
    #[inline]
    pub fn insert(self, value: V) {
        *self.entry = Some(value);
    }
}

impl<'a, K, V> Entry<'a, K, V> {
    pub(crate) fn new(key: K, entry: &'a mut Option<V>) -> Self {
        if entry.is_some() {
            Entry::Occupied(OccupiedEntry { key, entry })
        } else {
            Entry::Vacant(VacantEntry { key, entry })
        }
    }

    /// Get the key of the entry.
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    /// Convert the entry to an occupied entry.
    #[inline]
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.entry.as_mut().unwrap(),
            Entry::Vacant(entry) => entry.entry.insert(value),
        }
    }
}
