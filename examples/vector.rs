extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
struct Vector<T, const N: usize> {
    elements: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Mul<T, Output = T> + std::iter::Sum,
{
    fn new(elements: [T; N]) -> Self {
        Vector { elements }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Mul<T, Output = T> + std::iter::Sum + Clone,
{
    fn length2(self) -> T {
        self.elements
            .into_iter_fixed()
            .into_iter()
            .map(|x| x.clone() * x)
            .sum()
    }
}

impl<A, B, const N: usize> Add<Vector<B, N>> for Vector<A, N>
where
    A: Add<B>,
{
    type Output = Vector<<A as Add<B>>::Output, N>;

    fn add(self, other: Vector<B, N>) -> Self::Output {
        Vector {
            elements: self
                .elements
                .into_iter_fixed()
                .zip(other.elements)
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

fn main() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([1, 1, 1]);

    assert_eq!((a + b).elements, [2, 3, 4]);
    assert_eq!(b.length2(), 3);
}
