use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::challenges::{outfile, save};
use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn render_clock_things(animation: bool) {
    ch4_clock_face();

    if animation {
        ch4_bonus_animated_clock_frames()
    }
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

    let filename = "ch4_clock_face";
    save(canvas, filename);
}

/// Just a little extra fun, generate frames with clock "hands"
fn ch4_bonus_animated_clock_frames() {
    fn render_clock(minute: i32) {
        let radius = 40.0;
        let n_points = 12;

        let margin = 5.0;
        let canvas_dim = (radius + margin) * 2_f64;
        let mut canvas = &mut Canvas::new(canvas_dim as usize, canvas_dim as usize);

        // Hours
        for rmod in 0..3 {
            canvas = render_clock_face(canvas, radius - (rmod as f64), n_points, canvas_dim);
            canvas = render_clock_face(canvas, radius + (rmod as f64), n_points, canvas_dim);
        }
        // Minutes
        canvas = render_clock_face(canvas, radius, n_points * 5, canvas_dim);

        // hands
        canvas = render_clock_hands(canvas, radius, minute, canvas_dim);

        let this_frame_filename = format!("clockframes/clock_{:05}", minute);
        save(canvas, this_frame_filename.as_str());
    }

    let max = (60 * 12);
    for minute in 0..max {
        render_clock(minute);
    }
}

fn render_clock_face(
    mut canvas: &mut Canvas,
    radius: f64,
    n_points: i32,
    canvas_dim: f64,
) -> &mut Canvas {
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

fn render_clock_hands(
    mut canvas: &mut Canvas,
    radius: f64,
    minutes: i32,
    canvas_dim: f64,
) -> &mut Canvas {
    let minute_hand_angle = (2_f64 * PI) * ((minutes % 60) as f64) / 60.0;
    let minute_hand_length = radius;
    let (length, angle_radians) = (minute_hand_length as usize, minute_hand_angle);
    for i in 0..length {
        let p = Tuple::point(0., i as f64, 0.);
        let t = Matrix::transformation().rotate_z(angle_radians).translate(
            canvas_dim / 2.0,
            canvas_dim / 2.0,
            0.0,
        );
        let position = t * p;
        let color = Color::rgb(1.0, 1.0, 1.0);
        canvas = draw_dot(canvas, position.x as i32, position.y as i32, color);
    }

    // TODO reduce duplication by putting 'draw hand' in a function. was having issues with 'lifetime'
    let hour_hand_angle = (2.0 * PI) * ((minutes as f64) / 60_f64) / 12.0;
    let hour_hand_length = radius * 2. / 3.;
    let (length, angle_radians) = (hour_hand_length as usize, hour_hand_angle);
    for i in 0..length {
        let p = Tuple::point(0., i as f64, 0.);
        let t = Matrix::transformation().rotate_z(angle_radians).translate(
            canvas_dim / 2.0,
            canvas_dim / 2.0,
            0.0,
        );
        let position = t * p;
        let color = Color::rgb(1.0, 1.0, 1.0);
        canvas = draw_dot(canvas, position.x as i32, position.y as i32, color);
    }

    canvas
}


fn draw_dot(canvas: &mut Canvas, x: i32, y: i32, color: Color) -> &mut Canvas {
    let size = 0;

    for px in ((x - size) as usize)..((x + size + 1) as usize) {
        for py in ((y - size) as usize)..((y + size + 1) as usize) {
            canvas.write_pixel(px, py, color);
        }
    }
    canvas
}
