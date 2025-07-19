use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice;

use crate::array_as_mut::array_as_mut;
use crate::array_builder::ArrayBuilder;
use crate::array_from_iter::array_from_iter;
use crate::map::total::IntoIterArray;
use crate::map::total::Iter;
use crate::map::total::IterMut;
use crate::Ordinal;

/// Like [`OrdinalTotalMap`](crate::map::total::OrdinalTotalMap), but without heap allocation.
///
/// Due to limitations of stable Rust, ordinal size must be passed as a third type parameter.
///
/// # Example
///
/// ```
/// use ordinal_map::map::total::OrdinalTotalArrayMap;
/// use ordinal_map::Ordinal;
///
/// #[derive(Ordinal, Debug)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// let map: OrdinalTotalArrayMap<Color, String, { Color::ORDINAL_SIZE }> =
///     OrdinalTotalArrayMap::new(|color| format!("{color:?}").to_lowercase());
///
/// assert_eq!("green", map[&Color::Green]);
/// ```
#[repr(C)]
pub struct OrdinalTotalArrayMap<K, V, const S: usize> {
    map: [V; S],
    _phantom: PhantomData<K>,
}

impl<K: Ordinal, V, const S: usize> OrdinalTotalArrayMap<K, V, S> {
    const ASSERT: () = {
        assert!(K::ORDINAL_SIZE == S, "K::ORDINAL_SIZE != S");
    };

    /// Create a new map by initializing each value with a function.
    pub fn try_new<E>(mut init: impl FnMut(K) -> Result<V, E>) -> Result<Self, E> {
        const { Self::ASSERT };
        let mut a = ArrayBuilder::new();
        for v in K::all_values() {
            a.push(init(v)?);
        }
        Ok(OrdinalTotalArrayMap {
            map: a.finish(),
            _phantom: PhantomData,
        })
    }

    /// Create a new map by initializing each value with a function.
    pub fn new(mut init: impl FnMut(K) -> V) -> Self {
        const { Self::ASSERT };
        match Self::try_new(move |k| Ok::<_, Infallible>(init(k))) {
            Ok(map) => map,
            Err(infallible) => match infallible {},
        }
    }

    /// Create a new map from an array of values in ordinal order of keys.
    pub fn from_array(array: [V; S]) -> Self {
        OrdinalTotalArrayMap {
            map: array,
            _phantom: PhantomData,
        }
    }

    /// Returns the number of elements in the map, which is
    /// always equal to [`K::ORDINAL_SIZE`](Ordinal::ORDINAL_SIZE).
    pub const fn len(&self) -> usize {
        S
    }

    /// Return true if the map container no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<'a>(&'a self, key: &K) -> &'a V {
        &self.map[key.ordinal()]
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<'a>(&'a mut self, key: &K) -> &'a mut V {
        &mut self.map[key.ordinal()]
    }

    /// Return a new map with values as references to values of the original map.
    pub fn as_ref(&self) -> OrdinalTotalArrayMap<K, &V, S> {
        OrdinalTotalArrayMap::new(|k| self.get(&k))
    }

    /// Return a new map with values as mutable references to values of the original map.
    pub fn as_mut(&mut self) -> OrdinalTotalArrayMap<K, &mut V, S> {
        OrdinalTotalArrayMap::from_array(array_as_mut(&mut self.map))
    }

    /// Combine two maps into one.
    pub fn zip<W>(
        self,
        other: OrdinalTotalArrayMap<K, W, S>,
    ) -> OrdinalTotalArrayMap<K, (V, W), S> {
        OrdinalTotalArrayMap {
            map: array_from_iter(self.map.into_iter().zip(other.map)),
            _phantom: PhantomData,
        }
    }

    /// Map the values of the map.
    pub fn map<W>(self, mut f: impl FnMut(K, V) -> W) -> OrdinalTotalArrayMap<K, W, S> {
        OrdinalTotalArrayMap {
            map: array_from_iter(self.into_iter().map(|(k, v)| f(k, v))),
            _phantom: PhantomData,
        }
    }

    /// Map the values of the map.
    pub fn map_values<W>(self, mut f: impl FnMut(V) -> W) -> OrdinalTotalArrayMap<K, W, S> {
        self.map(|_k, v| f(v))
    }

    /// Iterate over the map.
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(self.map.iter(), 0)
    }

    /// Iterate over the map mutably.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(self.map.iter_mut())
    }

    /// Iterate keys of the map, which is equivalent to iterating all possible values of `K`.
    pub fn keys(&self) -> crate::OrdinalValues<K> {
        K::all_values()
    }

    /// Convert the map into an iterator over keys.
    ///
    /// This operation is identical to [`keys`](OrdinalTotalArrayMap::keys),
    /// but added here for consistency with other map implementations.
    pub fn into_keys(self) -> crate::OrdinalValues<K> {
        self.keys()
    }

    /// Iterate values of the map.
    pub fn values<'a>(&'a self) -> slice::Iter<'a, V> {
        self.map.iter()
    }

    /// Values arrays.
    pub fn values_array(&self) -> &[V; S] {
        &self.map
    }

    /// Iterate mutable references to values of the map.
    pub fn values_mut<'a>(&'a mut self) -> slice::IterMut<'a, V> {
        self.map.iter_mut()
    }

    /// Values arrays.
    pub fn values_array_mut(&mut self) -> &mut [V; S] {
        &mut self.map
    }

    /// Convert the map into an array of values.
    pub fn into_values(self) -> [V; S] {
        self.map
    }

    /// Insert a value into the map, returning the previous value.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> V {
        mem::replace(&mut self.map[key.ordinal()], value)
    }
}

impl<'a, K: Ordinal, V, const S: usize> Index<&'a K> for OrdinalTotalArrayMap<K, V, S> {
    type Output = V;

    fn index(&self, index: &'a K) -> &Self::Output {
        &self.map[index.ordinal()]
    }
}

impl<'a, K: Ordinal, V, const S: usize> IndexMut<&'a K> for OrdinalTotalArrayMap<K, V, S> {
    fn index_mut(&mut self, index: &'a K) -> &mut Self::Output {
        &mut self.map[index.ordinal()]
    }
}

impl<K, V: Clone, const S: usize> Clone for OrdinalTotalArrayMap<K, V, S> {
    fn clone(&self) -> Self {
        OrdinalTotalArrayMap {
            map: self.map.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<K: Ordinal, V: Default, const S: usize> Default for OrdinalTotalArrayMap<K, V, S> {
    fn default() -> Self {
        OrdinalTotalArrayMap::new(|_| V::default())
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for OrdinalTotalArrayMap<K, V, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Ordinal, V, const S: usize> IntoIterator for OrdinalTotalArrayMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIterArray<K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterArray::new(self.map.into_iter())
    }
}

impl<'a, K: Ordinal, V, const S: usize> IntoIterator for &'a OrdinalTotalArrayMap<K, V, S> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
