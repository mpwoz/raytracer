use crate::assert_eqf64;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{CanIntersect, Shape};
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::transformation(),
        }
    }
}

impl CanIntersect for Sphere {
    fn transform(&self) -> &Matrix {
        &(self.transform)
    }

    fn intersect(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Tuple::origin(); // sphere assumed to be at origin

        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let a = ray.direction.dot(ray.direction);
        let b = 2. * (ray.direction.dot(sphere_to_ray));
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
        let discriminant = b.powi(2) - (4. * a * c);

        // if discriminant is negative, the ray misses the sphere
        if discriminant < 0. {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn a_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        // the default transform of a shape should be the identity matrix
        assert_eq!(s.transform, Matrix::identity(4));

        // The "t" values at which the ray intersects the sphere
        let xs = s.intersect(r);
        assert_eq!(xs, vec!(4.0, 6.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, vec!(5.0, 5.0));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, vec!(-1.0, 1.0));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, vec!(-6.0, -4.0));
    }
}
