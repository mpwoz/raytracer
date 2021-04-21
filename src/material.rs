use crate::assert_eqf64;
use crate::color::Color;

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
}


#[cfg(test)]
mod tests {
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
}