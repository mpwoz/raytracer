use crate::assert_eqf64;
use crate::eqf64::eq_f64;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    elements: Vec<Vec<f64>>,
}

impl Matrix {
    /// Constructs a new matrix of given dimensions - all elements initialized to 0
    pub fn new(width: usize, height: usize) -> Self {
        let elements: Vec<Vec<f64>> = vec![vec![0.; width]; height];
        Matrix {
            width,
            height,
            elements,
        }
    }

    /// Constructs a new matrix given the 2d-array of elements, assumed to be well-formed.
    pub fn from(elements: Vec<Vec<f64>>) -> Self {
        let height = elements.len();
        let width = elements[0].len();

        for row in &elements {
            assert_eq!(row.len(), width);
        }

        Matrix {
            width,
            height,
            elements,
        }
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        // TODO make this an assert macro with good error msg
        (row < self.height) && (col < self.width)
    }

    /// Get a single value from the matrix given its:
    /// row (0-indexed from the top) and column (0-indexed from left)
    pub fn get(&self, row: usize, col: usize) -> f64 {
        assert!(self.in_bounds(row, col));
        self.elements[row][col]
    }

    /// Set a single element in the matrix given a row and column index
    pub fn set(&mut self, row: usize, col: usize, element: f64) {
        assert!(self.in_bounds(row, col));
        self.elements[row][col] = element;
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let same_dims = self.width == other.width && self.height == other.height;
        if !same_dims {
            return false;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                if !eq_f64(self.get(row, col), other.get(row, col)) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let m1 = Matrix::from(vec![vec![0.]]);
        let m2 = Matrix::new(1, 1);

        assert_eq!(m1.width, m2.width);
        assert_eq!(m1.height, m2.height);
        assert_eqf64!(m1.get(0, 0), m2.get(0, 0));
    }

    #[test]
    fn test_4x4() {
        let m = Matrix::from(vec![
            vec![1., 2., 3., 4.],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9., 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eqf64!(m.get(0, 0), 1.);
        assert_eqf64!(m.get(0, 3), 4.);
        assert_eqf64!(m.get(3, 0), 13.5);
        assert_eqf64!(m.get(3, 3), 16.5);
        assert_eqf64!(m.get(2, 2), 11.0);
    }

    #[test]
    fn test_3x3() {
        let m = Matrix::from(vec![
            vec![1., 2., 3.],
            vec![5.5, 6.5, 7.5],
            vec![13.5, 14.5, 15.5],
        ]);

        assert_eqf64!(m.get(0, 0), 1.);
        assert_eqf64!(m.get(0, 2), 3.);
        assert_eqf64!(m.get(2, 0), 13.5);
        assert_eqf64!(m.get(2, 2), 15.5);
    }

    #[test]
    fn test_2x2() {
        let m = Matrix::from(vec![vec![1., 3.], vec![13.5, 15.5]]);

        assert_eqf64!(m.get(0, 0), 1.);
        assert_eqf64!(m.get(0, 1), 3.);
        assert_eqf64!(m.get(1, 0), 13.5);
        assert_eqf64!(m.get(1, 1), 15.5);
    }

    #[test]
    #[should_panic]
    fn test_ctor_validation() {
        Matrix::from(vec![vec![1., 2.], vec![1.]]);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Matrix::from(vec![vec![0.]]), Matrix::new(1, 1));

        let m1 = Matrix::from(vec![
            vec![1., 0., 0.],
            vec![1., 0., 0.],
            vec![1., 0., 0.],
        ]);

        let mut m2 = Matrix::new(3, 3);
        assert_ne!(m1, m2);

        m2.set(0, 0, 1.);
        m2.set(1, 0, 1.);
        m2.set(2, 0, 1.);
        assert_eq!(m1, m2);
    }
}
