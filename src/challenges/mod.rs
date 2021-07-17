use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::{color, Color};
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{CanIntersect, hit, Intersection, Shape, sphere};
use crate::sphere::Sphere;
use crate::tuple::point;

pub mod ch2_projectile;
pub mod ch4_clock;
pub mod ch6_shading_spheres;

// TODO pass this in as a cli param from cargo run
static OUTPUT_DIRECTORY: &str = "./output";

/// prepends the project's output directory so that things are all written to the same place.
pub fn outfile(filename: &str) -> String {
    let formatted: String = format!("{}/{}", OUTPUT_DIRECTORY, filename);
    return formatted;
}

/// Utility method to automatically use the right extension, output to the project output directory, etc.
pub fn save(canvas: &Canvas, filename: &str) {
    let formatted = format!("{}.ppm", filename);
    canvas.save_to_disk(outfile(formatted.as_str()).as_str())
}
