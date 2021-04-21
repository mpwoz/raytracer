use crate::assert_eqf64;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{CanIntersect, Shape};
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
    inverse_transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::transformation(),
            inverse_transform: Matrix::transformation().inverse(),
            material: Material::new(),
        }
    }

    pub fn set_transform(self: &mut Self, transform: Matrix) {
        self.transform = transform;
        self.inverse_transform = self.transform().inverse();
    }

    fn inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }
}

impl CanIntersect for Sphere {
    fn transform(&self) -> &Matrix {
        &(self.transform)
    }

    fn intersect(&self, ray: Ray) -> Vec<f64> {
        // Transform the ray by the inverse of the object's transform.
        // this changes "world coordinates" to "object coordinates"
        let ray = ray.transform(self.inverse_transform());

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

    fn normal_at(&self, point: Tuple) -> Tuple {
        let inverse_transform = self.inverse_transform();
        let object_point = inverse_transform * &point;
        let object_normal = object_point - Tuple::origin();
        let mut world_normal = inverse_transform.transpose() * object_normal;

        world_normal.w = 0.; // hack to fix w
        world_normal.normalized()
    }

    fn material(&self) -> Material {
        self.material
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::material::Material;
    use crate::ray::ray;
    use crate::tuple::{point, Tuple, vector};

    use super::*;

    #[test]
    fn can_set_transform() {
        let mut s = Sphere::new();
        let t = Matrix::transformation().translate(1., 2., 3.);
        s.set_transform(t.clone());

        assert_eq!(s.transform, t);
    }

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

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2., 2., 2.));
        let xs = s.intersect(r);
        assert_eq!(xs, vec!(3., 7., ));
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5., 0., 0.));
        let xs = s.intersect(r);
        assert_eq!(xs, vec!()); // empty
    }

    #[test]
    fn normals_on_sphere() {
        let s = Sphere::new();
        assert_eq!(s.normal_at(point(1, 0, 0)), vector(1, 0, 0));
        assert_eq!(s.normal_at(point(0, 1, 0)), vector(0, 1, 0));
        assert_eq!(s.normal_at(point(0, 0, 1)), vector(0, 0, 1));
        assert_eq!(s.normal_at(point(0, 0, -1)), vector(0, 0, -1));

        // non-axis-aligned
        let srt = 3_f64.sqrt() / 3_f64;
        let normal: Tuple = s.normal_at(point(srt, srt, srt));
        assert_eq!(normal, vector(srt, srt, srt));
        assert_eq!(normal, normal.normalized());
    }

    #[allow(clippy::all)] //noinspection RsApproxConstant
    #[test]
    fn normals_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0., 1., 0.));
        let n = s.normal_at(point(0, 1.70711, -0.70711));
        assert_eq!(n.round(5), vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn normals_on_transformed_sphere() {
        let mut s = Sphere::new();
        let t: Matrix = Matrix::scaling(1., 0.5, 1.) * Matrix::rotation_z(PI / 5.0);
        s.set_transform(t);

        let trt = 2_f64.sqrt() / 2.;
        let n = s.normal_at(point(0, trt, -trt));
        assert_eq!(n.round(5), vector(0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_materials() {
        let mut sphere = Sphere::new();

        // has default mat
        assert_eq!(sphere.material(), Material::new());

        // assigning a mat
        let mut mat = Material::new();
        mat.ambient = 1.0;
        sphere.material = mat;

        assert_eq!(sphere.material(), mat);
    }
}
