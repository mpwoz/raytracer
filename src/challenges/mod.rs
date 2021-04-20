use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{hit, sphere};
use crate::tuple::point;

pub fn chapter5_render_sphere_silhouette() {
    let camera_origin = point(0, 0, -5);
    let sphere = sphere();
    let wall_z_coord = 5_f64;
    let half_wall = 5.;

    let canvas_dimensions = 150;
    let mut canvas = Canvas::new(canvas_dimensions, canvas_dimensions);

    let wall_dimensions = 10;

    let scaling_factor = (wall_dimensions as f64) / (canvas_dimensions as f64);
    // canvas assumed to be at z=0, this translates the x/y-coordinates to "wall space"
    let canvas_to_wall: Matrix = Matrix::transformation()
        .scale(scaling_factor, scaling_factor, 1.)
        .translate(-half_wall, -half_wall, wall_z_coord)
        .rotate_z(PI);


    for x in 0..canvas_dimensions {
        println!("{}% done", 100 * x / canvas_dimensions);
        for y in 0..canvas_dimensions {
            let wall_coordinate = &canvas_to_wall * &point(x as f64, y as f64, 0);

            let ray_direction = wall_coordinate - camera_origin;
            let ray = Ray::new(camera_origin, ray_direction);

            let intersections = &sphere.intersections(ray);
            let hit = hit(intersections);

            if hit.is_some() {
                canvas.write_pixel(x, y, Color::rgb(1., 0.2, 0.2));
            }
        }
    }

    canvas.save_to_disk("/tmp/output.ppm");
}
