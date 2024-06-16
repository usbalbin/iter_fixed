#![no_std]
#![allow(stable_features)]
#![cfg_attr(feature = "nightly_features", allow(incomplete_features))]
#![cfg_attr(feature = "nightly_features", feature(generic_const_exprs))]
// enable additionnal lints
#![warn(clippy::doc_markdown)]
#![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::use_self)]

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
/// Just like Iterator, `IteratorFixed` provides a lot of methods like:
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

/// Creates a new iterator of fixed size where each iteration calls the provided closure F: FnMut(usize) -> T
///
/// This allows very simple initialization of types that implement [`FromIteratorFixed`] such as arrays.
///
/// Note: This function is quite similar to [`iter::from_fn`] however note that in contrast to [`iter::from_fn`],
/// in `IteratorFixed::from_fn` the provided function does not have any say in the number of elements.
/// The length is entirely determined by `N`.
///
/// Basic usage:
/// ```
/// let zero_two_four: [usize; 3] = iter_fixed::from_fn(|i| 2 * i).collect();
///
/// assert_eq!(zero_two_four, [0, 2, 4]);
/// ```
#[inline]
pub fn from_fn<'a, F, T: 'a, const N: usize>(
    mut f: F,
) -> IteratorFixed<impl Iterator<Item = T> + 'a, N>
where
    F: FnMut(usize) -> T + 'a,
{
    [(); N]
        .into_iter_fixed()
        .enumerate()
        .map(move |(i, ())| f(i))
}

impl<I, const N: usize> IteratorFixed<I, N>
where
    I: Iterator,
{
    /// # Safety
    /// Caller has to guarantee that the given iterator will yield exactly N elements
    ///
    // TODO: Would it be ok if it generated more elements?
    #[inline]
    pub unsafe fn from_iter<II: IntoIterator<IntoIter = I>>(i: II) -> Self {
        Self {
            inner: i.into_iter(),
        }
    }

    /// See [`core::iter::Iterator::map`]
    #[inline]
    pub fn map<U, F: FnMut(<I as Iterator>::Item) -> U>(
        self,
        p: F,
    ) -> IteratorFixed<impl Iterator<Item = U>, N> {
        IteratorFixed {
            inner: self.inner.map(p),
        }
    }

    /// See [`core::iter::Iterator::inspect`]
    #[inline]
    pub fn inspect<F: FnMut(&<I as Iterator>::Item)>(
        self,
        p: F,
    ) -> IteratorFixed<impl Iterator<Item = I::Item>, N> {
        IteratorFixed {
            inner: self.inner.inspect(p),
        }
    }

    // TODO: what should happen when SKIP > N?
    /// See [`core::iter::Iterator::skip`]
    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn skip<const SKIP: usize>(
        self,
    ) -> IteratorFixed<impl Iterator<Item = I::Item>, { sub_or_zero(N, SKIP) }> {
        IteratorFixed {
            inner: self.inner.skip(SKIP),
        }
    }

    /// See [`core::iter::Iterator::step_by`]
    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn step_by<const STEP: usize>(
        self,
    ) -> IteratorFixed<impl Iterator<Item = I::Item>, { ceiling_div(N, STEP) }> {
        IteratorFixed {
            inner: self.inner.step_by(STEP),
        }
    }

    /// See [`core::iter::Iterator::chain`]
    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn chain<IIF, const M: usize>(
        self,
        other: IIF,
    ) -> IteratorFixed<impl Iterator<Item = I::Item>, { N + M }>
    where
        IIF: IntoIteratorFixed<M, Item = I::Item>,
    {
        IteratorFixed {
            inner: self.inner.chain(other.into_iter_fixed().inner),
        }
    }

    /// See [`core::iter::Iterator::enumerate`]
    #[inline]
    pub fn enumerate(self) -> IteratorFixed<impl Iterator<Item = (usize, I::Item)>, N> {
        IteratorFixed {
            inner: self.inner.enumerate(),
        }
    }

    /// See [`core::iter::Iterator::take`]
    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn take<const TAKE: usize>(
        self,
    ) -> IteratorFixed<impl Iterator<Item = I::Item>, { min(TAKE, N) }> {
        IteratorFixed {
            inner: self.inner.take(TAKE),
        }
    }

    /// See [`core::iter::Iterator::zip`]
    #[inline]
    pub fn zip<IIF>(
        self,
        other: IIF,
    ) -> IteratorFixed<impl Iterator<Item = (I::Item, IIF::Item)>, N>
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
    #[inline]
    pub fn rev(self) -> IteratorFixed<impl Iterator<Item = I::Item>, N>
    where
        I: iter::DoubleEndedIterator,
    {
        IteratorFixed {
            inner: self.inner.rev(),
        }
    }

    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn flatten<IIF, const M: usize>(
        self,
    ) -> IteratorFixed<impl Iterator<Item = IIF::Item>, { M * N }>
    where
        I: Iterator<Item = IIF>,
        IIF: IntoIteratorFixed<M>,
    {
        // The call to into_iter_fixed is needed because we cannot trust that
        // let x: I::Item;
        // x.into_iterator() == x.into_iter_fixed().into_iterator()
        IteratorFixed {
            inner: self.inner.flat_map(IntoIteratorFixed::into_iter_fixed),
        }
    }

    #[cfg(feature = "nightly_features")]
    #[inline]
    pub fn flat_map<F, IIF, const M: usize>(
        self,
        mut f: F,
    ) -> IteratorFixed<impl Iterator<Item = IIF::Item>, { M * N }>
    where
        F: FnMut(I::Item) -> IIF,
        IIF: IntoIteratorFixed<M>,
    {
        // The call to into_iter_fixed is needed because we cannot trust that
        // let x: I::Item;
        // x.into_iterator() == x.into_iter_fixed().into_iterator()
        IteratorFixed {
            inner: self.inner.flat_map(move |x| f(x).into_iter_fixed()),
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
    #[inline]
    pub fn collect<U: FromIteratorFixed<I::Item, N>>(self) -> U {
        U::from_iter_fixed(self)
    }
}

impl<'a, I, T: 'a, const N: usize> IteratorFixed<I, N>
where
    I: Iterator<Item = &'a T>,
{
    /// See [`core::iter::Iterator::copied`]
    #[inline]
    pub fn copied(self) -> IteratorFixed<impl Iterator<Item = T>, N>
    where
        T: Copy,
    {
        IteratorFixed {
            inner: self.inner.copied(),
        }
    }

    /// See [`core::iter::Iterator::cloned`]
    #[inline]
    pub fn cloned(self) -> IteratorFixed<impl Iterator<Item = T>, N>
    where
        T: Clone,
    {
        IteratorFixed {
            inner: self.inner.cloned(),
        }
    }
}

/// Convert the fixed size iterator into an ordinary [`core::iter::Iterator`]
/// allowing it to be used with for loop syntax
impl<T, I: Iterator<Item = T>, const N: usize> IntoIterator for IteratorFixed<I, N> {
    type Item = T;
    type IntoIter = I;

    /// Convert the fixed size iterator into an ordinary [`core::iter::Iterator`]
    #[inline]
    fn into_iter(self) -> I {
        self.inner
    }
}
