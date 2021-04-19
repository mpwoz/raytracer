use crate::assert_eqf64;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::transformation(),
        }
    }
    pub fn intersect(&self, ray: Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray.origin - Tuple::origin(); // sphere assumed to be at origin

        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let a = ray.direction.dot(ray.direction);
        let b = 2. * (ray.direction.dot(sphere_to_ray));
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
        let discriminant = b.powi(2) - (4. * a * c);

        // if discriminant is negative, the ray misses the sphere
        if discriminant < 0. {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Some((t1, t2))
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
        assert_eq!(xs, Some((4.0, 6.0)));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((5.0, 5.0)));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((-1.0, 1.0)));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs, Some((-6.0, -4.0)));
    }
}
