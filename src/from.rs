use crate::IteratorFixed;

/// Conversion from an [`IteratorFixed`].
///
/// By implementing `FromIteratorFixed` for a type, you define how it will be
/// created from an iterator of fixed size.
///
/// [`FromIteratorFixed::from_iter_fixed()`] is rarely called explicitly, and is instead
/// used through [`IteratorFixed::collect()`] method. See [`IteratorFixed::collect()`]'s
/// documentation for more examples.
///
/// See also: [`crate::IntoIteratorFixed`].
pub trait FromIteratorFixed<T, const N: usize> {
    /// Creates a value from a fixed size iterator.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::{IntoIteratorFixed, FromIteratorFixed};
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a = <[i32; 3]>::from_iter_fixed(two_four_six);
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    ///
    /// Using `IteratorFixed::collect()` to implicitly use `FromIteratorFixed`:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    fn from_iter_fixed<I: Iterator<Item = T>>(iter_fixed: IteratorFixed<I, N>) -> Self;
}

impl<T, const N: usize> FromIteratorFixed<T, N> for [T; N] {
    /// Creates an array from a fixed size iterator.
    ///
    /// Basic usage:
    /// ```
    /// use iter_fixed::{IntoIteratorFixed, FromIteratorFixed};
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a = <[i32; 3]>::from_iter_fixed(two_four_six);
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    ///
    /// Using `IteratorFixed::collect()` to implicitly use `FromIteratorFixed`:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    #[inline]
    fn from_iter_fixed<I: Iterator<Item = T>>(iter_fixed: IteratorFixed<I, N>) -> Self {
        let IteratorFixed { inner: mut it } = iter_fixed;
        // We know that it will yield N elements due to it originating from an IteratorFixed
        // of size N
        [(); N].map(|()| it.next().unwrap())
    }
}
