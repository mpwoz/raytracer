use std::cmp::Ordering;

use crate::assert_eqf64;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub trait CanIntersect {
    fn transform(&self) -> &Matrix;
    fn intersect(&self, ray: Ray) -> Vec<f64>;
}

#[derive(Debug, PartialEq, Clone)]
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

pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|i| i.t > 0.0)
        .reduce(|a, b| if a.t < b.t { a } else { b })
    // TODO can use f64::total_cmp once it's in stable:
    //  .min_by(|a, b| a.t.total_cmp(&b.t))
}

pub fn sphere() -> Shape {
    Shape::Sphere(Sphere::new())
}

pub fn intersection<A: Into<f64>>(t: A, obj: &Shape) -> Intersection {
    Intersection {
        t: t.into(),
        object: obj,
    }
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

    #[test]
    fn test_hit_all_positive() {
        let s = sphere();
        let ia = intersection(1, &s);
        let ib = intersection(2, &s);
        let is = vec![ia.clone(), ib.clone()];
        assert_eq!(hit(&is), Some(&ia));
    }

    #[test]
    fn test_hit_some_negative() {
        let s = sphere();
        let ia = intersection(-1, &s);
        let ib = intersection(1, &s);
        let is = vec![ia.clone(), ib.clone()];
        assert_eq!(hit(&is), Some(&ib));
    }

    #[test]
    fn test_hit_all_negative() {
        let s = sphere();
        let ia = intersection(-2, &s);
        let ib = intersection(-1, &s);
        let is = vec![ia.clone(), ib.clone()];
        assert_eq!(hit(&is), None);
    }
}
