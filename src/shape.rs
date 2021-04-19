use crate::assert_eqf64;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub trait CanIntersect {
    fn transform(&self) -> &Matrix;
    fn intersect(&self, ray: Ray) -> Vec<f64>;
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersections(&self, ray: Ray) -> Vec<Intersection> {
        self.intersect(ray)
            .iter()
            .map(|t| Intersection {
                t: *t,
                object: self,
            })
            .collect()
    }
}

pub fn sphere() -> Shape {
    Shape::Sphere(Sphere::new())
}

impl CanIntersect for Shape {
    fn transform(&self) -> &Matrix {
        match *self {
            Shape::Sphere(ref s) => s.transform(),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<f64> {
        match *self {
            Shape::Sphere(ref s) => s.intersect(ray),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, vector};

    use super::*;

    #[test]
    fn test_intersection_with_sphere() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        assert_eq!(s.intersect(r), vec!(4.0, 6.0));

        // Test that intersection objects store references to the original shape
        let is = s.intersections(r);
        assert_eqf64!(is[0].t, 4.0);
        assert_eqf64!(is[1].t, 6.0);
        assert_eq!(is[0].object, &s);
        assert_eq!(is[1].object, &s);
    }
}
