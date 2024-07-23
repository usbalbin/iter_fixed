//! Provides a type and traits for turning collections of fixed size, like arrays,
//! into [`IteratorFixed`] which can be used a bit like an ordinary [`Iterator`] but
//! with a compile time guaranteed length. This enables us to turn them back into
//! collections of fixed size without having to perform unnecessary checks during
//! run time.
//!
//! [`IteratorFixed`] provides on stable methods like `map`, `inspect`, `enumerate`,
//!  `zip`, `rev`, `copied`, `cloned`, with nightly `skip`, `step_by`, `chain`, `take`,
//!  `flatten`.
//!
//! However it does not and will never be able to support methods like
//!  `filter` or `take_while` which will affect the length during runtime.
//!
//! # ⚠️ Experimental
//! *This code is currently very experimental, type names, function names, trait bounds etc. are all very much subject to change.*
//!
//! # Origin
//! *This project is inspired by @leonardo-m 's idea <https://github.com/rust-lang/rust/issues/80094#issuecomment-749260428>*
//!
//! # Examples:
//! ```
//! # use crate::iter_fixed::IntoIteratorFixed;
//! // simply reverse an Array
//! let rev_array: [_; 4] = [1, 3, 2, 7]
//!     .into_iter_fixed()
//!     .rev()
//!     .collect();
//! assert_eq!(rev_array, [7, 2, 3, 1]);
//!
//! // .. and compute sum with values of a second array multiplied by 10
//! let sum_array: [_; 4] = rev_array
//!     .into_iter_fixed()
//!     .zip([4,1,3,7])
//!     .map(|(a, b)| a + (b * 10))
//!     .collect();
//! assert_eq!(sum_array, [47, 12, 33, 71]);
//! ```
//!
//! You can also take a look at examples : [`matrix.rs`] and [`vector.rs`]
//!
//! [`matrix.rs`]: source/examples/matrix.rs
//! [`vector.rs`]: source/examples/vector.rs
//!
#![no_std]
#![allow(stable_features)]
#![cfg_attr(feature = "nightly_features", allow(incomplete_features))]
#![cfg_attr(feature = "nightly_features", feature(generic_const_exprs))]
// enable additionnal lints
#![warn(clippy::doc_markdown)]
#![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::use_self)]

use core::{
    iter,
    ops::{Add, Div, Mul, Not, Rem, Sub},
};

mod from;
mod helpers;
mod into;

use helpers::*;

pub use from::FromIteratorFixed;
pub use into::IntoIteratorFixed;
use typenum::{Const, Eq, IsEqual, Min, Mod, ToUInt, U, U0};

type FlattenIter<I, IIF, const M: usize> = iter::FlatMap<
    I,
    IteratorFixed<<IIF as IntoIteratorFixed<M>>::IntoIter, M>,
    fn(IIF) -> IteratorFixed<<IIF as IntoIteratorFixed<M>>::IntoIter, M>,
>;

