use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use challenges::{
    ch2_projectile,
    ch4_clock::*,
    ch6_shading_spheres,
};

use crate::canvas::Canvas;
use crate::challenges::{ch4_clock, outfile};
use crate::color::Color;
use crate::matrix::Matrix;
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;

mod assert_eqf64;
mod canvas;
mod color;
mod eqf64;
mod matrix;
mod projectile;
mod tuple;
mod ray;
mod sphere;
mod shape;
mod challenges;
mod light;
mod material;

fn main() {
    let now = Instant::now();

    println!("Starting renders");

    ch2_projectile::ch2_projectile();
    let should_render_animation = true;
    ch4_clock::render_clock_things(should_render_animation);
    ch6_shading_spheres::chapter6_render_shaded_sphere(1000);

    let elapsed = now.elapsed().as_millis();
    println!("Elapsed: {:.2} ms", elapsed);
}
