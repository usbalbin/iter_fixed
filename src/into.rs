use crate::IteratorFixed;

use core::{array, slice};

/// Conversion into an [`IteratorFixed`].
///
/// By implementing `IntoIteratorFixed` for a type, you define how it will be
/// converted to an iterator of fixed size.
///
/// See also: [`crate::FromIteratorFixed`].
///
/// # Safety
/// Implementer has to guarantee that the inner iterator will always yield exactly N elements
pub unsafe trait IntoIteratorFixed<I: Iterator, const N: usize> {
    fn into_iter_fixed(self) -> IteratorFixed<I, N>;
}

// IteratorFixed implements IntoIteratorFixed
unsafe impl<I: Iterator, const N: usize> IntoIteratorFixed<I, N> for IteratorFixed<I, N>
where
    IteratorFixed<I, N>: IntoIterator,
{
    fn into_iter_fixed(self) -> IteratorFixed<I, N> {
        self
    }
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
