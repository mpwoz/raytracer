use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::canvas::Canvas;
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

fn main() {
    println!("Hello, world!");
    // ch2_projectile();
    ch4_clock_face();
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

fn draw_dot(canvas: &mut Canvas, x: i32, y: i32, color: Color) -> &mut Canvas {
    let size = 2;
    for px in ((x - size) as usize)..((x + size) as usize) {
        for py in ((y - size) as usize)..((y + size) as usize) {
            canvas.write_pixel(px, py, color);
        }
    }
    canvas
}

fn render_clock_face(mut canvas: &mut Canvas, radius: f64, n_points: i32, canvas_dim: f64) -> &mut Canvas {
    for i in 0..n_points {
        let p = Tuple::point(0., 0., 0.);
        let angle: f64 = f64::from(i) * ((2.0 * PI) / f64::from(n_points));
        let t: Matrix = Matrix::transformation()
            .translate(radius, 0., 0.)
            .rotate_z(angle) // rotate by 1/nth of the "clock face" each time
            .translate(canvas_dim / 2.0, canvas_dim / 2.0, 0.0); // centers origin in middle of canvas

        let position = t * p;

        // draw the point on canvas
        let color = Color::rgb(1.0, 1.0, 1.0);
        canvas = draw_dot(canvas, position.x as i32, position.y as i32, color);
    }

    canvas
}

fn ch4_clock_face() {
    let radius = 200.0;
    let n_points = 12;

    let margin = 20.0;
    let canvas_dim = (radius + margin) * 2_f64;
    let mut canvas = &mut Canvas::new(canvas_dim as usize, canvas_dim as usize);

    // Hours
    for rmod in 0..10 {
        canvas = render_clock_face(canvas, radius - (rmod as f64), n_points, canvas_dim);
        canvas = render_clock_face(canvas, radius + (rmod as f64), n_points, canvas_dim);
    }
    // Minutes
    canvas = render_clock_face(canvas, radius, n_points * 5, canvas_dim);


    // save canvas to file
    // write canvas to disk
    let ppm = canvas.render_as_ppm();

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
