use crate::map::init_array::InitArrayMap;
use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::Ordinal;

/// Map backed by an array.
///
/// Due to Rust limitations, the array size must be provided as a type parameter.
///
/// # Example
///
/// ```
/// use ordinal_map::map::ArrayMap;
/// use ordinal_map::Ordinal;
/// #[derive(Ordinal)]
/// enum Weather {
///     Sunny,
///     Rainy,
///     Snowy,
/// }
///
/// let mut map = ArrayMap::<_, _, { Weather::ORDINAL_SIZE }>::new();
/// map.insert(Weather::Sunny, "good");
/// map.insert(Weather::Rainy, "it depends");
/// ```
pub struct ArrayMap<K, V, const S: usize> {
    map: InitArrayMap<K, Option<V>, S>,
}

impl<K: Ordinal, V, const S: usize> ArrayMap<K, V, S> {
    /// Create a new map.
    pub fn new() -> Self {
        ArrayMap {
            map: InitArrayMap::new(|_| None),
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
    pub fn keys(&self) -> crate::Iter<K> {
        self.map.keys()
    }

    /// Iterate over the values of the map.
    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.map.values().filter_map(|v| v.as_ref())
    }

    /// Iterate over the values of the map mutably.
    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V> {
        self.map.values_mut().filter_map(|v| v.as_mut())
    }
}
