use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::{Color, color};
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{CanIntersect, hit, Intersection, Shape, sphere};
use crate::sphere::Sphere;
use crate::tuple::point;

pub fn chapter6_render_shaded_sphere() {
    let camera_origin = point(0, 0, -5);

    // set up Sphere with a material
    let mut s = Sphere::new();
    let mut m = Material::new();
    m.color = color(1, 0.2, 1);
    s.material = m;
    let sphere = Shape::Sphere(s);
    let light = PointLight {
        position: point(-10, 10, -10),
        intensity: Color::WHITE,
    };

    let wall_z_coord = 5_f64;
    let half_wall = 5.;

    let canvas_dimensions = 500;
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

            let ray_direction = (wall_coordinate - camera_origin).normalized();
            let ray = Ray::new(camera_origin, ray_direction);

            let intersections = &sphere.intersections(ray);
            let hit = hit(intersections);

            if hit.is_none() {
                continue // skip non-intersecting rays
            }

            let intersection = hit.unwrap();

            let pos = ray.position(intersection.t);
            let normalv = intersection.object.normal_at(pos);
            let eyev = -ray_direction;
            let hit_color = intersection.object.material()
                .lighting(light, pos, eyev, normalv);
            canvas.write_pixel(x, y, hit_color);
        }
    }

    canvas.save_to_disk("/tmp/ch6_shaded_sphere.ppm");
}

