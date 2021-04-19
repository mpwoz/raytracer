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
}

#[cfg(test)]
mod tests {
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
        let r = Ray::new(
            Tuple::point(2., 3., 4.),
            Tuple::vector(1., 0., 0.),
        );

        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }
}
