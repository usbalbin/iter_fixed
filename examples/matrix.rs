#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;

#[derive(Clone, Copy)]
struct Matrix<T, const R: usize, const K: usize> {
    rows: [[T; K]; R],
}

impl<T, const R: usize, const K: usize> Matrix<T, R, K> {
    fn get_col(&self, col_id: usize) -> [&T; R] {
        (&self.rows)
            .into_iter_fixed()
            .map(|row| &row[col_id])
            .collect()
    }

    fn get_row(&self, row_id: usize) -> [&T; K] {
        (&self.rows[row_id]).into_iter_fixed().collect()
    }
}

fn main() {
    let m = Matrix {
        rows: [[1, 2, 3], [4, 5, 6]],
    };

    assert_eq!(m.get_row(0), [&1, &2, &3]);
    assert_eq!(m.get_col(0), [&1, &4]);
}