/// Iterator of fixed size
///
/// A type that can be usen a bit like an ordinary Iterator but with a compile time guaranteed
/// length. This enables us to turn them back into collections of fixed size without having to
/// perform unnecessary checks during run time.
///
/// Just like [`Iterator`], [`IteratorFixed`] provides a lot of methods like:
/// - available on stable rust:  
///   [`map`], [`inspect`], [`enumerate`], [`zip`], [`rev`], [`copied`], [`cloned`], [`skip`], [`step_by`], [`chain`], [`take`], [`flatten`]
///     
/// - requires nightly compiler and enable `nightly_features`:  
///   [`flat_map`]
///
/// however it does not support methods like `filter` or `take_while` which will affect the length during runtime.
///
/// [`map`]: IteratorFixed::map
/// [`inspect`]: IteratorFixed::inspect
/// [`enumerate`]: IteratorFixed::enumerate
/// [`zip`]: IteratorFixed::zip
/// [`rev`]: IteratorFixed::rev
/// [`copied`]: IteratorFixed::copied
/// [`cloned`]: IteratorFixed::cloned
/// [`skip`]: IteratorFixed::skip
/// [`step_by`]: IteratorFixed::step_by
/// [`chain`]: IteratorFixed::chain
/// [`take`]: IteratorFixed::take
/// [`flatten`]: IteratorFixed::flatten
/// [`flat_map`]: IteratorFixed::flat_map
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
    #[inline]
    pub fn skip<const SKIP: usize>(self) -> RunTypeNumToFixedIter<iter::Skip<I>, RunSub<N, SKIP>>
    where
        Const<N>: ToUInt,
        Const<SKIP>: ToUInt,

        U<N>: Sub<U<SKIP>, Output: TypenumToFixedIter<iter::Skip<I>>>,
    {
        unsafe { ErasedIterFixed::new(self.inner.skip(SKIP)) }
    }

    /// See [`core::iter::Iterator::step_by`]
    #[inline]
    pub fn step_by<const STEP: usize>(
        self,
    ) -> RunTypeNumToFixedIter<iter::StepBy<I>, RunCelDiv<N, STEP>>
    where
        Const<N>: ToUInt,
        Const<STEP>: ToUInt,

        U<N>: Rem<U<STEP>, Output: IsEqual<U0, Output: Not>>,
        U<N>: Div<
            U<STEP>,
            Output: Add<
                TyNot<Eq<Mod<U<N>, U<STEP>>, U0>>,
                Output: TypenumToFixedIter<iter::StepBy<I>>,
            >,
        >,
    {
        unsafe { ErasedIterFixed::new(self.inner.step_by(STEP)) }
    }

    /// See [`core::iter::Iterator::chain`]
    #[inline]
    pub fn chain<IIF, const M: usize>(
        self,
        other: IIF,
    ) -> RunTypeNumToFixedIter<iter::Chain<I, IIF::IntoIter>, RunAdd<N, M>>
    where
        Const<N>: ToUInt,
        Const<M>: ToUInt,

        IIF: IntoIteratorFixed<M, Item = I::Item>,
        U<N>: Add<U<M>, Output: TypenumToFixedIter<iter::Chain<I, IIF::IntoIter>>>,
    {
        unsafe { ErasedIterFixed::new(self.inner.chain(other.into_iter_fixed().inner)) }
    }

    /// See [`core::iter::Iterator::enumerate`]
    #[inline]
    pub fn enumerate(self) -> IteratorFixed<impl Iterator<Item = (usize, I::Item)>, N> {
        IteratorFixed {
            inner: self.inner.enumerate(),
        }
    }

    /// See [`core::iter::Iterator::take`]
    #[inline]
    pub fn take<const TAKE: usize>(self) -> RunTypeNumToFixedIter<iter::Take<I>, RunMin<N, TAKE>>
    where
        Const<N>: ToUInt,
        Const<TAKE>: ToUInt,

        U<N>: Min<U<TAKE>, Output: TypenumToFixedIter<iter::Take<I>>>,
    {
        unsafe { ErasedIterFixed::new(self.inner.take(TAKE)) }
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

    #[inline]
    pub fn flatten<IIF, const M: usize>(
        self,
    ) -> RunTypeNumToFixedIter<FlattenIter<I, IIF, M>, RunMul<N, M>>
    where
        Const<N>: ToUInt,
        Const<M>: ToUInt,
        U<N>: Mul<U<M>, Output: TypenumToFixedIter<FlattenIter<I, IIF, M>>>,

        I: Iterator<Item = IIF>,
        IIF: IntoIteratorFixed<M>,
    {
        // The call to into_iter_fixed is needed because we cannot trust that
        // let x: I::Item;
        // x.into_iterator() == x.into_iter_fixed().into_iterator()
        unsafe { ErasedIterFixed::new(self.inner.flat_map(IIF::into_iter_fixed as fn(_) -> _)) }
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
