use std::cmp::min;

use crate::assert_eqf64;
use crate::eqf64::eq_f64;
use crate::matrix::Matrix;

/// This file contains operations like:
/// Finding the determinant of various size matrices
/// Inverting matrices

impl Matrix {
    pub fn is_invertible(&self) -> bool {
        !eq_f64(0_f64, self.determinant())
    }

    pub fn inverse(&self) -> Matrix {
        assert!(self.is_invertible());

        let mut m = Matrix::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let el = self.cofactor(y, x) / self.determinant();
                m.set(x, y, el);
            }
        }

        m
    }

    /// If the matrix is a square, returns the length of an edge
    fn square_size(&self) -> Option<usize> {
        if self.width == self.height {
            Some(self.width)
        } else {
            None
        }
    }

    /// Recursively calculate determinant of any square matrix
    pub fn determinant(&self) -> f64 {
        match self.square_size() {
            Some(2) => self.determinant_2(),
            Some(_) => self.determinant_x(),
            None => panic!(
                "Determinant input must be a square (NxN) matrix, got:\n{}",
                self
            ),
        }
    }

    fn determinant_x(&self) -> f64 {
        let mut det = 0_f64;
        for col in 0..self.width {
            det += self.get(0, col) * self.cofactor(0, col);
        }
        det
    }

    // "base case" function to compute determinant of a 2x2 matrix
    fn determinant_2(&self) -> f64 {
        assert_eq!(self.width, 2, "only 2x2 matrices supported");
        assert_eq!(self.height, 2, "only 2x2 matrices supported");
        let (a, b, c, d) = (
            self.get(0, 0),
            self.get(0, 1),
            self.get(1, 0),
            self.get(1, 1),
        );
        a * d - b * c
    }
}

