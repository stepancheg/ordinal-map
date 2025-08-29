use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::Ordinal;

/// [`Ordinal`] stored in [`AtomicUsize`].
///
/// # Example
///
/// ```
/// # use std::sync::atomic::Ordering;
/// # use ordinal_map::AtomicOrdinal;
/// # use ordinal_map_derive::Ordinal;
///
/// #[derive(Ordinal)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// let a = AtomicOrdinal::new(Color::Red);
/// let ret = a.compare_exchange(
///     Color::Red,
///     Color::Blue,
///     Ordering::Relaxed,
///     Ordering::Relaxed,
/// );
/// assert!(ret.is_ok());
/// ```
pub struct AtomicOrdinal<T> {
    atomic: AtomicUsize,
    _marker: PhantomData<T>,
}

impl<T: Ordinal + Debug> Debug for AtomicOrdinal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let v = self.load(Ordering::Relaxed);
        Debug::fmt(&v, f)
    }
}

impl<T: Ordinal> AtomicOrdinal<T> {
    /// Store an ordinal value inside [`AtomicUsize`].
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            atomic: AtomicUsize::new(value.ordinal()),
            _marker: PhantomData,
        }
    }

    /// Load an ordinal value from [`AtomicUsize`].
    pub fn load(&self, ordering: Ordering) -> T {
        T::from_ordinal(self.atomic.load(ordering)).unwrap()
    }

    /// Store the ordinal value in [`AtomicUsize`].
    pub fn store(&self, value: T, ordering: Ordering) {
        self.atomic.store(value.ordinal(), ordering);
    }

    /// Compare exchange.
    pub fn compare_exchange(
        &self,
        old: T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, T> {
        let old_ord = old.ordinal();
        let new_ord = new.ordinal();
        let res = self
            .atomic
            .compare_exchange(old_ord, new_ord, success, failure);
        match res {
            Ok(v) => Ok(T::from_ordinal(v).unwrap()),
            Err(v) => Err(T::from_ordinal(v).unwrap()),
        }
    }
}
