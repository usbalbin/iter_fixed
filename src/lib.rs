#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(array_map)]

use core::{array, slice};
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

// Safety: [T; N]::iter always yields N elements
unsafe impl<'a, T, const N: usize> IntoIteratorFixed<slice::Iter<'a, T>, N> for &'a [T; N] {
    fn into_iter_fixed(self) -> IteratorFixed<slice::Iter<'a, T>, N> {
        unsafe { IteratorFixed::from_iter(self.iter()) }
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

    pub fn inspect<F: FnMut(&<I as Iterator>::Item)>(
        self,
        p: F,
    ) -> IteratorFixed<iter::Inspect<I, F>, N> {
        IteratorFixed {
            inner: self.inner.inspect(p),
        }
    }

    pub fn skip<const SKIP: usize>(self) -> IteratorFixed<iter::Skip<I>, { sub_or_zero(N, SKIP) }> {
        IteratorFixed {
            inner: self.inner.skip(SKIP),
        }
    }

    pub fn step_by<const STEP: usize>(self) -> IteratorFixed<iter::StepBy<I>, { N / STEP }> {
        IteratorFixed {
            inner: self.inner.step_by(STEP),
        }
    }

    pub fn chain<IIF, I2, const M: usize>(self, other: IIF) -> IteratorFixed<iter::Chain<I, I2>, N>
    where
        IIF: IntoIteratorFixed<I2, M>,
        I2: Iterator<Item = <I as Iterator>::Item>,
    {
        IteratorFixed {
            inner: self.inner.chain(other.into_iter_fixed().inner),
        }
    }

    pub fn enumerate(self) -> IteratorFixed<iter::Enumerate<I>, N> {
        IteratorFixed {
            inner: self.inner.enumerate(),
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

    /*
    pub fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
    where
        I: Iterator<Item = (A, B)>,
        FromA: FromIteratorFixed<A, N>,
        FromB: FromIteratorFixed<B, N>,
    {
        unimplemented!()
    }
    */

    pub fn rev(self) -> IteratorFixed<iter::Rev<I>, N>
    where
        I: iter::DoubleEndedIterator,
    {
        IteratorFixed {
            inner: self.inner.rev(),
        }
    }

    pub fn collect<U: FromIteratorFixed<I, N>>(self) -> U {
        U::from_iter_fixed(self)
    }
}

impl<'a, I, T: 'a, const N: usize> IteratorFixed<I, N>
where
    I: Iterator<Item = &'a T>,
{
    pub fn copied(self) -> IteratorFixed<iter::Copied<I>, N>
    where
        T: Copy,
    {
        IteratorFixed {
            inner: self.inner.copied(),
        }
    }

    pub fn cloned(self) -> IteratorFixed<iter::Cloned<I>, N>
    where
        T: Clone,
    {
        IteratorFixed {
            inner: self.inner.cloned(),
        }
    }
}

impl<I, I2, const N: usize, const M: usize> IteratorFixed<I, N>
where
    I: Iterator<Item = IteratorFixed<I2, M>>,
    I2: Iterator,
{
    pub fn flatten(self) -> IteratorFixed<iter::Flatten<I>, { M * N }> {
        IteratorFixed {
            inner: self.inner.flatten(),
        }
    }

    /*
     pub fn flat_map(self, f: F) -> FlatMap<Self, U, F>
     where
         F: FnMut(Self::Item) -> U,
    pub      U: IntoIterator,
     {
         unimplemented!()
     }
     */
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
        let mut it = iter_fixed.into_iter();
        // We know that it will yield N elements due to it originating from an IteratorFixed
        // of size N
        [(); N].map(|_| it.next().unwrap())
    }
}

impl<T, I: Iterator<Item = T>, const N: usize> IntoIterator for IteratorFixed<I, N> {
    type Item = T;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self.inner
    }
}
