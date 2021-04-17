use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::eqf64::eq_f64;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

/// This file has all the Operator implementations for Matrix
/// Adding, subtracting, and multiplying both by another matrix as well as a Tuple (vector)

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

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = (self, rhs);

        // Determine the size of resulting multiplied matrix
        // also ensure these matrices can be multiplied
        let result_height = a.height;
        let result_width = b.width;
        let common_dimension = a.width;
        assert_eq!(a.width, b.height);

        let mut m = Matrix::new(result_width, result_height);

        // M_i,j is the dot product of row A_i_x with column B_x_j for x in 0..common_dimension
        for row in 0..result_height {
            for col in 0..result_width {
                let dot = (0..common_dimension)
                    .map(|i| a.get(row, i) * b.get(i, col))
                    .fold(0_f64, |a, b| a.add(b));

                m.set(row, col, dot);
            }
        }

        // return multiplied matrix
        m
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let b = Matrix::from_tuple(rhs).transpose();
        self * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(Matrix::from(vec![vec![0.]]), Matrix::new(1, 1));

        let m1 = Matrix::from(vec![vec![1., 0., 0.], vec![1., 0., 0.], vec![1., 0., 0.]]);

        let mut m2 = Matrix::new(3, 3);
        assert_ne!(m1, m2);

        m2.set(0, 0, 1.);
        m2.set(1, 0, 1.);
        m2.set(2, 0, 1.);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_multiplication() {
        let m1 = Matrix::from(vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ]);
        let m2 = Matrix::from(vec![
            vec![-2., 1., 2., 3.],
            vec![3., 2., 1., -1.],
            vec![4., 3., 6., 5.],
            vec![1., 2., 7., 8.],
        ]);

        let exp = Matrix::from(vec![
            vec![20., 22., 50., 48.],
            vec![44., 54., 114., 108.],
            vec![40., 58., 110., 102.],
            vec![16., 26., 46., 42.],
        ]);

        assert_eq!(m1 * m2, exp);
    }

    #[test]
    fn test_tuple_multiplication() {
        let m = Matrix::from(vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);

        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        let exp = Tuple {
            x: 18.0,
            y: 24.0,
            z: 33.0,
            w: 1.0,
        };
        let exp = Matrix::from_tuple(exp).transpose();

        assert_eq!(m * t, exp);
    }
}
