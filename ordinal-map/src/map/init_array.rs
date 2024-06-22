use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Index;

use crate::array_builder::ArrayBuilder;
use crate::map::InitIter;
use crate::map::InitIterMut;
use crate::Ordinal;

/// Like [`InitMap`](crate::InitMap), but without heap allocation.
///
/// Due to limitations of stable Rust, ordinal size must be passed as a third type parameter.
///
/// # Example
///
/// ```
/// use ordinal_map::map::InitArrayMap;
/// use ordinal_map::Ordinal;
///
/// #[derive(Ordinal, Debug)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// let map: InitArrayMap<Color, String, { Color::ORDINAL_SIZE }> =
///     InitArrayMap::new(|color| format!("{color:?}").to_lowercase());
///
/// assert_eq!("green", map[Color::Green]);
/// ```
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

    pub fn iter<'a>(&'a self) -> InitIter<'a, K, V> {
        InitIter::new(self.map.iter().enumerate())
    }

    pub fn iter_mut<'a>(&'a mut self) -> InitIterMut<'a, K, V> {
        InitIterMut::new(self.map.iter_mut().enumerate())
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

impl<K: Ordinal, V, const S: usize> Index<K> for InitArrayMap<K, V, S> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.map[index.ordinal()]
    }
}

impl<'a, K: Ordinal, V, const S: usize> Index<&'a K> for InitArrayMap<K, V, S> {
    type Output = V;

    fn index(&self, index: &'a K) -> &Self::Output {
        &self.map[index.ordinal()]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
