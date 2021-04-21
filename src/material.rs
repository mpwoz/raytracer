#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            ambient: 0.5,
            diffuse: 0.5,
            specular: 0.5,
            shininess: 0.5,
        }
    }
}
