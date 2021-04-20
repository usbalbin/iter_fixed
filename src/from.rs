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
    /// Using IteratorFixed::collect() to implicitly use FromIteratorFixed:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    fn from_iter_fixed(iter_fixed: IteratorFixed<I, N>) -> Self;
}

impl<I: Iterator, const N: usize> FromIteratorFixed<I, N> for [<I as Iterator>::Item; N] {
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
    /// Using IteratorFixed::collect() to implicitly use FromIteratorFixed:
    /// ```
    /// use iter_fixed::IntoIteratorFixed;
    ///
    /// let two_four_six = [1, 2, 3].into_iter_fixed().map(|x| 2 * x);
    ///
    /// let a: [i32; 3] = two_four_six.collect();
    /// assert_eq!(a, [2, 4, 6]);
    /// ```
    fn from_iter_fixed(iter_fixed: IteratorFixed<I, N>) -> Self {
        let IteratorFixed { inner: mut it } = iter_fixed;
        // We know that it will yield N elements due to it originating from an IteratorFixed
        // of size N
        #[cfg(feature = "nightly_features")]
        {
            [(); N].map(|_| it.next().unwrap())
        }

        // Taken from collect_into_array in rust/library/core/src/array/mod.rs and manually
        // inlined calls to some not yet stablized functions / changed the code to use stable
        // alternatives.
        #[cfg(not(feature = "nightly_features"))]
        {
            use core::{mem, mem::MaybeUninit, ptr};

            fn collect_into_array<I, const N: usize>(iter: &mut I) -> Option<[I::Item; N]>
            where
                I: Iterator,
            {
                if N == 0 {
                    // SAFETY: An empty array is always inhabited and has no validity invariants.
                    return unsafe { Some(mem::zeroed()) };
                }

                struct Guard<T, const N: usize> {
                    ptr: *mut T,
                    initialized: usize,
                }

                impl<T, const N: usize> Drop for Guard<T, N> {
                    fn drop(&mut self) {
                        debug_assert!(self.initialized <= N);

                        let initialized_part =
                            ptr::slice_from_raw_parts_mut(self.ptr, self.initialized);

                        // SAFETY: this raw slice will contain only initialized objects.
                        unsafe {
                            ptr::drop_in_place(initialized_part);
                        }
                    }
                }
                // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
                let mut array =
                    unsafe { MaybeUninit::<[MaybeUninit<_>; N]>::uninit().assume_init() };
                let mut guard: Guard<_, N> = Guard {
                    ptr: array.as_mut_ptr() as *mut _,
                    initialized: 0,
                };

                while let Some(item) = iter.next() {
                    // SAFETY: `guard.initialized` starts at 0, is increased by one in the
                    // loop and the loop is aborted once it reaches N (which is
                    // `array.len()`).
                    array[guard.initialized] = MaybeUninit::new(item);
                    guard.initialized += 1;

                    // Check if the whole array was initialized.
                    if guard.initialized == N {
                        mem::forget(guard);

                        // SAFETY: the condition above asserts that all elements are
                        // initialized and there fore transmuting should be fine
                        let out: [<I as Iterator>::Item; N] =
                            unsafe { mem::transmute_copy(&array) };
                        return Some(out);
                    }
                }
                None
            }

            // We know that it contains N elements due to originating from IteratorFixed
            // so this should not panic
            collect_into_array(&mut it).unwrap()
        }
    }
}