impl Matrix {
    /// A "minor" is just a determinant of a submatrix.
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    /// A cofactor is just a minor, with the possibility of negation based on where it lies in the matrix.
    /// To determine whether to negate or not, check row+column: odd? -> negate. even? don't.
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        let should_negate = (row + col) % 2 == 1; // negate if row+col is an odd number
        match should_negate {
            true => -1.0 * minor,
            false => minor,
        }
    }

    /// Given an NxN matrix, return an (N-1)x(N-1) matrix with row and col removed.
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        // create a smaller matrix, then map every index in the original to an index in the new one.
        // then copy each element to the new matrix individually
        let mut m = Matrix::new(self.width - 1, self.height - 1);

        let row_indices = (0..self.height)
            .filter(|i| i != &row) // skip the specified row
            .zip(0..m.height);

        for (y_old, y_new) in row_indices {
            // TODO optimization: generate this only a single time outside the loop
            let col_indices = (0..self.width)
                .filter(|i| i != &col) // skip the specified column
                .zip(0..m.width);

            for (x_old, x_new) in col_indices {
                let element = self.get(y_old, x_old);
                m.set(y_new, x_new, element);
            }
        }

        m
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn defaults() -> (Matrix, Matrix, Matrix) {
        let two = Matrix::from(vec![vec![1., 5.], vec![-3., 2.]]);
        let three = Matrix::from(vec![
            vec![3., 5., 0.],
            vec![2., -1., -7.],
            vec![6., -1., 5.],
        ]);
        let four = Matrix::from(vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);

        (two, three, four)
    }

    #[test]
    fn test_2x2_determinant() {
        let (m, _, _) = defaults();
        assert_eqf64!(m.determinant(), 17.)
    }

    #[test]
    fn test_submatrix_3x3() {
        let (_, m, _) = defaults();
        let exp = Matrix::from(vec![vec![2., -1.], vec![6., -1.]]);
        assert_eq!(m.submatrix(0, 2), exp);
    }

    #[test]
    fn test_submatrix_4x4() {
        let (_, _, m) = defaults();
        let exp = Matrix::from(vec![vec![2., 4., 2.], vec![8., 6., 1.], vec![0., 0., 1.]]);
        assert_eq!(m.submatrix(0, 2), exp);
    }

    #[test]
    fn test_minor_3x3() {
        let (_, m, _) = defaults();
        let b = m.submatrix(1, 0);
        assert_eqf64!(b.determinant(), 25.0);
        assert_eqf64!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactors() {
        let (_, a, _) = defaults();

        assert_eqf64!(a.minor(0, 0), -12.0);
        assert_eqf64!(a.cofactor(0, 0), -12.0);
        assert_eqf64!(a.minor(1, 0), 25.0);
        assert_eqf64!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3() {
        let a = Matrix::from(vec![vec![1., 2., 6.], vec![-5., 8., -4.], vec![2., 6., 4.]]);

        assert_eqf64!(a.cofactor(0, 0), 56.);
        assert_eqf64!(a.cofactor(0, 1), 12.);
        assert_eqf64!(a.cofactor(0, 2), -46.);
        assert_eqf64!(a.determinant(), -196.);
    }

    #[test]
    fn determinant_4x4() {
        let a = Matrix::from(vec![
            vec![-2., -8., 3., 5.],
            vec![-3., 1., 7., 3.],
            vec![1., 2., -9., 6.],
            vec![-6., 7., 7., -9.],
        ]);

        assert_eqf64!(a.cofactor(0, 0), 690.);
        assert_eqf64!(a.cofactor(0, 1), 447.);
        assert_eqf64!(a.cofactor(0, 2), 210.);
        assert_eqf64!(a.cofactor(0, 3), 51.);
        assert_eqf64!(a.determinant(), -4071.);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix::from_str(
            "
                |  6 |  4 |  4 |  4 |
                |  5 |  5 |  7 |  6 |
                |  4 | -9 |  3 | -7 |
                |  9 |  1 |  7 | -6 |",
        )
            .unwrap();

        assert_eqf64!(m.determinant(), -2120.0);
        assert!(m.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let m = Matrix::from_str(
            "
                | -4 |  2 | -2 | -3 |
                |  9 |  6 |  2 |  6 |
                |  0 | -5 |  1 | -5 |
                |  0 |  0 |  0 |  0 |
        ",
        )
            .unwrap();
        assert_eqf64!(m.determinant(), 0.);
        assert!(!m.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix::from_str(
            "
            | -5 |  2 |  6 | -8 |
            |  1 | -5 |  1 |  8 |
            |  7 |  7 | -6 | -7 |
            |  1 | -3 |  7 |  4 |",
        )
            .unwrap();
        let b = a.inverse();

        assert_eqf64!(a.determinant(), 532.0);
        assert_eqf64!(a.cofactor(2, 3), -160.0);
        assert_eqf64!(b.get(3, 2), -160_f64 / 532.0);
        assert_eqf64!(a.cofactor(3, 2), 105.0);
        assert_eqf64!(b.get(2, 3), 105_f64 / 532.0);

        // round to 5 figures so it matches the test matrix below
        let b = b.round_elements(5);

        let exp = Matrix::from_str(
            "\
            |  0.21805 |  0.45113 |  0.24060 | -0.04511 |
            | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
            | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
            | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
        ",
        )
            .unwrap();
        assert_eq!(b, exp);
    }

    #[test]
    fn more_inversion_test_cases() {
        fn test_inversion(a: &str, expected: &str) -> Matrix {
            let a: Matrix = Matrix::from_str(a).unwrap();
            let b: Matrix = a.inverse().round_elements(5);

            let expected: Matrix = Matrix::from_str(expected).unwrap();
            assert_eq!(b, expected);
            a.clone()
        }

        let a = "\
        |  8 | -5 |  9 |  2 |
        |  7 |  5 |  6 |  1 |
        | -6 |  0 |  9 |  6 |
        | -3 |  0 | -9 | -4 |";
        let a_inv = "\
        | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
        | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
        |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
        | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
        ";
        let a = test_inversion(a, a_inv);

        let b = "\
        |  9 |  3 |  0 |  9 |
        | -5 | -2 | -6 | -3 |
        | -4 |  9 |  6 |  4 |
        | -7 |  6 |  6 |  2 |";
        let b_inv = "\
        | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
        | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
        | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
        |  0.17778 |  0.06667 | -0.26667 |  0.33333 |";
        let b = test_inversion(b, b_inv);

        // Also test that if A*B=C, then C*inv(B)=A
        let c: Matrix = a.clone() * b.clone();
        let b_inv = b.inverse();
        let should_be_a = (c * (b_inv)).round_elements(5);

        assert_eq!(a, should_be_a);
    }
}
