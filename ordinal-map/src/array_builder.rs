use std::array;
use std::mem;
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub(crate) struct ArrayNotFilled;

pub(crate) struct ArrayBuilder<T, const S: usize> {
    array: [MaybeUninit<T>; S],
    filled: usize,
}

impl<T, const S: usize> Drop for ArrayBuilder<T, S> {
    fn drop(&mut self) {
        for i in 0..self.filled {
            unsafe {
                self.array[i].assume_init_drop();
            }
        }
    }
}

impl<T, const S: usize> ArrayBuilder<T, S> {
    pub(crate) fn new() -> Self {
        ArrayBuilder {
            array: array::from_fn(|_| MaybeUninit::uninit()),
            filled: 0,
        }
    }

    pub(crate) fn push(&mut self, value: T) {
        assert!(self.filled < S);
        unsafe {
            self.array
                .as_mut_ptr()
                .cast::<T>()
                .add(self.filled)
                .write(value);
        }
        self.filled += 1;
    }

    fn try_finish(mut self) -> Result<[T; S], ArrayNotFilled> {
        if self.filled != S {
            unsafe {
                ptr::drop_in_place(&mut self.array[self.filled..S]);
            }
            return Err(ArrayNotFilled);
        }
        let array = unsafe { mem::transmute_copy::<[MaybeUninit<T>; S], [T; S]>(&mut self.array) };
        mem::forget(self);
        Ok(array)
    }

    pub(crate) fn finish(self) -> [T; S] {
        self.try_finish().unwrap()
    }
}
