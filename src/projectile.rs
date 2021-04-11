use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::Tuple;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

impl Projectile {
    pub fn update(&self, env: &Environment, dtime: f64) -> Self {
        let position = self.position + (self.velocity * dtime);
        let accel = (env.gravity + env.wind) * dtime;
        let velocity = self.velocity + accel;

        Projectile { position, velocity }
    }

    pub fn coords(&self) -> (i64, i64) {
        (
            self.position.x.round() as i64,
            self.position.y.round() as i64,
        )
    }

    pub fn is_out_of_bounds(&self, canvas: &Canvas) -> bool {
        let (x, y) = self.coords();

        x < 0 || y < 0 || x.abs() as usize > canvas.width || y.abs() as usize > canvas.height
    }

    pub(crate) fn draw_on(&self, canvas: &mut Canvas) {
        let (x, y) = self.coords();
        let y = canvas.height - (y as usize);
        canvas.write_pixel(x as usize, y, Color::RED);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let position = Tuple::origin();
        let velocity = Tuple::vector(1., 1., 0.);
        let proj = Projectile { position, velocity };

        let gravity = Tuple::vector(0., -0.5, 0.);
        let wind = Tuple::vector(-0.1, 0., 0.);
        let env = Environment { gravity, wind };

        let next = proj.update(&env, 1.0);
        assert_eq!(next.position, Tuple::point(1., 1., 0.));
        assert_eq!(next.velocity, Tuple::vector(0.9, 0.5, 0.));

        let next = proj.update(&env, 2.0);
        assert_eq!(next.position, Tuple::point(2., 2., 0.));
        assert_eq!(next.velocity, Tuple::vector(0.8, 0.0, 0.));
    }
}
