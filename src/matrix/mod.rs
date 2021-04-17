use crate::assert_eqf64;
use crate::tuple::Tuple;

pub mod determinant;
pub mod operators;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<Vec<f64>>,
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

    pub fn from_tuple(t: Tuple) -> Matrix {
        Matrix::from(vec![vec![t.x, t.y, t.z, t.w]])
    }

    /// Given a size s, return an s-by-s identity matrix (all 0 with 1s on the diagonal)
    pub fn identity(s: usize) -> Matrix {
        let mut m = Matrix::new(s, s);
        for i in 0..s {
            m.set(i, i, 1.0);
        }
        m
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

    pub fn transpose(&self) -> Matrix {
        let mut m = Matrix::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                m.set(j, i, self.get(i, j));
            }
        }

        m
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
    fn test_transpose() {
        let m1 = Matrix::from(vec![vec![1., 2., 3.], vec![5., 6., 7.], vec![9., 8., 7.]]);
        let t = Matrix::from(vec![vec![1., 5., 9.], vec![2., 6., 8.], vec![3., 7., 7.]]);
        assert_eq!(m1.transpose(), t);
    }

    #[test]
    fn test_identity() {
        let i = Matrix::identity(4);
        let m = Matrix::from(vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);

        assert_eq!(m.clone() * i.clone(), m);
        assert_eq!((m.clone() * i.clone()), m);

        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!((i * t), Matrix::from_tuple(t).transpose());
    }

    #[test]
    fn test_transpose_identity() {
        assert_eq!(Matrix::identity(4), Matrix::identity(4).transpose());
    }
}
