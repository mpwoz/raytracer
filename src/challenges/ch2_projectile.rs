use crate::canvas::Canvas;
use crate::challenges::{outfile, save};
use crate::projectile::{Environment, Projectile};
use crate::tuple::Tuple;

/// Chapter 2 drawing a projectile with wind and gravity
pub fn ch2_projectile() {
    let mut c = Canvas::new(900, 550);

    // environment
    let gravity = Tuple::vector(0., -0.1, 0.);
    let wind = Tuple::vector(-0.01, 0., 0.);
    let e = Environment { gravity, wind };

    // projectile
    let start = Tuple::point(0., 1., 0.);
    let velocity = Tuple::vector(1., 1.8, 0.);
    let velocity = velocity.normalized() * 11.25;
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

    let filename = "ch2_projectile_trajectory";
    save(&c, filename);
}
