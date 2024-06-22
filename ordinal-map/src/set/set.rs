use std::marker::PhantomData;

use crate::Ordinal;

enum SetImpl {
    Small(u64),
    Large(Box<[u64]>),
}

impl SetImpl {}

pub struct Set<T: Ordinal> {
    set: SetImpl,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Set<T> {
    const IS_SMALL: bool = T::ORDINAL_SIZE <= u64::BITS as usize;

    #[inline]
    pub fn new() -> Self {
        Set {
            set: if Self::IS_SMALL {
                SetImpl::Small(0)
            } else {
                SetImpl::Large(Vec::<u64>::new().into_boxed_slice())
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        match (Self::IS_SMALL, &self.set) {
            (true, SetImpl::Small(set)) => set & (1 << ordinal.ordinal()) != 0,
            (false, SetImpl::Large(set)) => {
                let Some(bits) = set.get(ordinal.ordinal() / u64::BITS as usize) else {
                    return false;
                };
                bits & (1 << (ordinal.ordinal() % u64::BITS as usize)) != 0
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
