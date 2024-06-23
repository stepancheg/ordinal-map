use std::fmt;
use std::fmt::Debug;

use crate::map::init_array::OrdinalInitArrayMap;
use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::map::iter::ValuesMut;
use crate::map::Keys;
use crate::map::Values;
use crate::Ordinal;

/// Map backed by an array.
///
/// Due to Rust limitations, the array size must be provided as a type parameter.
///
/// # Example
///
/// ```
/// use ordinal_map::map::OrdinalArrayMap;
/// use ordinal_map::Ordinal;
/// #[derive(Ordinal)]
/// enum Weather {
///     Sunny,
///     Rainy,
///     Snowy,
/// }
///
/// let mut map = OrdinalArrayMap::<_, _, { Weather::ORDINAL_SIZE }>::new();
/// map.insert(Weather::Sunny, "good");
/// map.insert(Weather::Rainy, "it depends");
/// ```
pub struct OrdinalArrayMap<K, V, const S: usize> {
    map: OrdinalInitArrayMap<K, Option<V>, S>,
}

impl<K: Ordinal, V, const S: usize> OrdinalArrayMap<K, V, S> {
    /// Create a new map.
    pub fn new() -> Self {
        OrdinalArrayMap {
            map: OrdinalInitArrayMap::new(|_| None),
        }
    }

    /// The number of elements in the map.
    ///
    /// This operation is `O(K::ORDINAL_SIZE)`.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Look up a value by key.
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key).as_ref()
    }

    /// Look up a value by key.
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key).as_mut()
    }

    /// Insert a value into the map, returning the previous value if it existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.get_mut(&key).replace(value)
    }

    /// Remove a value from the map, returning it if it existed.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key).take()
    }

    /// Iterate over the map.
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(self.map.iter())
    }

    /// Iterate over the map mutably.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(self.map.iter_mut())
    }

    /// Iterate over the keys of the map.
    pub fn keys(&self) -> Keys<K, V> {
        Keys::new(self.iter())
    }

    /// Iterate over the values of the map.
    pub fn values(&self) -> Values<K, V> {
        Values::new(self.iter())
    }

    /// Iterate over the values of the map mutably.
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut::new(self.iter_mut())
    }
}

impl<K: Ordinal, V, const S: usize> Default for OrdinalArrayMap<K, V, S> {
    #[inline]
    fn default() -> Self {
        OrdinalArrayMap::new()
    }
}

impl<K: Ordinal, V, const S: usize> FromIterator<(K, V)> for OrdinalArrayMap<K, V, S> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = OrdinalArrayMap::new();
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<K: Ordinal, V: Clone, const S: usize> Clone for OrdinalArrayMap<K, V, S> {
    fn clone(&self) -> Self {
        OrdinalArrayMap {
            map: self.map.clone(),
        }
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for OrdinalArrayMap<K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::map::OrdinalArrayMap;
    use crate::Ordinal;

    #[quickcheck]
    fn qc(values: Vec<(u8, u32)>, check: Vec<u8>) {
        let mut map: OrdinalArrayMap<u8, u32, { u8::ORDINAL_SIZE }> = OrdinalArrayMap::new();
        let mut control: HashMap<u8, u32> = HashMap::new();

        for (key, value) in &values {
            let control_inserted = control.insert(*key, *value);
            let inserted = map.insert(*key, *value);
            assert_eq!(control_inserted, inserted);
            assert_eq!(control.len(), map.len());
        }

        for key in &check {
            assert_eq!(control.get(key), map.get(key));
        }
    }
}
