#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(array_map)]

use core::array;
use std::iter;

pub fn foo() {
    // Same as the last one that fails to compile in tests/test.rs
    // however this compiles fine:
    let _res: [_; 2] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        .skip::<1>()
        .take::<2>()
        .collect();
}

pub unsafe trait IntoIteratorFixed<I: Iterator, const N: usize> {
    fn into_iter_fixed(self) -> IteratorFixed<I, N>;
}

// Safety: array::IntoIter::new([T; N]) always yields N elements
unsafe impl<T, const N: usize> IntoIteratorFixed<array::IntoIter<T, N>, N> for [T; N] {
    fn into_iter_fixed(self) -> IteratorFixed<array::IntoIter<T, N>, N> {
        unsafe { IteratorFixed::from_iter(array::IntoIter::new(self)) }
    }
}

pub struct IteratorFixed<I: Iterator, const N: usize> {
    inner: I,
}

impl<I, const N: usize> IteratorFixed<I, N>
where
    I: Iterator,
{
    /// # Safety
    /// Caller has to guarantee that the given iterator will yield exactly N elements
    ///
    /// TODO: Would it be ok if it generated more elements?
    pub unsafe fn from_iter(i: I) -> Self {
        IteratorFixed { inner: i }
    }

    pub fn map<U, F: FnMut(<I as Iterator>::Item) -> U>(
        self,
        p: F,
    ) -> IteratorFixed<iter::Map<I, F>, N> {
        IteratorFixed {
            inner: self.inner.map(p),
        }
    }

    pub fn skip<const SKIP: usize>(self) -> IteratorFixed<iter::Skip<I>, { sub_or_zero(N, SKIP) }> {
        IteratorFixed {
            inner: self.inner.skip(SKIP),
        }
    }

    pub fn take<const TAKE: usize>(self) -> IteratorFixed<iter::Take<I>, { min(TAKE, N) }> {
        IteratorFixed {
            inner: self.inner.take(TAKE),
        }
    }

    pub fn zip<U, IIF: IntoIteratorFixed<I2, N>, I2: Iterator<Item = U>>(
        self,
        other: IIF,
    ) -> IteratorFixed<iter::Zip<I, I2>, N> {
        IteratorFixed {
            inner: self.inner.zip(other.into_iter_fixed().inner),
        }
    }

    pub fn collect<U: FromIteratorFixed<I, N>>(self) -> U {
        U::from_iter_fixed(self)
    }
}

pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub const fn sub_or_zero(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

pub trait FromIteratorFixed<I: Iterator, const N: usize> {
    fn from_iter_fixed(iter_fixed: IteratorFixed<I, N>) -> Self;
}

impl<I: Iterator, const N: usize> FromIteratorFixed<I, N> for [<I as Iterator>::Item; N] {
    fn from_iter_fixed(iter_fixed: IteratorFixed<I, N>) -> Self {
        let IteratorFixed { inner: mut it } = iter_fixed;
        // We know that it will yield N elements due to it originating from an IteratorFixed
        // of size N
        [(); N].map(|_| it.next().unwrap())
    }
}
