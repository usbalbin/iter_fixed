use typenum::{operator_aliases as ty_ops, U};

use crate::IteratorFixed;

/// A trait implemented for all `IterFixed<N>` to allow constructing `IterFixed` when `N` is unnamable.
pub trait ErasedIterFixed<I> {
    /// # Safety
    ///
    /// The N value of the [`IterFixed`] created must be the same as the number of items in the Iterator.
    unsafe fn new(iter: I) -> Self;
}

impl<I: Iterator, const N: usize> ErasedIterFixed<I> for IteratorFixed<I, N> {
    unsafe fn new(inner: I) -> Self {
        Self { inner }
    }
}

pub trait TypenumToFixedIter<I: Iterator> {
    type FixedIter: ErasedIterFixed<I>;
}

typenum_mappings::impl_typenum_mapping!(
    impl<const N: usize = 0..=1024, I: Iterator> TypenumToFixedIter<I> for #TypeNumName {
        type FixedIter = IteratorFixed<I, N>;
    }
);

pub(crate) type RunTypeNumToFixedIter<I, T> = <T as TypenumToFixedIter<I>>::FixedIter;

pub(crate) type TyNot<T> = <T as core::ops::Not>::Output;
type TyCelDiv<X, D> = ty_ops::Sum<
    // X / D
    ty_ops::Quot<X, D>,
    // + !
    TyNot<
        ty_ops::Eq<
            // X % 0
            ty_ops::Mod<X, D>,
            // == 0
            typenum::U0,
        >,
    >,
>;

pub(crate) type RunCelDiv<const X: usize, const D: usize> = TyCelDiv<U<X>, U<D>>;
pub(crate) type RunAdd<const X: usize, const Y: usize> = ty_ops::Sum<U<X>, U<Y>>;
pub(crate) type RunSub<const X: usize, const Y: usize> = ty_ops::Diff<U<X>, U<Y>>;
pub(crate) type RunMul<const X: usize, const Y: usize> = ty_ops::Prod<U<X>, U<Y>>;
pub(crate) type RunMin<const X: usize, const Y: usize> = ty_ops::Minimum<U<X>, U<Y>>;
