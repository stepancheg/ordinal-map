use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Index;
use std::slice;

use crate::map::init_iter::InitIter;
use crate::map::init_iter::InitIterMut;
use crate::Ordinal;

/// Map implementation where all values must be initialized at creation.
/// (But can be changed later.)
///
/// This version of map allocates values on the heap in a single contiguous block.
/// For a version that doesn't require heap allocation,
/// see [`InitArrayMap`](crate::map::InitArrayMap).
pub struct InitMap<T, V> {
    map: Box<[V]>,
    _phantom: PhantomData<T>,
}

impl<K: Ordinal, V> InitMap<K, V> {
    /// Create a new map by initializing each value with a function.
    pub fn try_new<E>(mut init: impl FnMut(K) -> Result<V, E>) -> Result<Self, E> {
        let mut map = Vec::with_capacity(K::ORDINAL_SIZE);
        for v in crate::Iter::<K>::new() {
            map.push(init(v)?);
        }
        Ok(InitMap {
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

    /// Iterate values of the map.
    pub fn values<'a>(&'a self) -> slice::Iter<'a, V> {
        self.map.iter()
    }

    /// Iterate mutable references to values of the map.
    pub fn values_mut<'a>(&'a mut self) -> slice::IterMut<'a, V> {
        self.map.iter_mut()
    }

    /// Iterate over the map.
    pub fn iter<'a>(&'a self) -> InitIter<'a, K, V> {
        InitIter::new(self.map.iter().enumerate())
    }

    /// Iterate over the map mutably.
    pub fn iter_mut<'a>(&'a mut self) -> InitIterMut<'a, K, V> {
        InitIterMut::new(self.map.iter_mut().enumerate())
    }
}

impl<K: Ordinal, V: Default> Default for InitMap<K, V> {
    fn default() -> Self {
        InitMap::new(|_| V::default())
    }
}

impl<K: Ordinal, V> Index<K> for InitMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.get(&key)
    }
}

impl<'a, K: Ordinal, V> Index<&'a K> for InitMap<K, V> {
    type Output = V;

    fn index(&self, key: &'a K) -> &Self::Output {
        self.get(key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
