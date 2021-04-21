#![no_std]
#![cfg_attr(feature = "nightly_features", allow(incomplete_features))]
#![cfg_attr(
    feature = "nightly_features",
    feature(array_map, const_generics, const_evaluatable_checked)
)]

use core::iter;

mod from;
mod helpers;
mod into;

#[cfg(feature = "nightly_features")]
use helpers::{ceiling_div, min, sub_or_zero};

pub use from::FromIteratorFixed;
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
    // TODO: Would it be ok if it generated more elements?
    pub unsafe fn from_iter<II: IntoIterator<IntoIter = I>>(i: II) -> Self {
        IteratorFixed {
            inner: i.into_iter(),
        }
    }

    /// See [`core::iter::Iterator::map`]
    pub fn map<U, F: FnMut(<I as Iterator>::Item) -> U>(
        self,
        p: F,
    ) -> IteratorFixed<iter::Map<I, F>, N> {
        IteratorFixed {
            inner: self.inner.map(p),
        }
    }

    /// See [`core::iter::Iterator::inspect`]
    pub fn inspect<F: FnMut(&<I as Iterator>::Item)>(
        self,
        p: F,
    ) -> IteratorFixed<iter::Inspect<I, F>, N> {
        IteratorFixed {
            inner: self.inner.inspect(p),
        }
    }

    // TODO: what should happen when SKIP > N?
    /// See [`core::iter::Iterator::skip`]
    #[cfg(feature = "nightly_features")]
    pub fn skip<const SKIP: usize>(self) -> IteratorFixed<iter::Skip<I>, { sub_or_zero(N, SKIP) }> {
        IteratorFixed {
            inner: self.inner.skip(SKIP),
        }
    }

    /// See [`core::iter::Iterator::step_by`]
    #[cfg(feature = "nightly_features")]
    pub fn step_by<const STEP: usize>(
        self,
    ) -> IteratorFixed<iter::StepBy<I>, { ceiling_div(N, STEP) }> {
        IteratorFixed {
            inner: self.inner.step_by(STEP),
        }
    }

    /// See [`core::iter::Iterator::chain`]
    #[cfg(feature = "nightly_features")]
    pub fn chain<IIF, const M: usize>(
        self,
        other: IIF,
    ) -> IteratorFixed<iter::Chain<I, IIF::IntoIter>, { N + M }>
    where
        IIF: IntoIteratorFixed<M, Item = I::Item>,
    {
        IteratorFixed {
            inner: self.inner.chain(other.into_iter_fixed().inner),
        }
    }

    /// See [`core::iter::Iterator::enumerate`]
    pub fn enumerate(self) -> IteratorFixed<iter::Enumerate<I>, N> {
        IteratorFixed {
            inner: self.inner.enumerate(),
        }
    }

    /// See [`core::iter::Iterator::take`]
    #[cfg(feature = "nightly_features")]
    pub fn take<const TAKE: usize>(self) -> IteratorFixed<iter::Take<I>, { min(TAKE, N) }> {
        IteratorFixed {
            inner: self.inner.take(TAKE),
        }
    }

    /// See [`core::iter::Iterator::zip`]
    pub fn zip<IIF>(self, other: IIF) -> IteratorFixed<iter::Zip<I, IIF::IntoIter>, N>
    where
        IIF: IntoIteratorFixed<N>,
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

    /// See [`core::iter::Iterator::rev`]
    pub fn rev(self) -> IteratorFixed<iter::Rev<I>, N>
    where
        I: iter::DoubleEndedIterator,
    {
        IteratorFixed {
            inner: self.inner.rev(),
        }
    }

    /// Transforms a fixed size iterator into a collection of compile time known size.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    pub fn collect<U: FromIteratorFixed<I, N>>(self) -> U {
        U::from_iter_fixed(self)
    }
}

impl<'a, I, T: 'a, const N: usize> IteratorFixed<I, N>
where
    I: Iterator<Item = &'a T>,
{
    /// See [`core::iter::Iterator::copied`]
    pub fn copied(self) -> IteratorFixed<iter::Copied<I>, N>
    where
        T: Copy,
    {
        IteratorFixed {
            inner: self.inner.copied(),
        }
    }

    /// See [`core::iter::Iterator::cloned`]
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
    /// See [`core::iter::Iterator::flatten`]
    #[cfg(feature = "nightly_features")]
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

/// Convert the fixed size iterator into an ordinary [`core::iter::Iterator`]
/// allowing it to be used with for loop syntax
impl<T, I: Iterator<Item = T>, const N: usize> IntoIterator for IteratorFixed<I, N> {
    type Item = T;
    type IntoIter = I;

    /// Convert the fixed size iterator into an ordinary [`core::iter::Iterator`]
    fn into_iter(self) -> I {
        self.inner
    }
}
