use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::canvas::Canvas;
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;

mod eqf64;
mod canvas;
mod color;
mod projectile;
mod tuple;
mod matrix;
mod assert_eqf64;

fn main() {
    println!("Hello, world!");
    ch2_projectile();
}

/// Chapter 2 drawing a projectile
/// todo: move to its own module
fn ch2_projectile() {
    let mut c = Canvas::new(900, 550);

    // environment
    let gravity = Tuple::vector(0., -0.1, 0.);
    let wind = Tuple::vector(-0.01, 0., 0.);
    let e = Environment { gravity, wind };

    // projectile
    let start = Tuple::point(0., 1., 0.);
    let velocity = Tuple::vector(1., 1.8, 0.);
    let velocity = Tuple::normalized(velocity) * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };

    loop {
        // if point is out of bounds, stop iterating
        if p.is_out_of_bounds(&c) {
            break;
        }

        p.draw_on(&mut c);

        // update point's velocity and position based on wind, gravity, time
        p = p.update(&e, 0.1);
    }

    // write canvas to disk
    let ppm = c.render_as_ppm();

    let path = Path::new("/tmp/output.ppm");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote output to {}", display),
    };
}
