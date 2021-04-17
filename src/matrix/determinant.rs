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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x2_determinant() {
        let m = Matrix::from(vec![vec![1., 5.], vec![-3., 2.]]);
        assert_eqf64!(m.determinant(), 17.)
    }
}
