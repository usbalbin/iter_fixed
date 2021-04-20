#![allow(incomplete_features)]
#![feature(const_generics)]

extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;

#[test]
fn test() {
    let res: [_; 2] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        .skip::<1>()
        .take::<2>()
        .collect();

    assert_eq!(res, [5, 5]);

    let foo: [(_, _); 3] = [1, 2, 3]
        .into_iter_fixed()
        .zip(core::iter::repeat(42))
        .collect();
    assert_eq!(foo, [(1, 42), (2, 42), (3, 42)]);
}

#[test]
fn test_changing_length() {
    let res: [_; 3] = [1, 2, 3, 4].into_iter_fixed().skip::<1>().collect();

    assert_eq!(res, [2, 3, 4]);

    let res: [_; 3] = [1, 2, 3, 4, 5].into_iter_fixed().step_by::<2>().collect();

    assert_eq!(res, [1, 3, 5]);

    let res: [_; 3] = [1, 2, 3, 4, 5, 6]
        .into_iter_fixed()
        .step_by::<2>()
        .collect();

    assert_eq!(res, [1, 3, 5]);

    let res: [_; 4] = [1, 2, 3, 4, 5, 6, 7]
        .into_iter_fixed()
        .step_by::<2>()
        .collect();

    assert_eq!(res, [1, 3, 5, 7]);

    let res: [_; 4] = [1, 2].into_iter_fixed().chain([3, 4]).collect();

    assert_eq!(res, [1, 2, 3, 4]);

    let res: [_; 2] = [1, 2, 3, 4].into_iter_fixed().take::<2>().collect();

    assert_eq!(res, [1, 2]);

    // Remove call to _.into_iter_fixed() once no longer needed
    let res: [_; 4] = [[1, 2].into_iter_fixed(), [3, 4].into_iter_fixed()]
        .into_iter_fixed()
        .flatten()
        .collect();

    assert_eq!(res, [1, 2, 3, 4]);
}
