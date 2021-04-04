use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
use crate::assert_eqf64;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub(crate) fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn tuple(&self) -> Tuple {
        Tuple::vector(self.red, self.green, self.blue)
    }

    fn from_tuple(v: Tuple) -> Color {
        Color {
            red: v.x,
            green: v.y,
            blue: v.z,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from_tuple(self.tuple() + rhs.tuple())
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color::from_tuple(-self.tuple())
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::from_tuple(self.tuple() * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::from_tuple(rhs.tuple() * self)
    }
}

/// Multiplying colors with one another is used for blending using a Hadamard product
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::from_tuple(Tuple::hadamard(self.tuple(), rhs.tuple()))
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1_f64 / rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.tuple() == other.tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_components() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };

        assert_eqf64!(c.red, -0.5);
        assert_eqf64!(c.green, 0.4);
        assert_eqf64!(c.blue, 1.7);
    }

    #[test]
    fn test_adding_subtracting_colors() {
        let c1: Color = Color::rgb(0.9f64, 0.6f64, 0.75f64);
        let c2: Color = Color::rgb(0.7f64, 0.1f64, 0.25f64);
        assert_eq!(c1 + c2, Color::rgb(1.6, 0.7, 1.0));
        assert_eq!(c1 + c2, Color::rgb(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_multiplying_by_scalar() {
        let c1: Color = Color::rgb(0.2, 0.3, 0.4);
        let exp: Color = Color::rgb(0.4, 0.6, 0.8);
        assert_eq!(c1 * 2f64, exp);
    }

    #[test]
    fn test_multiplying_colors() {
        let c1: Color = Color::rgb(1f64, 0.2f64, 0.4f64);
        let c2: Color = Color::rgb(0.9f64, 1f64, 0.1f64);
        assert_eq!(c1 * c2, Color::rgb(0.9, 0.2, 0.04));
    }
}
