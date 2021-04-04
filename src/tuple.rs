use std::ops::{Add, Div, Mul, Neg, Sub};

/// Deriving Copy/Clone treats these as primitive values. That means passing by value creates copies
/// so we don't lose ownership in the caller. Tuples are treated as immutable.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    /* whether this is a point (1) or vector (0) */
}

/// Instance methods
impl Tuple {
    /// Return a Vector's magnitude using Pythagoras' theorem.
    pub(crate) fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Dot product of this vector with another (defined as sum of products of each vector component)
    /// Intuitively: small number (-1): vectors point away from each other. Large (1) they point the same direction.
    /// dot of two unit vectors is the cosine of angle between them.
    pub(crate) fn dot(&self, rhs: Tuple) -> f64 {
        assert_eq!(self.w, 0., "left side must be a vector, but was: {}", self);
        assert_eq!(rhs.w, 0., "right side must be a vector, but was: {}", rhs);
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }
}

/// Static methods
impl Tuple {
    pub(crate) fn normalized(vector: Self) -> Self {
        let len = vector.magnitude();
        vector / len
    }

    pub(crate) fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub(crate) fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub(crate) fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub(crate) fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let point = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert_eq!(point.is_point(), true);
        assert_eq!(point.is_vector(), false);

        assert_eq!(Tuple::point(4.3, -4.2, 3.1), point)
    }

    #[test]
    fn test_vector() {
        let vector = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
        assert_eq!(vector.is_point(), false);
        assert_eq!(vector.is_vector(), true);

        assert_eq!(Tuple::vector(4.3, -4.2, 3.1), vector)
    }

    #[test]
    fn test_equality_floating_points() {
        // so precise
        let p1 = Tuple::point(9.0, 1.00000000000000000000000000000001, 9999999999.9);
        let p2 = Tuple::point(8.9999999999999999999999999999, 1.0, 9999999999.9);
        let p3 = Tuple::point(1., 1., 1.);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

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
        let a = Tuple::vector(3., 2., 1.);
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

    #[test]
    fn test_magnitude() {
        fn test(vec: Tuple, expected: f64) {
            assert_eq!(vec.magnitude(), expected);
        }

        test(Tuple::vector(1., 0., 0.), 1.);
        test(Tuple::vector(0., 1., 0.), 1.);
        test(Tuple::vector(0., 0., 1.), 1.);

        let expected = 14.0_f64.sqrt();
        let v = Tuple::vector(1., 2., 3.);
        test(v, expected);
        test(-v, expected);
    }

    #[test]
    fn test_unit_vector() {
        fn test(vec: Tuple, expected: Tuple) {
            assert_eq!(Tuple::normalized(vec), expected);
        }

        test(Tuple::vector(4., 0., 0.), Tuple::vector(1., 0., 0.));
        test(
            Tuple::vector(1., 2., 3.),
            Tuple::vector(
                (1.0 / 14.0_f64.sqrt()),
                (2.0 / 14.0_f64.sqrt()),
                (3.0 / 14.0_f64.sqrt()),
            ),
        );
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.dot(b), 20.);
    }
}
