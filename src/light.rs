use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new() -> PointLight {
        PointLight {
            position: Tuple::origin(),
            intensity: Color::WHITE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::point;

    use super::*;

    #[test]
    fn default_light_values() {
        let l = PointLight::new();
        assert_eq!(l.position, point(0, 0, 0));
        assert_eq!(l.intensity, Color::rgb(1., 1., 1.));
    }
}