use crate::assert_eqf64;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    // a point
    pub direction: Tuple, // a vector
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, transform: &Matrix) -> Ray {
        let origin = transform * &self.origin;
        let direction = transform * &self.direction;
        Ray { origin, direction }
    }
}

pub fn ray(p: Tuple, v: Tuple) -> Ray {
    Ray::new(p, v)
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, vector};

    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);

        let r = Ray::new(origin, direction);

        assert_eq!(r.direction, direction);
        assert_eq!(r.origin, origin);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(point(2, 3, 4), vector(1, 0, 0));

        assert_eq!(r.position(0.), point(2, 3, 4));
        assert_eq!(r.position(1.), point(3, 3, 4));
        assert_eq!(r.position(-1.), point(1, 3, 4));
        assert_eq!(r.position(2.5), point(4.5, 3, 4));
    }

    #[test]
    fn translating_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = Matrix::translation(3., 4., 5.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(4, 6, 8));
        assert_eq!(r2.direction, vector(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = Matrix::scaling(2., 3., 4.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(2, 6, 12));
        assert_eq!(r2.direction, vector(0, 3, 0));
    }
}
