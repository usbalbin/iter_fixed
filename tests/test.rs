extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;

fn main() {
    let _res: [_; 4] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .map(|a| a + 1)
        //.skip::<1>()
        //.take::<2>()
        .collect();

    let _res: [_; 4] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        //.skip::<1>()
        //.take::<2>()
        .collect();

    let _res: [_; 2] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        //.map(|(a, b)| a + b)
        .skip::<1>()
        .take::<2>()
        .collect();

    let _res: [_; 2] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        //.map(|a| a + 1)
        .skip::<1>()
        .take::<2>()
        .collect();

    // This one gives error:
    // error[E0277]: the trait bound `[_; 4]: FromIteratorFixed<std::iter::Take<Skip<Map<Zip<std::array::IntoIter<u32, 4_usize>, std::array::IntoIter<u32, 4_usize>>, [closure@tests/test.rs:39:14:39:28]>>>, {_: usize}>` is not satisfied
    let res: [_; 2] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        .skip::<1>()
        .take::<2>()
        .collect();
}
