use crate::IteratorFixed;

use core::{array, iter, slice};

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
    /// Creates a fixed size iterator from a value.
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
    fn into_iter_fixed(self) -> IteratorFixed<I, N>;
}

// IteratorFixed implements IntoIteratorFixed
unsafe impl<I: Iterator, const N: usize> IntoIteratorFixed<I, N> for IteratorFixed<I, N>
where
    IteratorFixed<I, N>: IntoIterator,
{
    /// `IteratorFixed` implements `IntoIteratorFixed` as a no op. This allows passing an
    /// `IteratorFixed` where an `IntoIteratorFixed` was expected
    ///
    /// Basic usage with zip which expects an IntoIteratorFixed as its argument:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    /// let one_one = [1, 1].into_iter_fixed();
    /// let zipped: [_; 2] = [1, 2].into_iter_fixed().zip(one_one).collect();
    ///
    /// assert_eq!(zipped, [(1, 1), (2, 1)]);
    /// ```
    fn into_iter_fixed(self) -> IteratorFixed<I, N> {
        self
    }
}

unsafe impl<T, const N: usize> IntoIteratorFixed<array::IntoIter<T, N>, N> for [T; N] {
    /// Creates a fixed size iterator from an array.
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
    fn into_iter_fixed(self) -> IteratorFixed<array::IntoIter<T, N>, N> {
        // Safety: array::IntoIter::new([T; N]) always yields N elements
        unsafe { IteratorFixed::from_iter(array::IntoIter::new(self)) }
    }
}

unsafe impl<'a, T, const N: usize> IntoIteratorFixed<slice::Iter<'a, T>, N> for &'a [T; N] {
    /// Creates a fixed size iterator from a borrowed array.
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
    fn into_iter_fixed(self) -> IteratorFixed<slice::Iter<'a, T>, N> {
        // Safety: [T; N]::iter always yields N elements
        unsafe { IteratorFixed::from_iter(self.iter()) }
    }
}

unsafe impl<T: Clone, const N: usize> IntoIteratorFixed<iter::Take<iter::Repeat<T>>, N>
    for iter::Repeat<T>
{
    /// Creates a fixed size iterator from an [`core::iter::Repeat`]
    ///
    /// Basic usage:
    /// ```
    /// use core::iter;
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let one_one_one = iter::repeat(1).into_iter_fixed();
    ///
    /// let a: [i32; 3] = one_one_one.collect();
    /// assert_eq!(a, [1, 1, 1]);
    /// ```
    fn into_iter_fixed(self) -> IteratorFixed<iter::Take<iter::Repeat<T>>, N> {
        // Safety: iter::repeat(_).take(N) always yields N elements
        unsafe { IteratorFixed::from_iter(self.take(N)) }
    }
}
