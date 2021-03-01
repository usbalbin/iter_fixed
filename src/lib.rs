#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(array_map)]

use std::iter;

mod from;
mod helpers;
mod into;

pub use from::FromIteratorFixed;
use helpers::{min, sub_or_zero};
pub use into::IntoIteratorFixed;

/// Iterator of fixed size
///
/// A type that can be usen a bit like an ordinary Iterator but with a compile time guaranteed
/// length. This enables us to turn them back into collections of fixed size without having to
/// perform unnecessary checks during run time.
///
/// Just like Iterator, IteratorFixed provides a lot of methods like:
///
/// * `map`
/// * `inspect`
/// * `skip`
/// * `step_by`
/// * `chain`
/// * `enumerate`
/// * `take`
/// * `zip`
/// * `rev`
/// * `copied`
/// * `cloned`
///
/// however it does not support methods like `filter` or `take_while` which will affect the length during runtime.
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
    pub unsafe fn from_iter<II: IntoIterator<IntoIter = I>>(i: II) -> Self {
        IteratorFixed {
            inner: i.into_iter(),
        }
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

    // TODO: what should happen when SKIP > N?
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
        I2: Iterator<Item = <I as IntoIterator>::Item>,
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

    pub fn zip<U, IIF, I2>(self, other: IIF) -> IteratorFixed<iter::Zip<I, I2>, N>
    where
        IIF: IntoIteratorFixed<I2, N>,
        I2: Iterator<Item = U>,
    {
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
    // TODO: Would it be better to have `I: Iterator<Item = IntoIteratorFixed`?
    pub fn flatten(self) -> IteratorFixed<iter::Flatten<I>, { M * N }> {
        IteratorFixed {
            inner: self.inner.flatten(),
        }
    }

    /*
    pub fn flat_map(self, f: F) -> FlatMap<Self, U, F>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        unimplemented!()
    }
    */
}

impl<T, I: Iterator<Item = T>, const N: usize> IntoIterator for IteratorFixed<I, N> {
    type Item = T;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self.inner
    }
}
