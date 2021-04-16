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
