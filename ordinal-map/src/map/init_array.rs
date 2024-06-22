use std::convert::Infallible;
use std::marker::PhantomData;

use crate::array_builder::ArrayBuilder;
use crate::Ordinal;

/// Like [`InitMap`](crate::InitMap), but without heap allocation.
///
/// Due to limitations of stable Rust, ordinal size must be passed as a third type parameter.
#[repr(C)]
pub struct InitArrayMap<K, V, const S: usize> {
    map: [V; S],
    _phantom: PhantomData<K>,
}

impl<K: Ordinal, V, const S: usize> InitArrayMap<K, V, S> {
    const ASSERT: () = {
        assert!(K::ORDINAL_SIZE == S, "K::ORDINAL_SIZE != S");
    };

    pub fn try_new<E>(mut init: impl FnMut(K) -> Result<V, E>) -> Result<Self, E> {
        const { Self::ASSERT };
        let mut a = ArrayBuilder::new();
        for v in crate::Iter::<K>::new() {
            a.push(init(v)?);
        }
        Ok(InitArrayMap {
            map: a.finish(),
            _phantom: PhantomData,
        })
    }

    pub fn new(mut init: impl FnMut(K) -> V) -> Self {
        const { Self::ASSERT };
        match Self::try_new(move |k| Ok::<_, Infallible>(init(k))) {
            Ok(map) => map,
            Err(infallible) => match infallible {},
        }
    }

    pub const fn len(&self) -> usize {
        S
    }

    pub fn get<'a>(&'a self, key: &K) -> &'a V {
        &self.map[key.ordinal()]
    }

    pub fn get_mut<'a>(&'a mut self, key: &K) -> &'a mut V {
        &mut self.map[key.ordinal()]
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (K, &'a V)> {
        crate::Iter::<K>::new().zip(self.map.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (K, &'a mut V)> {
        crate::Iter::<K>::new().zip(self.map.iter_mut())
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
