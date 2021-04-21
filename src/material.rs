use std::char::ToUppercase;

use crate::assert_eqf64;
use crate::color::Color;
use crate::light::PointLight;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
    ) -> Color {
        let effective_color = self.color * light.intensity;

        let lightv = (light.position - position).normalized();

        let ambient = effective_color * self.ambient;

        let (diffuse, specular);

        let light_dot_normal = lightv.dot(normalv);
        if (light_dot_normal < 0.0) {
            diffuse = Color::BLACK;
            specular = Color::BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye < 0.0 {
                specular = Color::BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

pub fn material() -> Material {
    Material::new()
}

#[cfg(test)]
mod tests {
    use crate::color::color;
    use crate::light::{point_light, PointLight};
    use crate::tuple::{point, vector};

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::rgb(1., 1., 1.));
        assert_eqf64!(m.ambient, 0.1);
        assert_eqf64!(m.diffuse, 0.9);
        assert_eqf64!(m.specular, 0.9);
        assert_eqf64!(m.shininess, 200.0);
    }

    // Common test setup for lighting tests. Inputs are an eye vector, position of the light, and
    // the expected color. Assumed to always look at the origin point (0,0,0)
    macro_rules! lighting_test {
        ($eyev:expr, $lightpos:expr, $expected_color:expr) => {
            let m = material();
            let position = point(0, 0, 0);

            let eyev: Tuple = $eyev;
            let normalv = vector(0, 0, -1);
            let light = point_light($lightpos, color(1, 1, 1));

            let result = m.lighting(light, position, eyev, normalv);
            assert_eq!(result.round(5), $expected_color);
        };
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let eyev = vector(0, 0, -1);
        let lightpos = point(0, 0, -10);
        let expected = color(1.9, 1.9, 1.9);
        // all components max
        lighting_test!(eyev, lightpos, expected);
    }

    #[test]
    fn lighting_eye_offset_45deg() {
        let trt = 2_f64.sqrt() / 2.0;
        let eyev = vector(0, trt, -trt);
        let lightpos = point(0, 0, -10);
        let expected = color(1.0, 1.0, 1.0);
        lighting_test!(eyev, lightpos, expected);
    }

    #[test]
    fn lighting_light_offset_45deg() {
        let eyev = vector(0, 0, 1);
        let lightpos = point(0, 10, -10);
        let expected = color(0.7364, 0.7364, 0.7364);
        lighting_test!(eyev, lightpos, expected);
    }

    #[test]
    fn lighting_eye_in_path_of_reflection() {
        let trt = 2_f64.sqrt() / 2.0;
        let eyev = vector(0, -trt, -trt);
        let lightpos = point(0, 10, -10);
        let expected = color(1.6364, 1.6364, 1.6364);
        lighting_test!(eyev, lightpos, expected);
    }

    #[test]
    fn lighting_lamp_behind_surface() {
        let eyev = vector(0, 0, -1);
        let lightpos = point(0, 0, 10);
        let expected = color(0.1, 0.1, 0.1);
        // only ambient
        lighting_test!(eyev, lightpos, expected);
    }
}
