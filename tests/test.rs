extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;

fn main() {
    let res: [u32; 3] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        .skip::<1>()
        .take::<2>()
        .collect();

    println!("{:?}", res);
}
