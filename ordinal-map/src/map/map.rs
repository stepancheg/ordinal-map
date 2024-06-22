use std::marker::PhantomData;

use crate::Ordinal;

pub struct Map<T, V> {
    map: Box<[Option<V>]>,
    _phantom: PhantomData<T>,
}

impl<K: Ordinal, V> Map<K, V> {
    #[inline]
    pub fn new() -> Self {
        Map {
            map: Box::default(),
            _phantom: PhantomData,
        }
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.map.get(key.ordinal())?.as_ref()
    }

    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.get_mut(key.ordinal())?.as_mut()
    }

    pub fn len(&self) -> usize {
        self.map.iter().filter(|x| x.is_some()).count()
    }

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

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.get_mut(key.ordinal())?.take()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (K, &'a V)> {
        crate::Iter::<K>::new()
            .zip(self.map.iter())
            .filter_map(|(k, v)| v.as_ref().map(|v| (k, v)))
    }

    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.iter().map(|(k, _)| k)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
