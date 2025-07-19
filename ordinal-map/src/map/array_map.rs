use std::fmt;
use std::fmt::Debug;

use crate::map::iter::IntoIterArray;
use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::map::iter::ValuesMut;
use crate::map::total::array_map::OrdinalTotalArrayMap;
use crate::map::Drain;
use crate::map::Entry;
use crate::map::Keys;
use crate::map::Values;
use crate::Ordinal;

/// Map backed by an array, allocated on the stack.
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
///
/// This map is sparse (not every key has an associated value).
/// For a total map, see [`OrdinalTotalMap`](crate::map::total::OrdinalTotalMap)
/// and [`OrdinalTotalArrayMap`](OrdinalTotalArrayMap).
pub struct OrdinalArrayMap<K, V, const S: usize> {
    map: OrdinalTotalArrayMap<K, Option<V>, S>,
}

impl<K: Ordinal, V, const S: usize> OrdinalArrayMap<K, V, S> {
    /// Create a new map.
    #[inline]
    pub fn new() -> Self {
        OrdinalArrayMap {
            map: OrdinalTotalArrayMap::new(|_| None),
        }
    }

    /// The number of elements in the map.
    ///
    /// This operation is `O(K::ORDINAL_SIZE)`.
    #[inline]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return true if the map container no elements.
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// Look up a value by key.
    #[inline]
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key).as_ref()
    }

    /// Look up a value by key.
    #[inline]
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key).as_mut()
    }

    /// Check if the map contains a key.
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Insert a value into the map, returning the previous value if it existed.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.get_mut(&key).replace(value)
    }

    /// Get an entry in the map for the given key.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        let entry = &mut self.map[&key];
        Entry::new(key, entry)
    }

    /// Remove a value from the map, returning it if it existed.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key).take()
    }

    /// Iterate over the map.
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(self.map.iter())
    }

    /// Iterate over the map mutably.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(self.map.iter_mut())
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

    /// Iterate over the values of the map mutably.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut::new(self.iter_mut())
    }

    /// Remove all elements from the map.
    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain::new(self.map.iter_mut())
    }

    /// Remove all elements from the map.
    #[inline]
    pub fn clear(&mut self) {
        self.drain();
    }

    /// Retain only the elements specified by the predicate.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        for (key, value_opt) in self.map.iter_mut() {
            if let Some(value) = value_opt {
                if !f(key, value) {
                    *value_opt = None;
                }
            }
        }
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

impl<K: Ordinal, V, const S: usize> IntoIterator for OrdinalArrayMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIterArray<K, V, S>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterArray::new(self.map.into_iter())
    }
}

impl<'a, K: Ordinal, V, const S: usize> IntoIterator for &'a OrdinalArrayMap<K, V, S> {
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
    use std::collections::HashSet;

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

    #[quickcheck]
    fn qc_retain(values: Vec<(u8, u32)>, retain: Vec<u8>) -> bool {
        let retain: HashSet<u8> = HashSet::from_iter(retain);

        let mut map = OrdinalArrayMap::<u8, u32, { u8::ORDINAL_SIZE }>::from_iter(values.clone());
        let mut control: HashMap<u8, u32> = HashMap::from_iter(values);

        map.retain(|key, _| retain.contains(&key));
        control.retain(|key, _| retain.contains(key));

        control == HashMap::from_iter(map)
    }
}
