use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::map::iter::Drain;
use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::map::iter::Keys;
use crate::map::iter::Values;
use crate::map::iter::ValuesMut;
use crate::map::Entry;
use crate::map::IntoIter;
use crate::map::TotalIntoIter;
use crate::map::TotalIter;
use crate::map::TotalIterMut;
use crate::Ordinal;

/// Map [`Ordinal`](crate::Ordinal) keys to values.
/// Map operations are constant time
/// (provided that [`K::ordinal()`](Ordinal::ordinal) is constant time).
///
/// This implementation allocates a boxed slice `[Option<V>; K::ORDINAL_SIZE]`
/// on the first insertion. For non-allocating map, consider using
/// [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct OrdinalMap<K, V> {
    // Empty when the map is just created.
    map: Box<[Option<V>]>,
    _phantom: PhantomData<K>,
}

impl<K: Ordinal, V> OrdinalMap<K, V> {
    /// Create a new empty map.
    /// This operation does not allocate memory, but first insertion allocates the whole map.
    #[inline]
    pub fn new() -> Self {
        OrdinalMap {
            map: Box::default(),
            _phantom: PhantomData,
        }
    }

    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key.ordinal())?.as_ref()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key.ordinal())?.as_mut()
    }

    /// Returns `true` if the map contains the key.
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.get(key.ordinal()).is_some()
    }

    /// Returns the number of elements in the map. This is an `O(K::ORDINAL_SIZE)` operation.
    #[inline]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    fn init_full_map(&mut self) {
        if self.map.is_empty() {
            let mut map = Vec::with_capacity(K::ORDINAL_SIZE);
            for _ in 0..K::ORDINAL_SIZE {
                map.push(None);
            }
            self.map = map.into_boxed_slice();
        }
    }

    /// Insert a value into the map, returning the previous value if it existed.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.init_full_map();
        self.map[key.ordinal()].replace(value)
    }

    /// Get an entry in the map for the given key.
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self.init_full_map();
        let entry = &mut self.map[key.ordinal()];
        Entry::new(key, entry)
    }

    /// Remove a value from the map, returning it if it existed.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key.ordinal())?.take()
    }

    /// Iterate over the map.
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(TotalIter::new(self.map.iter(), 0))
    }

    /// Iterate over the map mutably.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut::new(TotalIterMut::new(self.map.iter_mut()))
    }

    /// Iterate over the keys of the map.
    #[inline]
    pub fn keys(&self) -> Keys<K, V> {
        Keys::new(self.iter())
    }

    /// Iterate over the values of the map.
    #[inline]
    pub fn values(&self) -> Values<K, V> {
        Values::new(self.iter())
    }

    /// Iterate over the mutable references to the values of the map.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut::new(self.iter_mut())
    }

    /// Clears the map, returning all key-value pairs as an iterator.
    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain::new(TotalIterMut::new(self.map.iter_mut()))
    }

    /// Remove all elements from the map.
    #[inline]
    pub fn clear(&mut self) {
        self.drain();
    }
}

impl<K: Ordinal, V> Default for OrdinalMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ordinal, V> FromIterator<(K, V)> for OrdinalMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = OrdinalMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

impl<K, V: Clone> Clone for OrdinalMap<K, V> {
    fn clone(&self) -> Self {
        OrdinalMap {
            map: self.map.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for OrdinalMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Ordinal, V> IntoIterator for OrdinalMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(TotalIntoIter::new(self.map.into_vec().into_iter()))
    }
}

impl<'a, K: Ordinal, V> IntoIterator for &'a OrdinalMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::map::OrdinalMap;

    #[quickcheck]
    fn qc(values: Vec<(u8, u32)>, check: Vec<u8>) {
        let mut map: OrdinalMap<u8, u32> = OrdinalMap::new();
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
