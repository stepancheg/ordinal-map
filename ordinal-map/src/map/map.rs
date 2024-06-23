use std::marker::PhantomData;

use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::map::InitIter;
use crate::map::InitIterMut;
use crate::Ordinal;

/// Map [`Ordinal`](crate::Ordinal) keys to values.
/// Map operations are constant time
/// (provided that [`K::ordinal()`](Ordinal::ordinal) is constant time).
///
/// This implementation allocates a boxed slice `[Option<V>; K::ORDINAL_SIZE]`
/// on the first insertion. For non-allocating map, consider using
/// [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct OrdinalMap<K, V> {
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
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key.ordinal())?.as_ref()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key.ordinal())?.as_mut()
    }

    /// Returns the number of elements in the map. This is an `O(K::ORDINAL_SIZE)` operation.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Insert a value into the map, returning the previous value if it existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(v) = self.map.get_mut(key.ordinal()) {
            v.replace(value)
        } else {
            let mut map = Vec::with_capacity(K::ORDINAL_SIZE);
            for _ in 0..K::ORDINAL_SIZE {
                map.push(None);
            }
            map[key.ordinal()] = Some(value);
            self.map = map.into_boxed_slice();
            None
        }
    }

    /// Remove a value from the map, returning it if it existed.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key.ordinal())?.take()
    }

    /// Iterate over the map.
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(InitIter::new(self.map.iter().enumerate()))
    }

    /// Iterate over the map mutably.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(InitIterMut::new(self.map.iter_mut().enumerate()))
    }

    /// Iterate over the keys of the map.
    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.iter().map(|(k, _)| k)
    }

    /// Iterate over the values of the map.
    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }
}

impl<K: Ordinal, V> Default for OrdinalMap<K, V> {
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
