use crate::matrix::Matrix;

/// Implements some "functional" methods for Matrix, used as convenience methods elsewhere
impl Matrix {
    /// Constructs a new matrix by applying f to each element
    pub fn map_elements<F>(&self, f: F) -> Matrix
        where
            F: Fn(f64) -> f64,
    {
        let mut m = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let e = f(self.get(y, x));
                m.set(y, x, e);
            }
        }

        m
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping() {
        let m = Matrix::from(vec![
            vec![1., 2., 3., ],
            vec![1., 2., 3., ],
            vec![1., 2., 3., ],
        ]);

        let exp = Matrix::from(vec![
            vec![-1., -2., -3., ],
            vec![-1., -2., -3., ],
            vec![-1., -2., -3., ],
        ]);

        assert_eq!(m.map_elements(|e| -1.0 * e), exp);
    }
}