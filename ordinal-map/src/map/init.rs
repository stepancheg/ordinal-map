use std::convert::Infallible;
use std::marker::PhantomData;

use crate::Ordinal;

pub struct InitMap<T, V> {
    map: Box<[V]>,
    _phantom: PhantomData<T>,
}

impl<K: Ordinal, V> InitMap<K, V> {
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

    pub const fn len(&self) -> usize {
        K::ORDINAL_SIZE
    }

    pub fn new(mut init: impl FnMut(K) -> V) -> Self {
        match Self::try_new(move |k| Ok::<_, Infallible>(init(k))) {
            Ok(map) => map,
            Err(infallible) => match infallible {},
        }
    }

    pub fn get<'a>(&'a self, key: &K) -> &'a V {
        &self.map[key.ordinal()]
    }

    pub fn get_mut<'a>(&'a mut self, key: &K) -> &'a mut V {
        &mut self.map[key.ordinal()]
    }

    pub fn keys(&self) -> crate::Iter<K> {
        crate::Iter::<K>::new()
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.map.iter()
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V> {
        self.map.iter_mut()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (K, &'a V)> {
        crate::Iter::<K>::new().zip(self.map.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (K, &'a mut V)> {
        crate::Iter::<K>::new().zip(self.map.iter_mut())
    }
}

impl<K: Ordinal, V: Default> Default for InitMap<K, V> {
    fn default() -> Self {
        InitMap::new(|_| V::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
