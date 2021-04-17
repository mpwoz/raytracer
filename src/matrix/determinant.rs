use crate::assert_eqf64;
use crate::matrix::Matrix;

/// This file contains operations like:
/// Finding the determinant of various size matrices
/// Inverting matrices
impl Matrix {
    pub fn determinant(&self) -> f64 {
        // 2x2 case
        let (a, b, c, d) = (
            self.get(0, 0),
            self.get(0, 1),
            self.get(1, 0),
            self.get(1, 1),
        );
        a * d - b * c
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
            vec![1., 2., 3.],
            vec![0.1, 0.2, 0.3],
            vec![-3., -6., -9.],
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
        let exp = Matrix::from(vec![vec![0.1, 0.2], vec![-3., -6.]]);
        assert_eq!(m.submatrix(0, 2), exp);
    }

    #[test]
    fn test_submatrix_4x4() {
        let (_, _, m) = defaults();
        let exp = Matrix::from(vec![
            vec![2., 4., 2.],
            vec![8., 6., 1.],
            vec![0., 0., 1.],
        ]);
        assert_eq!(m.submatrix(0, 2), exp);
    }
}
