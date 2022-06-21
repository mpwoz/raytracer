use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
use crate::assert_eqf64;
use crate::eqf64::eq_f64;

mod operators;

/// Deriving Copy/Clone treats these as primitive values. That means passing by value creates copies
/// so we don't lose ownership in the caller. Tuples are treated as immutable.
#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    /* whether this is a point (1) or vector (0) */
}

impl fmt::Display for Tuple {
    /// adds ability to use the '{}' print marker for Tuples (i.e. toString)
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // can't @ bind a var if there are further matches in the inner patter as of https://github.com/rust-lang/rust/issues/65490
        // once that's fixed can refactor this to read like: tup@Tuple{x, y, z} if tup.is_point() => ...
        match self {
            Tuple { x, y, z, .. } if self.is_point() => {
                write!(f, "Tuple(point): {}, {}, {}", x, y, z)
            }
            Tuple { x, y, z, .. } if self.is_vector() => {
                write!(f, "Tuple(vector): {}, {}, {}", x, y, z)
            }
            Tuple { x, y, z, w } => write!(
                f,
                "Invalid Tuple (neither point nor vector)! x:{}, y:{}, z:{}, w:{}",
                x, y, z, w
            ),
        }
    }
}

/// Check that both operands are vectors.
macro_rules! assert_vectors {
    ($lhs: expr, $rhs: expr) => {{
        assert!($lhs.is_vector(), "LHS must be a vector, but was {}", $lhs);
        assert!($rhs.is_vector(), "RHS must be a vector, but was {}", $rhs);
    }};
}

#[cfg(test)]
mod assert_vectors_tests {
    use super::*;

    #[test]
    fn test_assert_vectors() {
        assert_vectors!(Tuple::vector(1., 2., 3.), Tuple::vector(2., 3., 4.))
    }

    #[test]
    #[should_panic]
    fn test_assert_non_vectors() {
        assert_vectors!(Tuple::point(1., 2., 3.), Tuple::vector(2., 3., 4.))
    }
}

/// Instance methods
impl Tuple {
    /// Return a Vector's magnitude using Pythagoras' theorem.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Dot product of this vector with another (defined as sum of products of each vector component)
    /// Intuitively: small number (-1): vectors point away from each other. Large (1) they point the same direction.
    /// dot of two unit vectors is the cosine of angle between them.
    pub fn dot(&self, rhs: Tuple) -> f64 {
        assert_vectors!(self, rhs);
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Cross product of two vectors
    /// Result is a vector perpendicular to them
    pub fn cross(self, rhs: Self) -> Self {
        assert_vectors!(self, rhs);
        let x: f64 = self.y * rhs.z - self.z * rhs.y;
        let y: f64 = self.z * rhs.x - self.x * rhs.z;
        let z: f64 = self.x * rhs.y - self.y * rhs.x;
        Tuple::vector(x, y, z)
    }

    /// Hadamard product of two tuples (multiply individual components together).
    /// Can be used to blend a color with another.
    pub fn hadamard(self, rhs: Self) -> Self {
        Tuple {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }

    pub fn round(&self, places: i32) -> Self {
        let fac: f64 = 10_f64.powi(places);
        let round = |i: f64| (i * fac).round() / fac;
        Tuple {
            x: round(self.x),
            y: round(self.y),
            z: round(self.z),
            w: round(self.w),
        }
    }

    pub fn normalized(&self) -> Self {
        let len = self.magnitude();
        self.clone() / len
    }

    pub fn reflect(&self, normal: Tuple) -> Self {
        *(self) - normal * 2.0 * (self.dot(normal))
    }

    pub fn origin() -> Tuple {
        Self::point(0., 0., 0.)
    }
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        eq_f64(self.w, 1_f64)
    }

    pub fn is_vector(&self) -> bool {
        eq_f64(self.w, 0_f64)
    }
}

pub fn point<X, Y, Z>(x: X, y: Y, z: Z) -> Tuple
    where
        X: Into<f64>,
        Y: Into<f64>,
        Z: Into<f64>,
{
    Tuple::point(x.into(), y.into(), z.into())
}

pub fn vector<X, Y, Z>(x: X, y: Y, z: Z) -> Tuple
    where
        X: Into<f64>,
        Y: Into<f64>,
        Z: Into<f64>,
{
    Tuple::vector(x.into(), y.into(), z.into())
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

        assert_eqf64!(point.x, 4.3);
        assert_eqf64!(point.y, -4.2);
        assert_eqf64!(point.z, 3.1);
        assert_eqf64!(point.w, 1.0);
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

        assert_eqf64!(vector.x, 4.3);
        assert_eqf64!(vector.y, -4.2);
        assert_eqf64!(vector.z, 3.1);
        assert_eqf64!(vector.w, 0.0);
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
    fn test_magnitude() {
        fn test(vec: Tuple, expected: f64) {
            assert_eqf64!(vec.magnitude(), expected);
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
            assert_eq!(vec.normalized(), expected);
        }

        test(Tuple::vector(4., 0., 0.), Tuple::vector(1., 0., 0.));
        test(
            Tuple::vector(1., 2., 3.),
            Tuple::vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt(),
            ),
        );
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eqf64!(a.dot(b), 20.);
    }

    #[test]
    fn test_cross_product() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(b), vector(-1, 2, -1));
        assert_eq!(b.cross(a), vector(1, -2, 1));
    }

    #[test]
    fn test_hadamard_product() {
        let a = vector(1., 2., 3);
        let b = vector(2., 3., 4);
        assert_eq!(a.hadamard(b), vector(2., 6., 12.));
        assert_eq!(a.hadamard(b), b.hadamard(a));
    }

    #[test]
    fn reflecting_vector_at_45deg() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);
        assert_eq!(v.reflect(n), vector(1, 1, 0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface_normal() {
        let v = vector(0, -1, 0);
        let trt = 2_f64.sqrt() / 2.0;
        let n = vector(trt, trt, 0);
        assert_eq!(v.reflect(n).round(5), vector(1, 0, 0));
    }
}
