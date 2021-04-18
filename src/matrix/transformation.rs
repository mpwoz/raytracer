use std::f64::consts::PI;

use crate::matrix::Matrix;
use crate::tuple::Tuple;

impl Matrix {
    /// Identity 4x4 matrix for transformations.
    pub(crate) fn transformation() -> Matrix {
        Matrix::from(vec![
            vec![1., 0., 0., 0.],
            vec![0., 1., 0., 0.],
            vec![0., 0., 1., 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::from(vec![
            vec![1., 0., 0., x],
            vec![0., 1., 0., y],
            vec![0., 0., 1., z],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::from(vec![
            vec![x, 0., 0., 0.],
            vec![0., y, 0., 0.],
            vec![0., 0., z, 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_x(r: f64) -> Matrix {
        Matrix::from(vec![
            vec![1., 0., 0., 0.],
            vec![0., r.cos(), -r.sin(), 0.],
            vec![0., r.sin(), r.cos(), 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_y(r: f64) -> Matrix {
        Matrix::from(vec![
            vec![r.cos(), 0., r.sin(), 0.],
            vec![0., 1., 0., 0.],
            vec![-r.sin(), 0., r.cos(), 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_z(r: f64) -> Matrix {
        Matrix::from(vec![
            vec![r.cos(), -r.sin(), 0., 0.],
            vec![r.sin(), r.cos(), 0., 0.],
            vec![0., 0., 1., 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        Matrix::from(vec![
            vec![1., xy, xz, 0.],
            vec![yx, 1., yz, 0.],
            vec![zx, zy, 1., 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z) * self
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Matrix::rotation_x(r) * self
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Matrix::rotation_y(r) * self
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Matrix::rotation_z(r) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        let translated = transform * p;
        assert_eq!(translated, Tuple::point(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5., -3., 2.);
        let inv = transform.inverse();
        let p = Tuple::point(-3., 4., 5.);
        assert_eq!(inv * p, Tuple::point(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5., -3., 2.);
        let v = Tuple::vector(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);
        assert_eq!(transform * p, Tuple::point(-8., 18., 32.));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2., 3., 4.);
        let v = Tuple::vector(-4., 6., 8.);
        assert_eq!(transform * v, Tuple::vector(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2., 3., 4.);
        let v = Tuple::vector(-4., 6., 8.);
        let inv = transform.inverse();
        assert_eq!(inv * v, Tuple::vector(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1., 1., 1.);
        let p = Tuple::point(-4., 6., 8.);
        assert_eq!(transform * p, Tuple::point(4., 6., 8.))
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);

        let rt = 2_f64.sqrt() / 2_f64;
        assert_eq!(half_quarter * p, Tuple::point(0., rt, rt));
        assert_eq!(full_quarter * p, Tuple::point(0., 0., 1.));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let inv = half_quarter.inverse();

        let rt = 2_f64.sqrt() / 2_f64;
        assert_eq!(inv * p, Tuple::point(0., rt, -rt));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0., 0., 1.);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);

        let rt = 2_f64.sqrt() / 2_f64;
        assert_eq!(half_quarter * p, Tuple::point(rt, 0., rt));
        assert_eq!(full_quarter * p, Tuple::point(1., 0., 0.));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);

        let rt = 2_f64.sqrt() / 2_f64;
        assert_eq!(half_quarter * p, Tuple::point(-rt, rt, 0.));
        assert_eq!(full_quarter * p, Tuple::point(-1., 0., 0.));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        // apply rotation first;
        let p2 = a * p;
        assert_eq!(p2.round(5), Tuple::point(1.0, -1.0, 0.0));
        // then apply scaling;
        let p3 = b * p2;
        assert_eq!(p3.round(5), Tuple::point(5.0, -5.0, 0.0));
        // then apply translation;
        let p4 = c * p3;
        assert_eq!(p4.round(5), Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_fluent_api_transformations() {
        // Same as above, but uses the fluent API to ensure correct ordering.
        let p = Tuple::point(1.0, 0.0, 1.0);
        let t = Matrix::transformation()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
