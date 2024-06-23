use crate::map::init_array::InitArrayMap;
use crate::map::iter::Iter;
use crate::map::iter::IterMut;
use crate::Ordinal;

pub struct ArrayMap<K, V, const S: usize> {
    map: InitArrayMap<K, Option<V>, S>,
}

impl<K: Ordinal, V, const S: usize> ArrayMap<K, V, S> {
    pub fn new() -> Self {
        ArrayMap {
            map: InitArrayMap::new(|_| None),
        }
    }

    pub const fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key).as_ref()
    }

    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key).as_mut()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.get_mut(&key).replace(value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key).take()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(self.map.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(self.map.iter_mut())
    }

    pub fn keys(&self) -> crate::Iter<K> {
        self.map.keys()
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.map.values().filter_map(|v| v.as_ref())
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V> {
        self.map.values_mut().filter_map(|v| v.as_mut())
    }
}
