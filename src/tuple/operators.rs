use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::eqf64::eq_f64;
use crate::tuple::Tuple;

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::vector(0., 0., 0.) - self
    }
}

/// Scalar multiplication
impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

/// We have to redefine scalar multiplication to make it commutative.
impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

/// division is just multiplication but inversed.
impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        eq_f64(self.x, other.x)
            && eq_f64(self.y, other.y)
            && eq_f64(self.z, other.z)
            && eq_f64(self.w, other.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::vector;

    use super::*;

    #[test]
    fn test_adding_two_tuples() {
        let p = Tuple::point(3., -2., 5.);
        let v = Tuple::vector(-2., 3., 1.);

        assert_eq!(p + v, Tuple::point(1., 1., 6.))
    }

    #[test]
    fn test_subtracting_two_points() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::point(5., 6., 7.);

        assert_eq!(a - b, Tuple::vector(-2., -4., -6.))
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let a = Tuple::vector(3., 2., 1.);
        let b = Tuple::vector(5., 6., 7.);

        assert_eq!(a - b, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn test_negating_vectors() {
        let a = vector(3, 2, 1);
        let b = -a;

        assert_eq!(-a, Tuple::vector(-3., -2., -1.));
        assert_eq!(-b, a);
    }

    #[test]
    fn test_multiplying_and_dividing_vectors() {
        let a = Tuple::vector(1., -2., 3.);
        assert_eq!(a * 3.5, Tuple::vector(3.5, -7., 10.5));
        assert_eq!(a * 0.5, Tuple::vector(0.5, -1., 1.5));

        // is commutative?
        assert_eq!(3.5 * a, Tuple::vector(3.5, -7., 10.5));
        assert_eq!(0.5 * a, Tuple::vector(0.5, -1., 1.5));

        // division
        assert_eq!(a * 0.5, a / 2.);
    }
}
