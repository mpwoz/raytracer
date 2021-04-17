use std::cmp::min;

use crate::assert_eqf64;
use crate::matrix::Matrix;

/// This file contains operations like:
/// Finding the determinant of various size matrices
/// Inverting matrices
impl Matrix {
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
            None => panic!("Determinant input must be a square (NxN) matrix, got:\n{}", self)
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
}
