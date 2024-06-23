use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice;

use crate::map::total_iter::TotalIter;
use crate::map::total_iter::TotalIterMut;
use crate::map::TotalIntoIter;
use crate::Ordinal;

/// Map implementation where all values must be initialized at creation.
///
/// This version of map allocates values on the heap in a single contiguous block.
/// For a version that doesn't require heap allocation,
/// see [`InitArrayMap`](crate::map::OrdinalTotalArrayMap).
pub struct OrdinalTotalMap<T, V> {
    map: Box<[V]>,
    _phantom: PhantomData<T>,
}

impl<K: Ordinal, V> OrdinalTotalMap<K, V> {
    /// Create a new map by initializing each value with a function.
    pub fn try_new<E>(mut init: impl FnMut(K) -> Result<V, E>) -> Result<Self, E> {
        let mut map = Vec::with_capacity(K::ORDINAL_SIZE);
        for v in crate::Iter::<K>::new() {
            map.push(init(v)?);
        }
        Ok(OrdinalTotalMap {
            map: map.into_boxed_slice(),
            _phantom: PhantomData,
        })
    }

    /// Create a new map by initializing each value with a function.
    pub fn new(mut init: impl FnMut(K) -> V) -> Self {
        match Self::try_new(move |k| Ok::<_, Infallible>(init(k))) {
            Ok(map) => map,
            Err(infallible) => match infallible {},
        }
    }

    /// Returns the number of elements in the map, which is
    /// always equal to [`K::ORDINAL_SIZE`](Ordinal::ORDINAL_SIZE).
    pub const fn len(&self) -> usize {
        K::ORDINAL_SIZE
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<'a>(&'a self, key: &K) -> &'a V {
        &self.map[key.ordinal()]
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<'a>(&'a mut self, key: &K) -> &'a mut V {
        &mut self.map[key.ordinal()]
    }

    /// Iterate keys of the map, which is equivalent to iterating all possible values of `K`.
    pub fn keys(&self) -> crate::Iter<K> {
        crate::Iter::<K>::new()
    }

    /// Iterate values of the map, which is equivalent to iterating all possible values of `K`.
    pub fn into_keys(self) -> crate::Iter<K> {
        crate::Iter::<K>::new()
    }

    /// Iterate values of the map.
    pub fn values<'a>(&'a self) -> slice::Iter<'a, V> {
        self.map.iter()
    }

    /// Iterate mutable references to values of the map.
    pub fn values_mut<'a>(&'a mut self) -> slice::IterMut<'a, V> {
        self.map.iter_mut()
    }

    /// Obtain the values from the map.
    pub fn into_values(self) -> Box<[V]> {
        self.map
    }

    /// Iterate over the map.
    #[inline]
    pub fn iter<'a>(&'a self) -> TotalIter<'a, K, V> {
        TotalIter::new(self.map.iter(), 0)
    }

    /// Iterate over the map mutably.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> TotalIterMut<'a, K, V> {
        TotalIterMut::new(self.map.iter_mut())
    }

    // TODO: add insert
}

impl<K: Ordinal, V: Default> Default for OrdinalTotalMap<K, V> {
    fn default() -> Self {
        OrdinalTotalMap::new(|_| V::default())
    }
}

impl<K: Ordinal, V> Index<K> for OrdinalTotalMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.get(&key)
    }
}

impl<'a, K: Ordinal, V> Index<&'a K> for OrdinalTotalMap<K, V> {
    type Output = V;

    fn index(&self, key: &'a K) -> &Self::Output {
        self.get(key)
    }
}

impl<K: Ordinal, V> IndexMut<K> for OrdinalTotalMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.get_mut(&key)
    }
}

impl<'a, K: Ordinal, V> IndexMut<&'a K> for OrdinalTotalMap<K, V> {
    fn index_mut(&mut self, key: &'a K) -> &mut Self::Output {
        self.get_mut(key)
    }
}

impl<K, V: Clone> Clone for OrdinalTotalMap<K, V> {
    fn clone(&self) -> Self {
        OrdinalTotalMap {
            map: self.map.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for OrdinalTotalMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Ordinal, V> IntoIterator for OrdinalTotalMap<K, V> {
    type Item = (K, V);
    type IntoIter = TotalIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        TotalIntoIter::new(self.map.into_vec().into_iter())
    }
}

impl<'a, K: Ordinal, V> IntoIterator for &'a OrdinalTotalMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = TotalIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
