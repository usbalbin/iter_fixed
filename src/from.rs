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
